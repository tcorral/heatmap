import AppKit
import Foundation
import HeatmapAppleMacOS
import HeatmapAppleShared

@MainActor
final class HeatmapSession: ObservableObject {
    let client: HeatmapClient
    private let captureBackend: HeatmapMacosRustCaptureBackend

    init(client: HeatmapClient, captureBackend: HeatmapMacosRustCaptureBackend) {
        self.client = client
        self.captureBackend = captureBackend
    }

    func trackScreen(_ screenId: String, layoutVersion: String? = nil) {
        do {
            try client.screen(screenId, layoutVersion: layoutVersion)
            recordViewportLifecycle(screenView: true, scroll: false)
        } catch {
            #if DEBUG
            print("[heatmap] screen(\(screenId)) failed: \(error)")
            #endif
        }
    }

    func recordViewportScroll() {
        recordViewportLifecycle(screenView: false, scroll: true)
    }

    func recordIdentifiedActivation(elementId: String) {
        guard HeatmapE2ECaptureHooks.usesWindowTapCapture else {
            return
        }
        guard let (width, height) = Self.keyWindowViewportSize() else {
            return
        }
        captureBackend.recordIdentifiedActivationClick(
            elementId: elementId,
            normalizedX: 0.5,
            normalizedY: 0.5,
            viewportWidth: width,
            viewportHeight: height
        )
        flushBatch()
    }

    func flushBatch() {
        do {
            try client.flush()
        } catch {
            #if DEBUG
            print("[heatmap] flush failed: \(error)")
            #endif
        }
    }

    private func recordViewportLifecycle(screenView: Bool, scroll: Bool) {
        guard let (width, height) = Self.keyWindowViewportSize() else {
            return
        }
        if screenView {
            captureBackend.recordScreenView(viewportWidth: width, viewportHeight: height)
        }
        if scroll {
            captureBackend.recordScroll(viewportWidth: width, viewportHeight: height)
        }
    }

    private static func keyWindowViewportSize() -> (UInt32, UInt32)? {
        let window = NSApp.keyWindow ?? NSApp.mainWindow ?? NSApp.windows.first
        if let window {
            let bounds = window.contentView?.bounds ?? window.contentLayoutRect
            let width = UInt32(max(bounds.width, 0).rounded(.towardZero))
            let height = UInt32(max(bounds.height, 0).rounded(.towardZero))
            if width > 0, height > 0 {
                return (width, height)
            }
        }
        if HeatmapE2ECaptureHooks.usesWindowTapCapture {
            return (900, 700)
        }
        return nil
    }
}

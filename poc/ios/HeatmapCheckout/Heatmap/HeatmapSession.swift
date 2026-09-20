import Foundation
import HeatmapAppleIOS
import HeatmapAppleShared
import UIKit

@MainActor
final class HeatmapSession: ObservableObject {
    let client: HeatmapClient
    private let captureBackend: HeatmapIosRustCaptureBackend

    init(client: HeatmapClient, captureBackend: HeatmapIosRustCaptureBackend) {
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
        let window = UIApplication.shared.connectedScenes
            .compactMap { $0 as? UIWindowScene }
            .flatMap(\.windows)
            .first(where: \.isKeyWindow)
        guard let bounds = window?.bounds else {
            return nil
        }
        let width = UInt32(max(bounds.width, 0).rounded(.towardZero))
        let height = UInt32(max(bounds.height, 0).rounded(.towardZero))
        guard width > 0, height > 0 else {
            return nil
        }
        return (width, height)
    }
}

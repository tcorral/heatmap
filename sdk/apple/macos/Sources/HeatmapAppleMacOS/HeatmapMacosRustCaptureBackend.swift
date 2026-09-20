import AppKit
import Foundation
import HeatmapAppleMacOSCapture
import HeatmapFFI

/// Forwards adapter payloads to UniFFI `HeatmapSdk` (not exposed on `HeatmapClient`).
public final class HeatmapMacosRustCaptureBackend: HeatmapMacosCaptureBackend {
    private let sdk: HeatmapSdk
    private let osVersion: String
    private let appVersion: String
    private let runOffUi: (@escaping () -> Void) -> Void

    public init(
        sdk: HeatmapSdk,
        osVersion: String,
        appVersion: String,
        runOffUi: @escaping (@escaping () -> Void) -> Void = { $0() }
    ) {
        self.sdk = sdk
        self.osVersion = osVersion
        self.appVersion = appVersion
        self.runOffUi = runOffUi
    }

    public convenience init(sdk: HeatmapSdk, runOffUi: @escaping (@escaping () -> Void) -> Void = { $0() }) {
        let osVersion = ProcessInfo.processInfo.operatingSystemVersionString
        let appVersion = Bundle.main.infoDictionary?["CFBundleShortVersionString"] as? String ?? "0.0.0"
        self.init(sdk: sdk, osVersion: osVersion, appVersion: appVersion, runOffUi: runOffUi)
    }

    public func recordActivationClick(_ payload: MacosActivationClickPayload) {
        runOffUi { [sdk, osVersion, appVersion] in
            try? sdk.recordMacosActivationClick(
                x: payload.position.x,
                y: payload.position.y,
                elementId: payload.elementId,
                viewportWidth: payload.viewportWidth,
                viewportHeight: payload.viewportHeight,
                timestampMs: payload.timestampMs,
                windowId: payload.windowId,
                osVersion: osVersion,
                appVersion: appVersion
            )
        }
    }

    public func recordScreenView(viewportWidth: UInt32, viewportHeight: UInt32, windowId: String) {
        runOffUi { [sdk, osVersion, appVersion] in
            try? sdk.recordMacosScreenView(
                viewportWidth: viewportWidth,
                viewportHeight: viewportHeight,
                windowId: windowId,
                osVersion: osVersion,
                appVersion: appVersion
            )
        }
    }

    public func recordScroll(viewportWidth: UInt32, viewportHeight: UInt32, windowId: String) {
        runOffUi { [sdk, osVersion, appVersion] in
            try? sdk.recordMacosScroll(
                viewportWidth: viewportWidth,
                viewportHeight: viewportHeight,
                windowId: windowId,
                osVersion: osVersion,
                appVersion: appVersion
            )
        }
    }

    public func recordIdentifiedActivationClick(
        elementId: String,
        normalizedX: Double,
        normalizedY: Double,
        viewportWidth: UInt32,
        viewportHeight: UInt32,
        timestampMs: Int64 = Int64(Date().timeIntervalSince1970 * 1000)
    ) {
        let window = NSApp.keyWindow ?? NSApp.mainWindow
        let windowId = window.map { MacosWindowIdentity.opaqueId(for: $0) } ?? "e2e"
        runOffUi { [sdk, osVersion, appVersion] in
            try? sdk.recordMacosActivationClick(
                x: normalizedX,
                y: normalizedY,
                elementId: elementId,
                viewportWidth: viewportWidth,
                viewportHeight: viewportHeight,
                timestampMs: timestampMs,
                windowId: windowId,
                osVersion: osVersion,
                appVersion: appVersion
            )
        }
    }

    public func onBackground() {
        runOffUi { [sdk] in
            sdk.onBackground()
        }
    }

    public func onForeground() {
        runOffUi { [sdk] in
            sdk.onForeground()
        }
    }
}

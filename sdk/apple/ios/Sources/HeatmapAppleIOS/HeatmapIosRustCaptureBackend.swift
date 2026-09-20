import Foundation
import HeatmapAppleIOSCapture
import HeatmapFFI

/// Forwards adapter payloads to UniFFI `HeatmapSdk` (not exposed on `HeatmapClient`).
public final class HeatmapIosRustCaptureBackend: HeatmapIosCaptureBackend {
    private let sdk: HeatmapSdk
    private let osVersion: String
    private let appVersion: String

    public init(sdk: HeatmapSdk, osVersion: String, appVersion: String) {
        self.sdk = sdk
        self.osVersion = osVersion
        self.appVersion = appVersion
    }

    public convenience init(sdk: HeatmapSdk) {
        let osVersion = ProcessInfo.processInfo.operatingSystemVersionString
        let appVersion = Bundle.main.infoDictionary?["CFBundleShortVersionString"] as? String ?? "0.0.0"
        self.init(sdk: sdk, osVersion: osVersion, appVersion: appVersion)
    }

    public func recordActivationClick(_ payload: IosActivationClickPayload) {
        try? sdk.recordActivationClick(
            x: payload.position.x,
            y: payload.position.y,
            elementId: payload.elementId,
            viewportWidth: payload.viewportWidth,
            viewportHeight: payload.viewportHeight,
            timestampMs: payload.timestampMs,
            osVersion: osVersion,
            appVersion: appVersion
        )
    }

    public func recordScreenView(viewportWidth: UInt32, viewportHeight: UInt32) {
        try? sdk.recordIosScreenView(
            viewportWidth: viewportWidth,
            viewportHeight: viewportHeight,
            osVersion: osVersion,
            appVersion: appVersion
        )
    }

    public func recordScroll(viewportWidth: UInt32, viewportHeight: UInt32) {
        try? sdk.recordIosScroll(
            viewportWidth: viewportWidth,
            viewportHeight: viewportHeight,
            osVersion: osVersion,
            appVersion: appVersion
        )
    }

    public func onBackground() {
        sdk.onBackground()
    }

    public func onForeground() {
        sdk.onForeground()
    }
}

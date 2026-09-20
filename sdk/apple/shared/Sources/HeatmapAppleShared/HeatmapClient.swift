import Foundation
import HeatmapFFI

/// App-facing Heatmap SDK client. Wraps UniFFI `HeatmapSdk` with a stable Swift API.
public final class HeatmapClient: @unchecked Sendable {
    private let sdk: HeatmapSdk
    private let flushScheduler: HeatmapFlushScheduler

    /// Initialize from JSON matching the core `HeatmapConfig` shape.
    public init(configJson: String) throws {
        sdk = try HeatmapSdk.initFromJson(configJson: configJson)
        flushScheduler = HeatmapClient.makeFlushScheduler(sdk: sdk)
    }

    /// Initialize with required fields and core defaults for optional settings.
    public init(projectKey: String, endpoint: String) throws {
        let config = InitConfig(
            projectKey: projectKey,
            endpoint: endpoint,
            sessionTimeoutMs: 1_800_000,
            sampleRate: 1.0,
            excludeElements: [],
            excludeScreens: [],
            maskElements: [],
            storageDirectory: HeatmapClient.defaultStoragePath,
            appVersion: "1.0.0"
        )
        sdk = try HeatmapSdk.`init`(config: config)
        flushScheduler = HeatmapClient.makeFlushScheduler(sdk: sdk)
    }

    /// Persistent identity and queue directory for this install.
    public static var defaultStoragePath: String {
        let base = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask).first
            ?? FileManager.default.temporaryDirectory
        let dir = base.appendingPathComponent("heatmap-sdk", isDirectory: true)
        try? FileManager.default.createDirectory(at: dir, withIntermediateDirectories: true)
        return dir.path
    }

    /// Use the same `HeatmapSdk` instance as `HeatmapIosRustCaptureBackend`.
    public init(sdk: HeatmapSdk) {
        self.sdk = sdk
        flushScheduler = HeatmapClient.makeFlushScheduler(sdk: sdk)
    }

    deinit {
        flushScheduler.stop()
    }

    private static func makeFlushScheduler(sdk: HeatmapSdk) -> HeatmapFlushScheduler {
        HeatmapFlushScheduler(pollInterval: 10) { [weak sdk] in
            try? sdk?.pollFlush()
        }
    }

    public func identify(userId: String) throws {
        try sdk.identify(userId: userId)
    }

    public func signOut() {
        sdk.signOut()
    }

    public func screen(_ screenId: String, layoutVersion: String? = nil) throws {
        try sdk.screen(screenId: screenId, layoutVersion: layoutVersion)
    }

    public func setTrackingEnabled(_ enabled: Bool) {
        sdk.setTrackingEnabled(enabled: enabled)
    }

    public func flush() throws {
        try flushScheduler.perform {
            try self.sdk.flush()
        }
    }
}

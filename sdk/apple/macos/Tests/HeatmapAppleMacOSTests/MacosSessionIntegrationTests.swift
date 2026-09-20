import XCTest
@testable import HeatmapAppleMacOS
@testable import HeatmapAppleMacOSCapture

#if canImport(HeatmapFFI)
import HeatmapFFI

final class MacosSessionIntegrationTests: XCTestCase {
    func testBackgroundThenForegroundKeepsSessionBeforeTimeout() throws {
        let config = InitConfig(
            projectKey: "pk_test",
            endpoint: "https://ingest.example.com",
            sessionTimeoutMs: 10_000,
            sampleRate: 1.0,
            excludeElements: [],
            excludeScreens: [],
            maskElements: [],
            storageDirectory: FileManager.default.temporaryDirectory
                .appendingPathComponent(UUID().uuidString, isDirectory: true)
                .path,
            appVersion: "1.0.0"
        )
        let sdk = try HeatmapSdk.`init`(config: config)
        let backend = HeatmapMacosRustCaptureBackend(
            sdk: sdk,
            osVersion: "macOS 14.6",
            appVersion: "1.0.0"
        )

        backend.recordActivationClick(
            MacosActivationClickPayload(
                elementId: "checkout.pay",
                windowId: "win_checkout_primary",
                position: NormalizedWindowPosition(x: 0.5, y: 0.5),
                viewportWidth: 800,
                viewportHeight: 600,
                timestampMs: 100
            )
        )
        let firstSessionIds = try sdk.adapterClickSessionIdsFromBatch()
        XCTAssertEqual(firstSessionIds.count, 1)

        backend.onBackground()
        backend.onForeground()
        backend.recordActivationClick(
            MacosActivationClickPayload(
                elementId: nil,
                windowId: "win_checkout_primary",
                position: NormalizedWindowPosition(x: 0.25, y: 0.5),
                viewportWidth: 800,
                viewportHeight: 600,
                timestampMs: 200
            )
        )
        let secondSessionIds = try sdk.adapterClickSessionIdsFromBatch()
        XCTAssertEqual(secondSessionIds.count, 1)
        XCTAssertEqual(secondSessionIds[0], firstSessionIds[0])
    }
}
#else
final class MacosSessionIntegrationTests: XCTestCase {
    func testBindingsMissing() throws {
        throw XCTSkip("HeatmapFFI bindings not available in this build")
    }
}
#endif

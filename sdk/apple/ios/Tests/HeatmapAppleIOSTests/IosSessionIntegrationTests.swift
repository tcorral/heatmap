import XCTest
@testable import HeatmapAppleIOS
@testable import HeatmapAppleIOSCapture

#if canImport(HeatmapFFI)
import HeatmapFFI

final class IosSessionIntegrationTests: XCTestCase {
    func testBackgroundThenForegroundKeepsSessionBeforeTimeout() throws {
        let config = InitConfig(
            projectKey: "pk_test",
            endpoint: "https://ingest.example.com",
            sessionTimeoutMs: 10_000,
            sampleRate: 1.0,
            excludeElements: [],
            excludeScreens: [],
            maskElements: []
        )
        let sdk = try HeatmapSdk.`init`(config: config)
        let backend = HeatmapIosRustCaptureBackend(
            sdk: sdk,
            osVersion: "iOS 18.0",
            appVersion: "1.0.0"
        )

        backend.recordActivationClick(
            IosActivationClickPayload(
                elementId: "checkout.pay",
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
            IosActivationClickPayload(
                elementId: nil,
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
final class IosSessionIntegrationTests: XCTestCase {
    func testBindingsMissing() throws {
        throw XCTSkip("HeatmapFFI bindings not available in this build")
    }
}
#endif

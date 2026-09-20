import XCTest
@testable import HeatmapAppleIOSCapture

final class IosLifecycleTests: XCTestCase {
    func testLifecycleWrapperForwardsBackgroundAndForeground() {
        let backend = RecordingIosCaptureBackend()
        let lifecycle = HeatmapIosLifecycle(backend: backend)

        lifecycle.onBackground()
        lifecycle.onForeground()

        XCTAssertEqual(backend.backgroundCount, 1)
        XCTAssertEqual(backend.foregroundCount, 1)
    }
}

final class RecordingIosCaptureBackend: HeatmapIosCaptureBackend {
    private(set) var backgroundCount = 0
    private(set) var foregroundCount = 0
    private(set) var recordedClicks: [IosActivationClickPayload] = []

    func recordActivationClick(_ payload: IosActivationClickPayload) {
        recordedClicks.append(payload)
    }

    func recordScreenView(viewportWidth: UInt32, viewportHeight: UInt32) {}

    func recordScroll(viewportWidth: UInt32, viewportHeight: UInt32) {}

    func onBackground() {
        backgroundCount += 1
    }

    func onForeground() {
        foregroundCount += 1
    }
}

import XCTest
@testable import HeatmapAppleIOSCapture

final class IosActivationPayloadTests: XCTestCase {
    func testPayMarkedClickPayloadIncludesElementId() throws {
        let pay = MockCaptureView(accessibilityIdentifier: "checkout.pay")
        let built = IosActivationPayloadBuilder.buildClick(
            hitView: pay,
            locationInWindow: CGPoint(x: 120, y: 80),
            windowBounds: CGRect(x: 0, y: 0, width: 800, height: 600),
            timestampMs: 1_726_771_205_000
        )

        let payload = try XCTUnwrap(built)
        XCTAssertEqual(payload.elementId, "checkout.pay")
        XCTAssertEqual(payload.position.x, 0.15, accuracy: 0.0001)
        XCTAssertEqual(payload.position.y, 80.0 / 600.0, accuracy: 0.0001)
        XCTAssertEqual(payload.viewportWidth, 800)
        XCTAssertEqual(payload.viewportHeight, 600)
    }

    func testUnmarkedNonSecretClickPayloadHasCoordinatesOnly() throws {
        let cancel = MockCaptureView(accessibilityIdentifier: nil)
        let built = IosActivationPayloadBuilder.buildClick(
            hitView: cancel,
            locationInWindow: CGPoint(x: 200, y: 300),
            windowBounds: CGRect(x: 0, y: 0, width: 800, height: 600),
            timestampMs: 1
        )

        let payload = try XCTUnwrap(built)
        XCTAssertNil(payload.elementId)
        XCTAssertEqual(payload.position.x, 0.25, accuracy: 0.0001)
        XCTAssertEqual(payload.position.y, 0.5, accuracy: 0.0001)
    }

    func testSecretFieldClickProducesNoPayload() {
        let password = MockTextInput(
            accessibilityIdentifier: nil,
            isSecureTextEntry: true,
            textContentType: nil
        )
        let payload = IosActivationPayloadBuilder.buildClick(
            hitView: password,
            locationInWindow: CGPoint(x: 40, y: 40),
            windowBounds: CGRect(x: 0, y: 0, width: 800, height: 600),
            timestampMs: 1
        )
        XCTAssertNil(payload)
    }

    func testBuilderDoesNotReadForbiddenTextSurface() {
        let pay = MockCaptureView(
            accessibilityIdentifier: "checkout.pay",
            onForbiddenRead: { key in
                XCTFail("forbidden read: \(key)")
            }
        )
        _ = IosActivationPayloadBuilder.buildClick(
            hitView: pay,
            locationInWindow: CGPoint(x: 10, y: 10),
            windowBounds: CGRect(x: 0, y: 0, width: 100, height: 100),
            timestampMs: 1
        )
    }
}

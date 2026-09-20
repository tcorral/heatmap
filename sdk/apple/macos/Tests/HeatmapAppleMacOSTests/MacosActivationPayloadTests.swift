import XCTest
@testable import HeatmapAppleMacOSCapture

final class MacosActivationPayloadTests: XCTestCase {
    func testPayMarkedClickPayloadIncludesElementIdAndWindowId() throws {
        let pay = MockCaptureView(accessibilityIdentifier: "checkout.pay")
        let built = MacosActivationPayloadBuilder.buildClick(
            hitView: pay,
            locationInWindow: CGPoint(x: 120, y: 520),
            windowBounds: CGRect(x: 0, y: 0, width: 800, height: 600),
            windowId: "win_checkout_primary",
            timestampMs: 1_726_771_205_000
        )

        let payload = try XCTUnwrap(built)
        XCTAssertEqual(payload.elementId, "checkout.pay")
        XCTAssertEqual(payload.windowId, "win_checkout_primary")
        XCTAssertEqual(payload.position.x, 0.15, accuracy: 0.0001)
        XCTAssertEqual(payload.position.y, 80.0 / 600.0, accuracy: 0.0001)
        XCTAssertEqual(payload.viewportWidth, 800)
        XCTAssertEqual(payload.viewportHeight, 600)
    }

    func testUnmarkedNonSecretClickPayloadHasCoordinatesOnly() throws {
        let cancel = MockCaptureView(accessibilityIdentifier: nil)
        let built = MacosActivationPayloadBuilder.buildClick(
            hitView: cancel,
            locationInWindow: CGPoint(x: 200, y: 300),
            windowBounds: CGRect(x: 0, y: 0, width: 800, height: 600),
            windowId: "win_checkout_primary",
            timestampMs: 1
        )

        let payload = try XCTUnwrap(built)
        XCTAssertNil(payload.elementId)
        XCTAssertEqual(payload.windowId, "win_checkout_primary")
        XCTAssertEqual(payload.position.x, 0.25, accuracy: 0.0001)
        XCTAssertEqual(payload.position.y, 0.5, accuracy: 0.0001)
    }

    func testSecretFieldClickProducesNoPayload() {
        let password = MockSecureTextInput(
            accessibilityIdentifier: nil,
            isSecureTextField: true
        )
        let payload = MacosActivationPayloadBuilder.buildClick(
            hitView: password,
            locationInWindow: CGPoint(x: 40, y: 40),
            windowBounds: CGRect(x: 0, y: 0, width: 800, height: 600),
            windowId: "win_checkout_primary",
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
        _ = MacosActivationPayloadBuilder.buildClick(
            hitView: pay,
            locationInWindow: CGPoint(x: 10, y: 590),
            windowBounds: CGRect(x: 0, y: 0, width: 100, height: 100),
            windowId: "win_a",
            timestampMs: 1
        )
    }

    func testResizeAffectsOnlyNewEventNormalization() throws {
        let view = MockCaptureView(accessibilityIdentifier: nil)
        let before = MacosActivationPayloadBuilder.buildClick(
            hitView: view,
            locationInWindow: CGPoint(x: 400, y: 300),
            windowBounds: CGRect(x: 0, y: 0, width: 800, height: 600),
            windowId: "win_a",
            timestampMs: 1
        )
        let after = MacosActivationPayloadBuilder.buildClick(
            hitView: view,
            locationInWindow: CGPoint(x: 400, y: 150),
            windowBounds: CGRect(x: 0, y: 0, width: 400, height: 300),
            windowId: "win_a",
            timestampMs: 2
        )
        let first = try XCTUnwrap(before)
        let second = try XCTUnwrap(after)
        XCTAssertEqual(first.position.x, 0.5, accuracy: 0.0001)
        XCTAssertEqual(first.position.y, 0.5, accuracy: 0.0001)
        XCTAssertEqual(second.position.x, 1.0, accuracy: 0.0001)
        XCTAssertEqual(second.position.y, 0.5, accuracy: 0.0001)
        XCTAssertEqual(first.viewportWidth, 800)
        XCTAssertEqual(second.viewportWidth, 400)
    }

    func testDistinctWindowsCarryDistinctWindowIds() throws {
        let view = MockCaptureView(accessibilityIdentifier: "checkout.pay")
        let primary = try XCTUnwrap(
            MacosActivationPayloadBuilder.buildClick(
                hitView: view,
                locationInWindow: CGPoint(x: 10, y: 590),
                windowBounds: CGRect(x: 0, y: 0, width: 100, height: 100),
                windowId: "win_primary",
                timestampMs: 1
            )
        )
        let secondary = try XCTUnwrap(
            MacosActivationPayloadBuilder.buildClick(
                hitView: view,
                locationInWindow: CGPoint(x: 10, y: 590),
                windowBounds: CGRect(x: 0, y: 0, width: 100, height: 100),
                windowId: "win_secondary",
                timestampMs: 2
            )
        )
        XCTAssertEqual(primary.windowId, "win_primary")
        XCTAssertEqual(secondary.windowId, "win_secondary")
    }
}

import XCTest
@testable import HeatmapAppleMacOSCapture

final class MacosTapIdentityTests: XCTestCase {
    func testPayAccessibilityIdentifierAttributedAsCheckoutPay() {
        let pay = MockCaptureView(accessibilityIdentifier: "checkout.pay")
        let id = MacosTapIdentity.elementId(for: pay)
        XCTAssertEqual(id, "checkout.pay")
    }

    func testPayUsesViewIdentifierWhenAccessibilityIdentifierEmpty() {
        let pay = MockCaptureView(
            accessibilityIdentifier: nil,
            identifierRawValue: "checkout.pay"
        )
        let id = MacosTapIdentity.elementId(for: pay)
        XCTAssertEqual(id, "checkout.pay")
    }

    func testAccessibilityIdentifierTakesPrecedenceOverViewIdentifier() {
        let pay = MockCaptureView(
            accessibilityIdentifier: "checkout.pay.a11y",
            identifierRawValue: "checkout.pay"
        )
        let id = MacosTapIdentity.elementId(for: pay)
        XCTAssertEqual(id, "checkout.pay.a11y")
    }

    func testUnmarkedControlHasNoElementId() {
        let cancel = MockCaptureView(accessibilityIdentifier: nil, identifierRawValue: nil)
        let id = MacosTapIdentity.elementId(for: cancel)
        XCTAssertNil(id)
    }

    func testElementIdWalksUpSuperviewChain() {
        let pay = MockCaptureView(accessibilityIdentifier: nil, identifierRawValue: "checkout.pay")
        let inner = MockCaptureView(accessibilityIdentifier: nil, superview: pay)
        let id = MacosTapIdentity.elementId(for: inner)
        XCTAssertEqual(id, "checkout.pay")
    }

    func testBlankAccessibilityIdentifierFallsThroughToViewIdentifier() {
        let pay = MockCaptureView(
            accessibilityIdentifier: "   ",
            identifierRawValue: "checkout.pay"
        )
        let id = MacosTapIdentity.elementId(for: pay)
        XCTAssertEqual(id, "checkout.pay")
    }
}

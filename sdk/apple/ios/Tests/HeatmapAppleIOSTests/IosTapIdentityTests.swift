import XCTest
@testable import HeatmapAppleIOSCapture

final class IosTapIdentityTests: XCTestCase {
    func testPayAccessibilityIdentifierAttributedAsCheckoutPay() {
        let pay = MockCaptureView(accessibilityIdentifier: "checkout.pay")
        let id = IosTapIdentity.elementId(for: pay)
        XCTAssertEqual(id, "checkout.pay")
    }

    func testUnmarkedControlHasNoElementId() {
        let cancel = MockCaptureView(accessibilityIdentifier: nil)
        let id = IosTapIdentity.elementId(for: cancel)
        XCTAssertNil(id)
    }

    func testElementIdWalksUpSuperviewChain() {
        let pay = MockCaptureView(accessibilityIdentifier: "checkout.pay")
        let inner = MockCaptureView(accessibilityIdentifier: nil, superview: pay)
        let id = IosTapIdentity.elementId(for: inner)
        XCTAssertEqual(id, "checkout.pay")
    }

    func testBlankAccessibilityIdentifierIsIgnored() {
        let view = MockCaptureView(accessibilityIdentifier: "   ")
        let id = IosTapIdentity.elementId(for: view)
        XCTAssertNil(id)
    }
}

import XCTest

final class CheckoutPayIdentifierTests: XCTestCase {
    func testPayButtonAccessibilityIdentifierMatchesSharedContract() {
        XCTAssertEqual(CheckoutIdentifiers.payButton, "checkout.pay")
    }
}

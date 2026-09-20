import XCTest

enum CheckoutJourneySteps {
    @MainActor
    static func runSharedCheckout(in app: XCUIApplication, includeBrowseProducts: Bool = true) {
        if includeBrowseProducts {
            app.buttons["Browse products"].firstMatch.tap()
        }
        tapActivateProduct(in: app)
        completeCheckoutFromProduct(in: app)
    }

    @MainActor
    static func tapActivateProduct(in app: XCUIApplication) {
        let activate = app.buttons["Activate product"].firstMatch
        XCTAssertTrue(activate.waitForExistence(timeout: 10))
        activate.tap()
    }

    @MainActor
    static func completeCheckoutFromProduct(in app: XCUIApplication) {
        let checkoutHeading = app.staticTexts["Checkout"].firstMatch
        XCTAssertTrue(checkoutHeading.waitForExistence(timeout: 10))

        scrollCheckoutUntilPayVisible(in: app)

        app.buttons["checkout.coupon"].firstMatch.tap()

        let pay = app.buttons["checkout.pay"].firstMatch
        XCTAssertTrue(pay.waitForExistence(timeout: 10))
        pay.tap()

        let confirmation = app.staticTexts["Confirmation"].firstMatch
        XCTAssertTrue(confirmation.waitForExistence(timeout: 15))
    }

    @MainActor
    private static func scrollCheckoutUntilPayVisible(in app: XCUIApplication) {
        let pay = app.buttons["checkout.pay"].firstMatch
        for _ in 0..<8 where !pay.isHittable {
            app.swipeUp()
        }
        XCTAssertTrue(pay.waitForExistence(timeout: 10))
    }
}

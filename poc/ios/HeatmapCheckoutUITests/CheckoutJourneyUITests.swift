import XCTest

final class CheckoutJourneyUITests: XCTestCase {
    private let eventClient = HeatmapE2EEventClient()

    override func setUpWithError() throws {
        continueAfterFailure = false
    }

    @MainActor
    func testCheckoutJourneyIngestsSharedSequenceWithNormalizedCoordinates() async throws {
        let app = XCUIApplication()
        app.launchEnvironment["HEATMAP_E2E_WINDOW_CAPTURE"] = "1"
        app.launch()

        CheckoutJourneySteps.runSharedCheckout(in: app)

        let events = try await eventClient.waitForEvents(minimumCount: 6)
        HeatmapE2EEventAssertions.assertSharedCheckoutSequence(events)
        HeatmapE2EEventAssertions.assertNormalizedClickCoordinates(events)
        HeatmapE2EEventAssertions.assertSingleSession(events)
    }

    @MainActor
    func testBackgroundResumeKeepsSingleSession() async throws {
        let app = XCUIApplication()
        app.launchEnvironment["HEATMAP_E2E_WINDOW_CAPTURE"] = "1"
        app.launch()

        app.buttons["Browse products"].firstMatch.tap()
        CheckoutJourneySteps.tapActivateProduct(in: app)

        XCUIDevice.shared.press(.home)
        app.activate()

        CheckoutJourneySteps.completeCheckoutFromProduct(in: app)

        let events = try await eventClient.waitForEvents(minimumCount: 6)
        HeatmapE2EEventAssertions.assertSharedCheckoutSequence(events)
        HeatmapE2EEventAssertions.assertSingleSession(events)
    }

    @MainActor
    func testHostRemainsUsableWhenIngestUnreachable() {
        let app = XCUIApplication()
        app.launchEnvironment["HEATMAP_ENDPOINT"] = "http://127.0.0.1:1"
        app.launch()

        CheckoutJourneySteps.runSharedCheckout(in: app)

        XCTAssertTrue(app.staticTexts["Confirmation"].firstMatch.exists)
        XCTAssertTrue(app.buttons["Back to home"].firstMatch.exists)
    }
}

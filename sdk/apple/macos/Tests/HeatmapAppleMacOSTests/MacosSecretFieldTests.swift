import XCTest
@testable import HeatmapAppleMacOSCapture

final class MacosSecretFieldTests: XCTestCase {
    func testSecureTextFieldDroppedEvenWhenUnmarked() {
        let field = MockSecureTextInput(
            accessibilityIdentifier: nil,
            isSecureTextField: true
        )
        XCTAssertTrue(MacosSecretField.isSecretInteractionTarget(field))
    }

    func testChildInsideSecureFieldIsDropped() {
        let field = MockSecureTextInput(
            accessibilityIdentifier: nil,
            isSecureTextField: true
        )
        let icon = MockCaptureView(accessibilityIdentifier: nil, superview: field)
        XCTAssertTrue(MacosSecretField.isSecretInteractionTarget(icon))
    }

    func testMarkedNonSecretControlIsNotDropped() {
        let button = MockCaptureView(accessibilityIdentifier: "checkout.pay")
        XCTAssertFalse(MacosSecretField.isSecretInteractionTarget(button))
    }
}

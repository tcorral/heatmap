import XCTest
@testable import HeatmapAppleIOSCapture

final class IosSecretFieldTests: XCTestCase {
    func testSecureTextEntryDroppedEvenWhenUnmarked() {
        let field = MockTextInput(
            accessibilityIdentifier: nil,
            isSecureTextEntry: true,
            textContentType: nil
        )
        XCTAssertTrue(IosSecretField.isSecretInteractionTarget(field))
    }

    func testPasswordContentTypeDroppedWhenUnmarked() {
        let field = MockTextInput(
            accessibilityIdentifier: nil,
            isSecureTextEntry: false,
            textContentType: .password
        )
        XCTAssertTrue(IosSecretField.isSecretInteractionTarget(field))
    }

    func testPaymentContentTypesDroppedWhenUnmarked() {
        let types: [IosTextContentType] = [
            .creditCardNumber,
            .creditCardExpiration,
            .creditCardSecurityCode,
            .newPassword,
        ]
        for contentType in types {
            let field = MockTextInput(
                accessibilityIdentifier: nil,
                isSecureTextEntry: false,
                textContentType: contentType
            )
            XCTAssertTrue(
                IosSecretField.isSecretInteractionTarget(field),
                "expected secret for \(contentType)"
            )
        }
    }

    func testChildInsideSecureFieldIsDropped() {
        let field = MockTextInput(
            accessibilityIdentifier: nil,
            isSecureTextEntry: true,
            textContentType: nil
        )
        let icon = MockCaptureView(accessibilityIdentifier: nil, superview: field)
        XCTAssertTrue(IosSecretField.isSecretInteractionTarget(icon))
    }

    func testMarkedNonSecretControlIsNotDropped() {
        let button = MockCaptureView(accessibilityIdentifier: "checkout.pay")
        XCTAssertFalse(IosSecretField.isSecretInteractionTarget(button))
    }
}

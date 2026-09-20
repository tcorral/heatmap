import CoreGraphics
@testable import HeatmapAppleMacOSCapture

class MockCaptureView: MacosCaptureViewNode {
    let heatmapAccessibilityIdentifier: String?
    let heatmapViewIdentifierRawValue: String?
    private weak var parent: MockCaptureView?
    private let forbiddenRead: ((String) -> Void)?

    init(
        accessibilityIdentifier: String?,
        identifierRawValue: String? = nil,
        superview: MockCaptureView? = nil,
        onForbiddenRead: ((String) -> Void)? = nil
    ) {
        self.heatmapAccessibilityIdentifier = accessibilityIdentifier
        self.heatmapViewIdentifierRawValue = identifierRawValue
        self.parent = superview
        self.forbiddenRead = onForbiddenRead
    }

    var captureSuperview: MacosCaptureViewNode? { parent }

    func assertForbiddenRead(_ key: String) -> Never {
        forbiddenRead?(key)
        fatalError("forbidden read: \(key)")
    }
}

final class MockSecureTextInput: MockCaptureView, MacosTextInputCaptureNode {
    let isSecureTextField: Bool

    init(
        accessibilityIdentifier: String?,
        isSecureTextField: Bool,
        superview: MockCaptureView? = nil
    ) {
        self.isSecureTextField = isSecureTextField
        super.init(accessibilityIdentifier: accessibilityIdentifier, superview: superview)
    }
}

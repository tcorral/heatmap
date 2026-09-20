import CoreGraphics
@testable import HeatmapAppleIOSCapture

class MockCaptureView: IosCaptureViewNode {
    let accessibilityIdentifier: String?
    private weak var parent: MockCaptureView?
    private let forbiddenRead: ((String) -> Void)?

    init(
        accessibilityIdentifier: String?,
        superview: MockCaptureView? = nil,
        onForbiddenRead: ((String) -> Void)? = nil
    ) {
        self.accessibilityIdentifier = accessibilityIdentifier
        self.parent = superview
        self.forbiddenRead = onForbiddenRead
    }

    var captureSuperview: IosCaptureViewNode? { parent }

    func assertForbiddenRead(_ key: String) -> Never {
        forbiddenRead?(key)
        fatalError("forbidden read: \(key)")
    }
}

final class MockTextInput: MockCaptureView, IosTextInputCaptureNode {
    let isSecureTextEntry: Bool
    let textContentType: IosTextContentType?

    init(
        accessibilityIdentifier: String?,
        isSecureTextEntry: Bool,
        textContentType: IosTextContentType?,
        superview: MockCaptureView? = nil
    ) {
        self.isSecureTextEntry = isSecureTextEntry
        self.textContentType = textContentType
        super.init(accessibilityIdentifier: accessibilityIdentifier, superview: superview)
    }
}

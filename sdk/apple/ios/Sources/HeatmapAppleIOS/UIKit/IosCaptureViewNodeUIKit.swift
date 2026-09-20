#if os(iOS)
import HeatmapAppleIOSCapture
import UIKit

extension UIView: IosCaptureViewNode {
    public var captureSuperview: IosCaptureViewNode? {
        superview
    }
}

extension UITextField: IosTextInputCaptureNode {
    public var textContentType: IosTextContentType? {
        IosTextContentTypeMapping.from(uiTextContentType)
    }
}

extension UITextView: IosTextInputCaptureNode {
    public var isSecureTextEntry: Bool { false }

    public var textContentType: IosTextContentType? {
        IosTextContentTypeMapping.from(uiTextContentType)
    }
}

enum IosTextContentTypeMapping {
    static func from(_ type: UITextContentType?) -> IosTextContentType? {
        guard let type else { return nil }
        switch type {
        case .password:
            return .password
        case .newPassword:
            return .newPassword
        case .creditCardNumber:
            return .creditCardNumber
        case .creditCardExpiration:
            return .creditCardExpiration
        case .creditCardSecurityCode:
            return .creditCardSecurityCode
        default:
            return .other
        }
    }
}
#endif

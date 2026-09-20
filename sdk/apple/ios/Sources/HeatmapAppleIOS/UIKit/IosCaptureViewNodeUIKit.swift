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
        let uiType = value(forKey: "textContentType") as? UITextContentType
        return IosTextContentTypeMapping.from(uiType)
    }
}

extension UITextView: IosTextInputCaptureNode {
    public var isSecureTextEntry: Bool { false }

    public var textContentType: IosTextContentType? {
        let uiType = value(forKey: "textContentType") as? UITextContentType
        return IosTextContentTypeMapping.from(uiType)
    }
}

enum IosTextContentTypeMapping {
    static func from(_ type: UITextContentType?) -> IosTextContentType? {
        guard let type else { return nil }
        if #available(iOS 17.0, *) {
            switch type {
            case .creditCardExpiration:
                return .creditCardExpiration
            case .creditCardSecurityCode:
                return .creditCardSecurityCode
            default:
                break
            }
        }
        switch type {
        case .password:
            return .password
        case .newPassword:
            return .newPassword
        case .creditCardNumber:
            return .creditCardNumber
        default:
            return .other
        }
    }
}
#endif

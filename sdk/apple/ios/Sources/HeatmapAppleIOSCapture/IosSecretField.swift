import Foundation

public enum IosSecretField {
    public static func isSecretInteractionTarget(_ hitView: IosCaptureViewNode?) -> Bool {
        guard let field = resolveTextInput(from: hitView) else {
            return false
        }
        return isSecretTextInput(field)
    }

    private static func resolveTextInput(from hitView: IosCaptureViewNode?) -> IosTextInputCaptureNode? {
        var current: IosCaptureViewNode? = hitView
        while let view = current {
            if let input = view as? IosTextInputCaptureNode {
                return input
            }
            current = view.captureSuperview
        }
        return nil
    }

    private static func isSecretTextInput(_ field: IosTextInputCaptureNode) -> Bool {
        if field.isSecureTextEntry {
            return true
        }
        guard let contentType = field.textContentType else {
            return false
        }
        switch contentType {
        case .password, .newPassword, .creditCardNumber, .creditCardExpiration, .creditCardSecurityCode:
            return true
        case .other:
            return false
        }
    }
}

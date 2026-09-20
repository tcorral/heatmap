import Foundation

public enum MacosSecretField {
    public static func isSecretInteractionTarget(_ hitView: MacosCaptureViewNode?) -> Bool {
        guard let field = resolveTextInput(from: hitView) else {
            return false
        }
        return field.isSecureTextField
    }

    private static func resolveTextInput(from hitView: MacosCaptureViewNode?) -> MacosTextInputCaptureNode? {
        var current: MacosCaptureViewNode? = hitView
        while let view = current {
            if let input = view as? MacosTextInputCaptureNode {
                return input
            }
            current = view.captureSuperview
        }
        return nil
    }
}

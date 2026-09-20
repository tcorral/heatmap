import Foundation

public enum IosTapIdentity {
    /// Stable heatmap identity from `accessibilityIdentifier` on the hit view or an ancestor.
    public static func elementId(for hitView: IosCaptureViewNode?) -> String? {
        var current: IosCaptureViewNode? = hitView
        while let view = current {
            if let raw = view.accessibilityIdentifier {
                let trimmed = raw.trimmingCharacters(in: .whitespacesAndNewlines)
                if !trimmed.isEmpty {
                    return trimmed
                }
            }
            current = view.captureSuperview
        }
        return nil
    }
}

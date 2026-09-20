import Foundation

public enum MacosTapIdentity {
    /// Stable heatmap identity: `accessibilityIdentifier`, then `identifier.rawValue` on the hit view or an ancestor.
    public static func elementId(for hitView: MacosCaptureViewNode?) -> String? {
        var current: MacosCaptureViewNode? = hitView
        while let view = current {
            if let fromAccessibility = nonEmptyTrimmed(view.heatmapAccessibilityIdentifier) {
                return fromAccessibility
            }
            if let fromIdentifier = nonEmptyTrimmed(view.heatmapViewIdentifierRawValue) {
                return fromIdentifier
            }
            current = view.captureSuperview
        }
        return nil
    }

    private static func nonEmptyTrimmed(_ raw: String?) -> String? {
        guard let raw else { return nil }
        let trimmed = raw.trimmingCharacters(in: .whitespacesAndNewlines)
        return trimmed.isEmpty ? nil : trimmed
    }
}

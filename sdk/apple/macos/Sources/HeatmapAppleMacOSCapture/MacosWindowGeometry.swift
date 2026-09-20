import CoreGraphics

public enum MacosWindowGeometry {
    /// Normalizes AppKit window coordinates (origin bottom-left) to 0–1 fractions with origin top-left.
    public static func normalizeWindowPosition(
        locationInWindow: CGPoint,
        windowBounds: CGRect
    ) -> NormalizedWindowPosition? {
        let width = windowBounds.width
        let height = windowBounds.height
        guard width > 0, height > 0 else {
            return nil
        }
        let x = clampFraction(Double(locationInWindow.x / width))
        let yFromTop = height - locationInWindow.y
        let y = clampFraction(Double(yFromTop / height))
        return NormalizedWindowPosition(x: x, y: y)
    }

    private static func clampFraction(_ value: Double) -> Double {
        if value < 0 { return 0 }
        if value > 1 { return 1 }
        return value
    }
}

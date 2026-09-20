import CoreGraphics

public enum IosWindowGeometry {
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
        let y = clampFraction(Double(locationInWindow.y / height))
        return NormalizedWindowPosition(x: x, y: y)
    }

    private static func clampFraction(_ value: Double) -> Double {
        if value < 0 { return 0 }
        if value > 1 { return 1 }
        return value
    }
}

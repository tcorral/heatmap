import CoreGraphics

public enum MacosActivationPayloadBuilder {
    public static func buildClick(
        hitView: MacosCaptureViewNode?,
        locationInWindow: CGPoint,
        windowBounds: CGRect,
        windowId: String,
        timestampMs: Int64
    ) -> MacosActivationClickPayload? {
        if MacosSecretField.isSecretInteractionTarget(hitView) {
            return nil
        }
        guard let position = MacosWindowGeometry.normalizeWindowPosition(
            locationInWindow: locationInWindow,
            windowBounds: windowBounds
        ) else {
            return nil
        }
        let width = UInt32(windowBounds.width.rounded(.towardZero))
        let height = UInt32(windowBounds.height.rounded(.towardZero))
        guard width > 0, height > 0 else {
            return nil
        }
        let trimmedWindowId = windowId.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !trimmedWindowId.isEmpty else {
            return nil
        }
        let elementId = MacosTapIdentity.elementId(for: hitView)
        return MacosActivationClickPayload(
            elementId: elementId,
            windowId: trimmedWindowId,
            position: position,
            viewportWidth: width,
            viewportHeight: height,
            timestampMs: timestampMs
        )
    }
}

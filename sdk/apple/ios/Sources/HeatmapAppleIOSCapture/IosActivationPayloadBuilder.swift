import CoreGraphics

public enum IosActivationPayloadBuilder {
    public static func buildClick(
        hitView: IosCaptureViewNode?,
        locationInWindow: CGPoint,
        windowBounds: CGRect,
        timestampMs: Int64
    ) -> IosActivationClickPayload? {
        if IosSecretField.isSecretInteractionTarget(hitView) {
            return nil
        }
        guard let position = IosWindowGeometry.normalizeWindowPosition(
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
        let elementId = IosTapIdentity.elementId(for: hitView)
        return IosActivationClickPayload(
            elementId: elementId,
            position: position,
            viewportWidth: width,
            viewportHeight: height,
            timestampMs: timestampMs
        )
    }
}

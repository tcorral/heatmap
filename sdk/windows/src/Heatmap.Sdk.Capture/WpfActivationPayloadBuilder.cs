namespace Heatmap.Sdk.Capture;

public static class WpfActivationPayloadBuilder
{
    public static WpfActivationClickPayload? BuildClick(
        ICaptureElement? hitElement,
        double locationInWindowX,
        double locationInWindowY,
        double windowWidth,
        double windowHeight,
        string windowId,
        long timestampMs)
    {
        if (WpfSecretField.IsSecretInteractionTarget(hitElement))
        {
            return null;
        }

        var position = WpfWindowGeometry.NormalizeWindowPosition(
            locationInWindowX,
            locationInWindowY,
            windowWidth,
            windowHeight);
        if (position is null)
        {
            return null;
        }

        var width = (uint)Math.Max(0, Math.Floor(windowWidth));
        var height = (uint)Math.Max(0, Math.Floor(windowHeight));
        if (width == 0 || height == 0)
        {
            return null;
        }

        var trimmedWindowId = windowId.Trim();
        if (trimmedWindowId.Length == 0)
        {
            return null;
        }

        var elementId = WpfTapIdentity.ElementId(hitElement);
        return new WpfActivationClickPayload(
            elementId,
            trimmedWindowId,
            position.Value,
            width,
            height,
            timestampMs);
    }
}

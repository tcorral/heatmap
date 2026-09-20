namespace Heatmap.Sdk.Capture;

public static class WpfTapIdentity
{
    /// <summary>
    /// Stable heatmap identity from <c>AutomationProperties.AutomationId</c> on the hit element or an ancestor.
    /// </summary>
    public static string? ElementId(ICaptureElement? hitElement)
    {
        var current = hitElement;
        while (current is not null)
        {
            var fromAutomation = NonEmptyTrimmed(current.AutomationId);
            if (fromAutomation is not null)
            {
                return fromAutomation;
            }

            current = current.CaptureParent;
        }

        return null;
    }

    private static string? NonEmptyTrimmed(string? raw)
    {
        if (raw is null)
        {
            return null;
        }

        var trimmed = raw.Trim();
        return trimmed.Length == 0 ? null : trimmed;
    }
}

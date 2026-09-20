namespace Heatmap.Sdk.Capture;

public static class WpfSecretField
{
    public static bool IsSecretInteractionTarget(ICaptureElement? hitElement)
    {
        var current = hitElement;
        while (current is not null)
        {
            if (current.IsPasswordBox)
            {
                return true;
            }

            current = current.CaptureParent;
        }

        return false;
    }
}

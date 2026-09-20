namespace Heatmap.Sdk.Capture;

public static class WpfWindowGeometry
{
    /// <summary>Normalizes WPF window coordinates (origin top-left, logical DIPs) to 0–1 fractions.</summary>
    public static NormalizedWindowPosition? NormalizeWindowPosition(
        double locationInWindowX,
        double locationInWindowY,
        double windowWidth,
        double windowHeight)
    {
        if (windowWidth <= 0 || windowHeight <= 0)
        {
            return null;
        }

        var x = ClampFraction(locationInWindowX / windowWidth);
        var y = ClampFraction(locationInWindowY / windowHeight);
        return new NormalizedWindowPosition(x, y);
    }

    private static double ClampFraction(double value)
    {
        if (value < 0)
        {
            return 0;
        }

        if (value > 1)
        {
            return 1;
        }

        return value;
    }
}

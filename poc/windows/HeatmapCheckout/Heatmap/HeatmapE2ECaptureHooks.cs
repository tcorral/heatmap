namespace HeatmapCheckout.Heatmap;

public static class HeatmapE2ECaptureHooks
{
    /// <summary>
    /// When set to <c>1</c>, identified controls also record via the capture backend (FlaUI-friendly).
    /// </summary>
    public static bool UsesWindowTapCapture =>
        string.Equals(
            Environment.GetEnvironmentVariable("HEATMAP_E2E_WINDOW_CAPTURE"),
            "1",
            StringComparison.Ordinal);

    public static bool OpenAuxiliaryWindowOnStartup =>
        string.Equals(
            Environment.GetEnvironmentVariable("HEATMAP_E2E_OPEN_AUX_WINDOW"),
            "1",
            StringComparison.Ordinal);

    public static bool TryReadUiScale(out double scale)
    {
        scale = 1.0;
        var raw = Environment.GetEnvironmentVariable("HEATMAP_E2E_UI_SCALE");
        if (string.IsNullOrWhiteSpace(raw))
        {
            return false;
        }

        return double.TryParse(raw, System.Globalization.NumberStyles.Float, null, out scale)
            && scale > 0;
    }
}

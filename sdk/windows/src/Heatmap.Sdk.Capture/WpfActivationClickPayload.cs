namespace Heatmap.Sdk.Capture;

public sealed record WpfActivationClickPayload(
    string? ElementId,
    string WindowId,
    NormalizedWindowPosition Position,
    uint ViewportWidth,
    uint ViewportHeight,
    long TimestampMs);

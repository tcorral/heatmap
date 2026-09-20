namespace Heatmap.Sdk.Capture;

/// <summary>Testable surface for visual-tree identity and secret-field checks (no WPF types).</summary>
public interface ICaptureElement
{
    string? AutomationId { get; }

    ICaptureElement? CaptureParent { get; }

    bool IsPasswordBox { get; }
}

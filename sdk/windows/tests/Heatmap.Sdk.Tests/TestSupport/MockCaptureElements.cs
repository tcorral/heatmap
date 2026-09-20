using Heatmap.Sdk.Capture;

namespace Heatmap.Sdk.Tests.TestSupport;

internal sealed class MockCaptureElement : ICaptureElement
{
    private readonly ICaptureElement? _parent;

    internal MockCaptureElement(
        string? automationId,
        ICaptureElement? parent = null,
        bool isPasswordBox = false)
    {
        AutomationId = automationId;
        _parent = parent;
        IsPasswordBox = isPasswordBox;
    }

    public string? AutomationId { get; }

    public ICaptureElement? CaptureParent => _parent;

    public bool IsPasswordBox { get; }
}

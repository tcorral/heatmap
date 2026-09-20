namespace Heatmap.Sdk.Capture;

public sealed class WpfLifecycle
{
    private readonly IWpfCaptureBackend _backend;

    public WpfLifecycle(IWpfCaptureBackend backend)
    {
        _backend = backend;
    }

    public void OnBackground() => _backend.OnBackground();

    public void OnForeground() => _backend.OnForeground();
}

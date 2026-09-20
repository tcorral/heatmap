namespace Heatmap.Sdk.Capture;

/// <summary>Adapter-only enqueue surface; not part of the public integrator API.</summary>
public interface IWpfCaptureBackend
{
    void RecordActivationClick(WpfActivationClickPayload payload);

    void RecordScreenView(uint viewportWidth, uint viewportHeight, string windowId);

    void RecordScroll(uint viewportWidth, uint viewportHeight, string windowId);

    void OnBackground();

    void OnForeground();
}

using System.Windows;
using Heatmap.Sdk;
using Heatmap.Sdk.Capture;
using Heatmap.Sdk.Wpf;

namespace HeatmapCheckout.Heatmap;

public sealed class HeatmapSession
{
    private readonly HeatmapClient _client;
    private readonly HeatmapWpfRustCaptureBackend _captureBackend;

    public HeatmapSession(HeatmapClient client, HeatmapWpfRustCaptureBackend captureBackend)
    {
        _client = client;
        _captureBackend = captureBackend;
    }

    public void RecordIdentifiedActivation(Window window, string elementId)
    {
        if (!HeatmapE2ECaptureHooks.UsesWindowTapCapture)
        {
            return;
        }

        var width = Math.Max(1.0, window.ActualWidth);
        var height = Math.Max(1.0, window.ActualHeight);
        var element = new IdentifiedCaptureElement(elementId);
        var payload = WpfActivationPayloadBuilder.BuildClick(
            element,
            width / 2,
            height / 2,
            width,
            height,
            WpfWindowIdentity.OpaqueId(window),
            DateTimeOffset.UtcNow.ToUnixTimeMilliseconds());
        if (payload is null)
        {
            return;
        }

        _captureBackend.RecordActivationClick(payload);
        FlushBatch();
    }

    private sealed class IdentifiedCaptureElement(string automationId) : ICaptureElement
    {
        public string? AutomationId => automationId;

        public ICaptureElement? CaptureParent => null;

        public bool IsPasswordBox => false;
    }

    public void TrackScreen(string screenId, string? layoutVersion = null)
    {
        try
        {
            _client.Screen(screenId, layoutVersion);
            RecordViewportScreenView();
        }
        catch (Exception ex)
        {
#if DEBUG
            System.Diagnostics.Debug.WriteLine($"[heatmap] screen({screenId}) failed: {ex}");
#endif
        }
    }

    private void RecordViewportScreenView()
    {
        if (Application.Current?.MainWindow is not Window window)
        {
            return;
        }

        var width = (uint)Math.Max(1.0, window.ActualWidth);
        var height = (uint)Math.Max(1.0, window.ActualHeight);
        _captureBackend.RecordScreenView(width, height);
    }

    public void FlushBatch()
    {
        try
        {
            _client.Flush();
        }
        catch (Exception ex)
        {
#if DEBUG
            System.Diagnostics.Debug.WriteLine($"[heatmap] flush failed: {ex}");
#endif
        }
    }
}

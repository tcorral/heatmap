using Heatmap.Sdk;
using Heatmap.Sdk.Wpf;

namespace HeatmapCheckout.Heatmap;

public sealed class HeatmapRuntime : IDisposable
{
    public HeatmapClient Client { get; }
    public HeatmapWpfRustCaptureBackend CaptureBackend { get; }

    public HeatmapRuntime(HeatmapClient client, HeatmapWpfRustCaptureBackend captureBackend)
    {
        Client = client;
        CaptureBackend = captureBackend;
    }

    public void Dispose() => Client.Dispose();
}

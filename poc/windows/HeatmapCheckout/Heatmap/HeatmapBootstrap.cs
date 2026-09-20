using Heatmap.Sdk;
using Heatmap.Sdk.Wpf;

namespace HeatmapCheckout.Heatmap;

public static class HeatmapBootstrap
{
    public static HeatmapRuntime MakeRuntime(HeatmapClientConfig config)
    {
        var client = new HeatmapClient(config.ProjectKey, config.Endpoint);
        var backend = new HeatmapWpfRustCaptureBackend(client);
        return new HeatmapRuntime(client, backend);
    }
}

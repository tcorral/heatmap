using System.Windows;
using Heatmap.Sdk.Capture;
using Heatmap.Sdk.Wpf;

namespace HeatmapCheckout.Heatmap;

public sealed class HeatmapWpfCaptureHolder
{
    private readonly IWpfCaptureBackend _backend;
    private HeatmapWpfCapture? _capture;

    public HeatmapWpfCaptureHolder(IWpfCaptureBackend backend)
    {
        _backend = backend;
    }

    public void Install(Window window)
    {
        Uninstall();
        _capture = HeatmapWpfCapture.Install(window, _backend);
    }

    public void Uninstall()
    {
        _capture?.Uninstall();
        _capture = null;
    }
}

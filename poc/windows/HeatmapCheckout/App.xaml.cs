using System.Windows;
using HeatmapCheckout.E2E;
using HeatmapCheckout.Heatmap;

namespace HeatmapCheckout;

public partial class App : Application
{
    private HeatmapRuntime? _runtime;
    private HeatmapWpfCaptureHolder? _captureHolder;

    public HeatmapSession HeatmapSession { get; private set; } = null!;

    public HeatmapWpfCaptureHolder CaptureHolder => _captureHolder!;

    protected override void OnStartup(StartupEventArgs e)
    {
        base.OnStartup(e);
        var config = HeatmapConfigReader.Load();
        _runtime = HeatmapBootstrap.MakeRuntime(config);
        HeatmapSession = new HeatmapSession(_runtime.Client, _runtime.CaptureBackend);
        _captureHolder = new HeatmapWpfCaptureHolder(_runtime.CaptureBackend);

        if (HeatmapE2ECaptureHooks.OpenAuxiliaryWindowOnStartup)
        {
            var aux = new E2eAuxiliaryWindow();
            aux.Show();
        }
    }

    protected override void OnExit(ExitEventArgs e)
    {
        _captureHolder?.Uninstall();
        _runtime?.Dispose();
        base.OnExit(e);
    }
}

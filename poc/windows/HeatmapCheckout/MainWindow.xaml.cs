using System.Windows;
using System.Windows.Media;
using HeatmapCheckout.E2E;
using HeatmapCheckout.Heatmap;
using HeatmapCheckout.Views;

namespace HeatmapCheckout;

public partial class MainWindow : Window
{
    public MainWindow()
    {
        InitializeComponent();
        Loaded += OnLoaded;
        MainFrame.Navigate(new HomePage());
    }

    private void OnLoaded(object sender, RoutedEventArgs e)
    {
        if (HeatmapE2ECaptureHooks.TryReadUiScale(out var scale) && scale > 0 && Math.Abs(scale - 1.0) > 0.001)
        {
            LayoutTransform = new ScaleTransform(scale, scale);
        }

        if (Application.Current is App app)
        {
            app.CaptureHolder.Install(this);
        }
    }

}

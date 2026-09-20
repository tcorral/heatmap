using System.Windows;
using HeatmapCheckout.Heatmap;

namespace HeatmapCheckout.E2E;

public partial class E2eAuxiliaryWindow : Window
{
    public E2eAuxiliaryWindow()
    {
        InitializeComponent();
        Loaded += OnLoaded;
        SecondaryMarkButton.Click += OnSecondaryMark;
    }

    private void OnLoaded(object sender, RoutedEventArgs e)
    {
        if (Application.Current is App app)
        {
            app.CaptureHolder.Install(this);
        }
    }

    private void OnSecondaryMark(object sender, RoutedEventArgs e)
    {
        if (Application.Current is App app)
        {
            app.HeatmapSession.RecordIdentifiedActivation(this, "e2e.secondary.mark");
        }
    }
}

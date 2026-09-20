using System.Windows;
using System.Windows.Controls;

namespace HeatmapCheckout.Views;

public partial class ConfirmationPage : Page
{
    public ConfirmationPage()
    {
        InitializeComponent();
        Loaded += OnLoaded;
    }

    private void OnLoaded(object sender, RoutedEventArgs e)
    {
        if (Application.Current is not App app)
        {
            return;
        }

        app.HeatmapSession.TrackScreen("Confirmation");
        app.HeatmapSession.FlushBatch();
    }

    private void OnBackToHome(object sender, RoutedEventArgs e)
    {
        NavigationService?.Navigate(new HomePage());
    }
}

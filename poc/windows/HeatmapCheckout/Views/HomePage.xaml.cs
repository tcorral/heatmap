using System.Windows;
using System.Windows.Controls;
using HeatmapCheckout.E2E;

namespace HeatmapCheckout.Views;

public partial class HomePage : Page
{
    public HomePage()
    {
        InitializeComponent();
        Loaded += OnLoaded;
    }

    private void OnLoaded(object sender, RoutedEventArgs e)
    {
        if (Application.Current is App app)
        {
            app.HeatmapSession.TrackScreen("Home");
        }
    }

    private void OnBrowseProducts(object sender, RoutedEventArgs e)
    {
        NavigationService?.Navigate(new ProductPage());
    }

    private void OnOpenAuxiliaryWindow(object sender, RoutedEventArgs e)
    {
        var owner = Window.GetWindow(this);
        var aux = new E2eAuxiliaryWindow();
        if (owner is not null)
        {
            aux.Owner = owner;
        }

        aux.Show();
    }
}

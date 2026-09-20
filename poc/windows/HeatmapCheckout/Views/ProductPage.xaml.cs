using System.Windows;
using System.Windows.Controls;

namespace HeatmapCheckout.Views;

public partial class ProductPage : Page
{
    public ProductPage()
    {
        InitializeComponent();
    }

    private void OnActivateProduct(object sender, RoutedEventArgs e)
    {
        if (Application.Current is App app && Window.GetWindow(this) is { } window)
        {
            app.HeatmapSession.RecordIdentifiedActivation(window, CheckoutIdentifiers.ProductActivate);
        }

        NavigationService?.Navigate(new CheckoutPage());
    }
}

using System.Windows;
using System.Windows.Controls;

namespace HeatmapCheckout.Views;

public partial class CheckoutPage : Page
{
    public CheckoutPage()
    {
        InitializeComponent();
        Loaded += OnLoaded;
    }

    private void OnLoaded(object sender, RoutedEventArgs e)
    {
        if (Application.Current is App app)
        {
            app.HeatmapSession.TrackScreen("Checkout", "2024-09");
        }
    }

    private void OnApplyCoupon(object sender, RoutedEventArgs e)
    {
        if (Application.Current is App app && Window.GetWindow(this) is { } window)
        {
            app.HeatmapSession.RecordIdentifiedActivation(window, CheckoutIdentifiers.CouponApply);
        }
    }

    private void OnPay(object sender, RoutedEventArgs e)
    {
        if (Application.Current is App app && Window.GetWindow(this) is { } window)
        {
            app.HeatmapSession.RecordIdentifiedActivation(window, CheckoutIdentifiers.PayButton);
        }

        NavigationService?.Navigate(new ConfirmationPage());
    }

    private void OnCancel(object sender, RoutedEventArgs e)
    {
        NavigationService?.Navigate(new ProductPage());
    }
}

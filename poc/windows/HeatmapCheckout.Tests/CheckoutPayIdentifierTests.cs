using HeatmapCheckout;
using Xunit;

namespace HeatmapCheckout.Tests;

public sealed class CheckoutPayIdentifierTests
{
    [Fact]
    public void PayButtonAutomationIdMatchesSharedContract()
    {
        Assert.Equal("checkout.pay", CheckoutIdentifiers.PayButton);
    }
}

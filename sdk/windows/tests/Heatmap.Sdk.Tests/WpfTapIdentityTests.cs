using Heatmap.Sdk.Capture;
using Heatmap.Sdk.Tests.TestSupport;
using Xunit;

namespace Heatmap.Sdk.Tests;

public sealed class WpfTapIdentityTests
{
    [Fact]
    public void ElementIdReturnsAutomationIdFromHitElement()
    {
        var pay = new MockCaptureElement("checkout.pay");
        Assert.Equal("checkout.pay", WpfTapIdentity.ElementId(pay));
    }

    [Fact]
    public void ElementIdWalksAncestorsForAutomationId()
    {
        var parent = new MockCaptureElement("checkout.pay");
        var child = new MockCaptureElement(null, parent: parent);
        Assert.Equal("checkout.pay", WpfTapIdentity.ElementId(child));
    }

    [Fact]
    public void ElementIdReturnsNullWhenUnmarked()
    {
        var unmarked = new MockCaptureElement(null);
        Assert.Null(WpfTapIdentity.ElementId(unmarked));
    }
}

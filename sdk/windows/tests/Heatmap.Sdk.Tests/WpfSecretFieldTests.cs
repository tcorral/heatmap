using Heatmap.Sdk.Capture;
using Heatmap.Sdk.Tests.TestSupport;
using Xunit;

namespace Heatmap.Sdk.Tests;

public sealed class WpfSecretFieldTests
{
    [Fact]
    public void PasswordBoxTargetIsSecret()
    {
        var password = new MockCaptureElement(null, isPasswordBox: true);
        Assert.True(WpfSecretField.IsSecretInteractionTarget(password));
    }

    [Fact]
    public void PasswordBoxAncestorMakesChildSecret()
    {
        var password = new MockCaptureElement(null, isPasswordBox: true);
        var inner = new MockCaptureElement("inner", parent: password);
        Assert.True(WpfSecretField.IsSecretInteractionTarget(inner));
    }

    [Fact]
    public void OrdinaryControlIsNotSecret()
    {
        var button = new MockCaptureElement("checkout.pay");
        Assert.False(WpfSecretField.IsSecretInteractionTarget(button));
    }
}

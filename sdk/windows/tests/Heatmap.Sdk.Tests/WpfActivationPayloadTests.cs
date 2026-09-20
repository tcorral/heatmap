using Heatmap.Sdk.Capture;
using Heatmap.Sdk.Tests.TestSupport;
using Xunit;

namespace Heatmap.Sdk.Tests;

public sealed class WpfActivationPayloadTests
{
    [Fact]
    public void PayMarkedClickPayloadIncludesElementIdAndWindowId()
    {
        var pay = new MockCaptureElement("checkout.pay");
        var built = WpfActivationPayloadBuilder.BuildClick(
            pay,
            locationInWindowX: 120,
            locationInWindowY: 80,
            windowWidth: 800,
            windowHeight: 600,
            windowId: "win_checkout_primary",
            timestampMs: 1_726_771_205_000);

        Assert.NotNull(built);
        Assert.Equal("checkout.pay", built!.ElementId);
        Assert.Equal("win_checkout_primary", built.WindowId);
        Assert.Equal(0.15, built.Position.X, precision: 4);
        Assert.Equal(80.0 / 600.0, built.Position.Y, precision: 4);
        Assert.Equal(800u, built.ViewportWidth);
        Assert.Equal(600u, built.ViewportHeight);
    }

    [Fact]
    public void UnmarkedNonSecretClickPayloadHasCoordinatesOnly()
    {
        var cancel = new MockCaptureElement(null);
        var built = WpfActivationPayloadBuilder.BuildClick(
            cancel,
            locationInWindowX: 200,
            locationInWindowY: 300,
            windowWidth: 800,
            windowHeight: 600,
            windowId: "win_checkout_primary",
            timestampMs: 1);

        Assert.NotNull(built);
        Assert.Null(built!.ElementId);
        Assert.Equal("win_checkout_primary", built.WindowId);
        Assert.Equal(0.25, built.Position.X, precision: 4);
        Assert.Equal(0.5, built.Position.Y, precision: 4);
    }

    [Fact]
    public void PasswordBoxClickProducesNoPayload()
    {
        var password = new MockCaptureElement(null, isPasswordBox: true);
        var payload = WpfActivationPayloadBuilder.BuildClick(
            password,
            locationInWindowX: 40,
            locationInWindowY: 40,
            windowWidth: 800,
            windowHeight: 600,
            windowId: "win_checkout_primary",
            timestampMs: 1);

        Assert.Null(payload);
    }

    [Fact]
    public void ResizeAffectsOnlyNewEventNormalization()
    {
        var view = new MockCaptureElement(null);
        var before = WpfActivationPayloadBuilder.BuildClick(
            view,
            locationInWindowX: 400,
            locationInWindowY: 300,
            windowWidth: 800,
            windowHeight: 600,
            windowId: "win_a",
            timestampMs: 1);
        var after = WpfActivationPayloadBuilder.BuildClick(
            view,
            locationInWindowX: 400,
            locationInWindowY: 150,
            windowWidth: 400,
            windowHeight: 300,
            windowId: "win_a",
            timestampMs: 2);

        Assert.NotNull(before);
        Assert.NotNull(after);
        Assert.Equal(0.5, before!.Position.X, precision: 4);
        Assert.Equal(0.5, before.Position.Y, precision: 4);
        Assert.Equal(1.0, after!.Position.X, precision: 4);
        Assert.Equal(0.5, after.Position.Y, precision: 4);
        Assert.Equal(800u, before.ViewportWidth);
        Assert.Equal(400u, after.ViewportWidth);
    }

    [Fact]
    public void DistinctWindowsCarryDistinctWindowIds()
    {
        var view = new MockCaptureElement("checkout.pay");
        var primary = WpfActivationPayloadBuilder.BuildClick(
            view,
            locationInWindowX: 10,
            locationInWindowY: 10,
            windowWidth: 100,
            windowHeight: 100,
            windowId: "win_primary",
            timestampMs: 1);
        var secondary = WpfActivationPayloadBuilder.BuildClick(
            view,
            locationInWindowX: 10,
            locationInWindowY: 10,
            windowWidth: 100,
            windowHeight: 100,
            windowId: "win_secondary",
            timestampMs: 2);

        Assert.Equal("win_primary", primary!.WindowId);
        Assert.Equal("win_secondary", secondary!.WindowId);
    }

    [Fact]
    public void LogicalDipNormalizationIsStableAcrossProportionalViewports()
    {
        var view = new MockCaptureElement(null);
        var large = WpfActivationPayloadBuilder.BuildClick(
            view,
            locationInWindowX: 400,
            locationInWindowY: 300,
            windowWidth: 800,
            windowHeight: 600,
            windowId: "win_a",
            timestampMs: 1);
        var scaled = WpfActivationPayloadBuilder.BuildClick(
            view,
            locationInWindowX: 200,
            locationInWindowY: 150,
            windowWidth: 400,
            windowHeight: 300,
            windowId: "win_a",
            timestampMs: 2);

        Assert.NotNull(large);
        Assert.NotNull(scaled);
        Assert.Equal(large!.Position.X, scaled!.Position.X, precision: 4);
        Assert.Equal(large.Position.Y, scaled.Position.Y, precision: 4);
    }
}

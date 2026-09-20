using Heatmap.Sdk.Internal;
using Xunit;

namespace Heatmap.Sdk.Tests;

public sealed class HeatmapPublicApiTest
{
    [Fact]
    public void HeatmapClientHasNoPublicTrackMethod()
    {
        var methodNames = typeof(HeatmapClient)
            .GetMethods(System.Reflection.BindingFlags.Public | System.Reflection.BindingFlags.Instance)
            .Select(m => m.Name)
            .ToHashSet(StringComparer.Ordinal);

        Assert.DoesNotContain("Track", methodNames);
    }

    [Fact]
    public void WireActivationTypeIsClickOnly()
    {
        Assert.Equal(new[] { WireActivationType.Click }, Enum.GetValues<WireActivationType>());
    }
}

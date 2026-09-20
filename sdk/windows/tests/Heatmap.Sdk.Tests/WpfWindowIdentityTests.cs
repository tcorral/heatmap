using Heatmap.Sdk.Capture;
using Xunit;

namespace Heatmap.Sdk.Tests;

public sealed class WpfWindowIdentityTests
{
    [Fact]
    public void OpaqueIdIsStableForSameWindowToken()
    {
        var token = new object();
        var first = WpfWindowIdentity.OpaqueId(token);
        var second = WpfWindowIdentity.OpaqueId(token);
        Assert.Equal(first, second);
    }

    [Fact]
    public void OpaqueIdDiffersForDistinctWindowTokens()
    {
        var a = new object();
        var b = new object();
        Assert.NotEqual(WpfWindowIdentity.OpaqueId(a), WpfWindowIdentity.OpaqueId(b));
    }
}

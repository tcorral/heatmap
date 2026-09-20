using System.Runtime.CompilerServices;

namespace Heatmap.Sdk.Capture;

public static class WpfWindowIdentity
{
    /// <summary>Opaque, stable id for a WPF window instance (no titles or user-visible strings).</summary>
    public static string OpaqueId(object windowToken)
    {
        ArgumentNullException.ThrowIfNull(windowToken);
        return $"win-{RuntimeHelpers.GetHashCode(windowToken):X}";
    }
}

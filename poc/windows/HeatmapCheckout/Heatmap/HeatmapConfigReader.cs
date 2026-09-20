using System.IO;
using System.Text.Json;

namespace HeatmapCheckout.Heatmap;

public static class HeatmapConfigReader
{
    private sealed record HeatmapConfigFile(string? ProjectKey, string? Endpoint);

    public static HeatmapClientConfig Load()
    {
        var bundled = ReadBundled();
        var envProjectKey = TrimOrNull(Environment.GetEnvironmentVariable("HEATMAP_PROJECT_KEY"));
        var envEndpoint = TrimOrNull(Environment.GetEnvironmentVariable("HEATMAP_ENDPOINT"));

        var projectKey = envProjectKey
            ?? TrimOrNull(bundled?.ProjectKey)
            ?? HeatmapClientConfig.DefaultProjectKey;
        var endpoint = envEndpoint
            ?? TrimOrNull(bundled?.Endpoint)
            ?? HeatmapClientConfig.DefaultEndpoint;

        return new HeatmapClientConfig(projectKey, endpoint);
    }

    private static HeatmapConfigFile? ReadBundled()
    {
        var path = Path.Combine(AppContext.BaseDirectory, "Config", "heatmap.json");
        if (!File.Exists(path))
        {
            return null;
        }

        try
        {
            var json = File.ReadAllText(path);
            return JsonSerializer.Deserialize<HeatmapConfigFile>(
                json,
                new JsonSerializerOptions { PropertyNameCaseInsensitive = true });
        }
        catch
        {
            return null;
        }
    }

    private static string? TrimOrNull(string? raw)
    {
        if (string.IsNullOrWhiteSpace(raw))
        {
            return null;
        }

        var trimmed = raw.Trim();
        return trimmed.Length == 0 ? null : trimmed;
    }
}

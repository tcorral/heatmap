namespace HeatmapCheckout.E2ETests;

public static class HeatmapE2EConfig
{
    /// <summary>Keep in sync with <c>test/e2e/scripts/run-windows-e2e.ps1</c>.</summary>
    public const int DefaultBackendPort = 18084;

    public static Uri ApiBaseUri
    {
        get
        {
            var port = DefaultBackendPort;
            var fromEnv = Environment.GetEnvironmentVariable("E2E_WINDOWS_BACKEND_PORT")
                ?? Environment.GetEnvironmentVariable("HEATMAP_E2E_BACKEND_PORT");
            if (!string.IsNullOrWhiteSpace(fromEnv) && int.TryParse(fromEnv, out var parsed))
            {
                port = parsed;
            }

            return new Uri($"http://127.0.0.1:{port}");
        }
    }

    public static string HeatmapEndpoint => ApiBaseUri.ToString().TrimEnd('/');
}

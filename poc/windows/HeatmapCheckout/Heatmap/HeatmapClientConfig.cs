namespace HeatmapCheckout.Heatmap;

public sealed record HeatmapClientConfig(string ProjectKey, string Endpoint)
{
    public const string DefaultProjectKey = "test-project-key";
    public const string DefaultEndpoint = "http://127.0.0.1:8080";
}

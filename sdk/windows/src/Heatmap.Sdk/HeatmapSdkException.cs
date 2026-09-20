namespace Heatmap.Sdk;

/// <summary>Thrown when a C ABI call returns a non-zero status code.</summary>
public sealed class HeatmapSdkException : Exception
{
    public HeatmapSdkException(int code, string operation)
        : base($"Heatmap SDK {operation} failed with code {code}.")
    {
        Code = code;
        Operation = operation;
    }

    public int Code { get; }

    public string Operation { get; }
}

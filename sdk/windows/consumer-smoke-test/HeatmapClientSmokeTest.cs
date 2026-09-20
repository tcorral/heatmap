using Xunit;

namespace Heatmap.Sdk.ConsumerSmokeTest;

/// <summary>
/// Consumer-style smoke test: depends on a packed <c>Heatmap.Sdk</c> NuGet from <c>artifacts/</c>
/// and exercises the public API when <c>heatmap_ffi</c> loads.
/// </summary>
public sealed class HeatmapClientSmokeTest
{
    [SkippableFact]
    public void InitFromJsonScreenIdentifySignOutAndFlush()
    {
        Skip.If(!TryEnsureNativeLibrary(), MissingNativeMessage);

        const string config =
            """
            {
                "project_key": "pk_test",
                "endpoint": "https://ingest.example.com"
            }
            """;

        using var client = new HeatmapClient(config);
        client.Screen("Checkout", layoutVersion: "layout-a");
        client.Identify("user-smoke");
        client.SignOut();
        client.SetTrackingEnabled(false);
        client.SetTrackingEnabled(true);
        client.Flush();
    }

    [SkippableFact]
    public void StructuredInitConstructor()
    {
        Skip.If(!TryEnsureNativeLibrary(), MissingNativeMessage);

        using var client = new HeatmapClient("pk_test", "https://ingest.example.com");
        client.Screen("Checkout");
        client.Flush();
    }

    private const string MissingNativeMessage =
        "heatmap_ffi not loaded; run scripts/build-rust-windows.sh (--host-dylib on macOS dev) or set HEATMAP_FFI_LIBRARY_PATH";

    private static bool TryEnsureNativeLibrary()
    {
        if (!string.IsNullOrWhiteSpace(Environment.GetEnvironmentVariable("HEATMAP_FFI_LIBRARY_PATH")))
        {
            return File.Exists(Environment.GetEnvironmentVariable("HEATMAP_FFI_LIBRARY_PATH")!);
        }

        var repoRoot = FindRepoRoot();
        if (repoRoot is null)
        {
            return false;
        }

        var dylib = Path.Combine(repoRoot, "target", "release", "libheatmap_ffi.dylib");
        if (File.Exists(dylib))
        {
            Environment.SetEnvironmentVariable("HEATMAP_FFI_LIBRARY_PATH", dylib);
            return true;
        }

        var dll = Path.Combine(
            repoRoot,
            "sdk",
            "windows",
            "runtimes",
            "win-x64",
            "native",
            "heatmap_ffi.dll");
        if (File.Exists(dll))
        {
            Environment.SetEnvironmentVariable("HEATMAP_FFI_LIBRARY_PATH", dll);
            return true;
        }

        return false;
    }

    private static string? FindRepoRoot()
    {
        var dir = new DirectoryInfo(AppContext.BaseDirectory);
        while (dir is not null)
        {
            if (File.Exists(Path.Combine(dir.FullName, "core", "heatmap-ffi", "Cargo.toml")))
            {
                return dir.FullName;
            }

            dir = dir.Parent;
        }

        return null;
    }
}

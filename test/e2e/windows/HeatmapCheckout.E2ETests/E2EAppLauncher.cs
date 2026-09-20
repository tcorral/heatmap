using System.Diagnostics;
using FlaUI.Core;
using FlaUI.UIA3;

namespace HeatmapCheckout.E2ETests;

public sealed class E2EAppLauncher : IDisposable
{
    private readonly Process _process;
    private readonly Application _application;
    private readonly UIA3Automation _automation;

    private E2EAppLauncher(Process process, Application application, UIA3Automation automation)
    {
        _process = process;
        _application = application;
        _automation = automation;
    }

    public Application Application => _application;

    public UIA3Automation Automation => _automation;

    public static E2EAppLauncher Start(string executablePath, IReadOnlyDictionary<string, string>? extraEnvironment = null)
    {
        if (!File.Exists(executablePath))
        {
            throw new FileNotFoundException("HeatmapCheckout executable not found; build poc/windows first.", executablePath);
        }

        var startInfo = new ProcessStartInfo(executablePath)
        {
            UseShellExecute = false,
            WorkingDirectory = Path.GetDirectoryName(executablePath)!,
        };

        startInfo.Environment["HEATMAP_ENDPOINT"] = HeatmapE2EConfig.HeatmapEndpoint;
        startInfo.Environment["HEATMAP_PROJECT_KEY"] = "test-project-key";
        if (extraEnvironment is not null)
        {
            foreach (var (key, value) in extraEnvironment)
            {
                startInfo.Environment[key] = value;
            }
        }

        var process = Process.Start(startInfo)
            ?? throw new InvalidOperationException("Failed to start HeatmapCheckout.");
        var automation = new UIA3Automation();
        var app = Application.Attach(process);
        return new E2EAppLauncher(process, app, automation);
    }

    public static string ResolveExecutablePath()
    {
        var fromEnv = Environment.GetEnvironmentVariable("HEATMAP_E2E_APP_PATH");
        if (!string.IsNullOrWhiteSpace(fromEnv))
        {
            return Path.GetFullPath(fromEnv);
        }

        var repoRoot = ResolveRepoRoot();
        return Path.Combine(
            repoRoot,
            "poc",
            "windows",
            "HeatmapCheckout",
            "bin",
            "Debug",
            "net8.0-windows",
            "HeatmapCheckout.exe");
    }

    private static string ResolveRepoRoot()
    {
        var dir = new DirectoryInfo(AppContext.BaseDirectory);
        while (dir is not null)
        {
            if (File.Exists(Path.Combine(dir.FullName, "Cargo.toml"))
                && Directory.Exists(Path.Combine(dir.FullName, "poc", "windows")))
            {
                return dir.FullName;
            }

            dir = dir.Parent;
        }

        throw new InvalidOperationException("Could not locate repository root from test output path.");
    }

    public void Dispose()
    {
        try
        {
            if (!_process.HasExited)
            {
                _process.CloseMainWindow();
                if (!_process.WaitForExit(3000))
                {
                    _process.Kill(entireProcessTree: true);
                }
            }
        }
        catch
        {
            // Best-effort shutdown for E2E.
        }

        _automation.Dispose();
        _process.Dispose();
    }
}

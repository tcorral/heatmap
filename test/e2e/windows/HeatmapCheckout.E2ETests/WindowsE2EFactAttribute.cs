namespace HeatmapCheckout.E2ETests;

[AttributeUsage(AttributeTargets.Method)]
public sealed class WindowsE2EFactAttribute : Xunit.FactAttribute
{
    public WindowsE2EFactAttribute()
    {
        if (!OperatingSystem.IsWindows())
        {
            Skip = "Windows E2E runs on Windows only (runner builds this project cross-host).";
        }
    }
}

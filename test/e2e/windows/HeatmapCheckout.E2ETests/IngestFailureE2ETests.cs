using FlaUI.Core.AutomationElements;
using FlaUI.Core.Definitions;
using Xunit;

namespace HeatmapCheckout.E2ETests;

public sealed class IngestFailureE2ETests
{
    [WindowsE2EFact]
    public void HostRemainsUsableWhenIngestUnreachable()
    {
        using var launcher = E2EAppLauncher.Start(
            E2EAppLauncher.ResolveExecutablePath(),
            extraEnvironment: new Dictionary<string, string>
            {
                ["HEATMAP_ENDPOINT"] = "http://127.0.0.1:1",
            });
        var main = CheckoutJourneySteps.WaitForMainWindow(launcher.Application, launcher.Automation);
        CheckoutJourneySteps.RunSharedCheckout(main, launcher.Automation);

        var confirmation = main.FindFirstDescendant(cf => cf.ByName("Confirmation"));
        Assert.NotNull(confirmation);
        var back = main.FindFirstDescendant(cf => cf.ByName("Back to home"));
        Assert.NotNull(back);
    }
}

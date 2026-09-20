using Xunit;

namespace HeatmapCheckout.E2ETests;

public sealed class CheckoutJourneyE2ETests
{
    private readonly HeatmapE2EEventClient _eventClient = new();

    [WindowsE2EFact]
    public async Task CheckoutJourney_ingestsSharedSequenceWithNormalizedCoordinates()
    {
        using var launcher = E2EAppLauncher.Start(E2EAppLauncher.ResolveExecutablePath());
        var main = CheckoutJourneySteps.WaitForMainWindow(launcher.Application, launcher.Automation);
        CheckoutJourneySteps.RunSharedCheckout(main, launcher.Automation);

        var events = await _eventClient.WaitForEventsAsync(minimumCount: 6).ConfigureAwait(false);
        HeatmapE2EEventAssertions.AssertSharedCheckoutSequence(events);
        HeatmapE2EEventAssertions.AssertNormalizedClickCoordinates(events);
        HeatmapE2EEventAssertions.AssertSingleSession(events);
    }
}

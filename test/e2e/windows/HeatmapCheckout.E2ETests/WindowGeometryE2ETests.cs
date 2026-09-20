using System.Runtime.InteropServices;
using System.Text.Json;
using FlaUI.Core.AutomationElements;
using FlaUI.Core.Tools;
using Xunit;

namespace HeatmapCheckout.E2ETests;

public sealed class WindowGeometryE2ETests
{
    private readonly HeatmapE2EEventClient _eventClient = new();

    [DllImport("user32.dll", SetLastError = true)]
    private static extern bool MoveWindow(IntPtr hWnd, int x, int y, int width, int height, bool repaint);

    [WindowsE2EFact]
    public async Task AfterResize_newClicksStayNormalizedToCurrentWindow()
    {
        using var launcher = E2EAppLauncher.Start(E2EAppLauncher.ResolveExecutablePath());
        var main = CheckoutJourneySteps.WaitForMainWindow(launcher.Application, launcher.Automation);
        CheckoutJourneySteps.ClickButton(main, launcher.Automation, "Browse products");
        CheckoutJourneySteps.ClickByAutomationId(main, launcher.Automation, "product");

        var beforeResize = await _eventClient.WaitForEventsAsync(minimumCount: 3).ConfigureAwait(false);
        var beforeViewport = beforeResize
            .Select(HeatmapE2EEventAssertions.Viewport)
            .LastOrDefault(v => v is not null);

        ResizeMainWindow(main, 640, 480);
        Thread.Sleep(400);

        CheckoutJourneySteps.ClickButton(main, launcher.Automation, "Cancel");

        var afterEvents = await _eventClient.WaitForEventsAsync(minimumCount: beforeResize.Count + 1)
            .ConfigureAwait(false);
        var cancelClick = afterEvents.LastOrDefault(e => HeatmapE2EEventAssertions.EventType(e) == "click");
        Assert.NotEqual(default(JsonElement), cancelClick);

        var position = HeatmapE2EEventAssertions.ClickPosition(cancelClick);
        Assert.NotNull(position);
        Assert.InRange(position.Value.X, 0, 1);
        Assert.InRange(position.Value.Y, 0, 1);

        var afterViewport = HeatmapE2EEventAssertions.Viewport(cancelClick);
        if (beforeViewport is not null && afterViewport is not null)
        {
            Assert.True(
                afterViewport.Value.Width < beforeViewport.Value.Width
                || afterViewport.Value.Height < beforeViewport.Value.Height,
                $"Expected viewport to reflect resize (before {beforeViewport}, after {afterViewport}).");
        }
    }

    [WindowsE2EFact]
    public async Task UiScale_newClicksRemainWindowNormalized()
    {
        using var launcher = E2EAppLauncher.Start(
            E2EAppLauncher.ResolveExecutablePath(),
            extraEnvironment: new Dictionary<string, string> { ["HEATMAP_E2E_UI_SCALE"] = "1.25" });
        var main = CheckoutJourneySteps.WaitForMainWindow(launcher.Application, launcher.Automation);
        CheckoutJourneySteps.ClickButton(main, launcher.Automation, "Browse products");
        CheckoutJourneySteps.ClickByAutomationId(main, launcher.Automation, "product");

        var events = await _eventClient.WaitForEventsAsync(minimumCount: 3).ConfigureAwait(false);
        HeatmapE2EEventAssertions.AssertNormalizedClickCoordinates(events);
    }

    [WindowsE2EFact]
    public async Task SecondaryWindow_clicksAttributeToDistinctWindowId()
    {
        using var launcher = E2EAppLauncher.Start(E2EAppLauncher.ResolveExecutablePath());
        var main = CheckoutJourneySteps.WaitForMainWindow(launcher.Application, launcher.Automation);
        CheckoutJourneySteps.ClickButton(main, launcher.Automation, "Browse products");
        var aux = CheckoutJourneySteps.OpenAuxiliaryWindow(launcher.Application, main, launcher.Automation);
        CheckoutJourneySteps.ClickByAutomationId(aux, launcher.Automation, "e2e.secondary.mark");

        var events = await _eventClient.WaitForEventsAsync(minimumCount: 2).ConfigureAwait(false);
        var clicks = events.Where(e => HeatmapE2EEventAssertions.EventType(e) == "click").ToList();
        var secondaryClicks = clicks
            .Where(e => HeatmapE2EEventAssertions.ElementId(e) == "e2e.secondary.mark")
            .ToList();
        var primaryClicks = clicks
            .Where(e => HeatmapE2EEventAssertions.ElementId(e) != "e2e.secondary.mark")
            .ToList();

        Assert.NotEmpty(secondaryClicks);
        HeatmapE2EEventAssertions.AssertDistinctWindowIdsForClicks(primaryClicks, secondaryClicks);
        HeatmapE2EEventAssertions.AssertNormalizedClickCoordinates(secondaryClicks);
    }

    private static void ResizeMainWindow(Window window, int width, int height)
    {
        var handle = window.Properties.NativeWindowHandle;
        var rect = window.BoundingRectangle;
        if (!MoveWindow(handle, (int)rect.X, (int)rect.Y, width, height, true))
        {
            throw new InvalidOperationException("MoveWindow failed to resize the main window.");
        }
    }
}

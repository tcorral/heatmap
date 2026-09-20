using FlaUI.Core;
using FlaUI.Core.AutomationElements;
using FlaUI.Core.Definitions;
using FlaUI.Core.Tools;
using FlaUI.UIA3;

namespace HeatmapCheckout.E2ETests;

public static class CheckoutJourneySteps
{
    public static Window WaitForMainWindow(Application app, UIA3Automation automation, TimeSpan? timeout = null)
    {
        var deadline = DateTime.UtcNow + (timeout ?? TimeSpan.FromSeconds(20));
        while (DateTime.UtcNow < deadline)
        {
            var window = app.GetAllTopLevelWindows(automation)
                .FirstOrDefault(w => w.Name.Contains("Heatmap Checkout", StringComparison.OrdinalIgnoreCase));
            if (window is not null)
            {
                window.Focus();
                return window;
            }

            Thread.Sleep(200);
        }

        throw new TimeoutException("Main Heatmap Checkout window did not appear.");
    }

    public static void RunSharedCheckout(Window mainWindow, UIA3Automation automation)
    {
        ClickButton(mainWindow, automation, "Browse products");
        ClickByAutomationId(mainWindow, automation, "product");
        CompleteCheckoutFromProduct(mainWindow, automation);
    }

    public static void CompleteCheckoutFromProduct(Window mainWindow, UIA3Automation automation)
    {
        Retry.WhileNull(
            () => mainWindow.FindFirstDescendant(cf => cf.ByName("Checkout")),
            timeout: TimeSpan.FromSeconds(10));

        ScrollCheckoutUntilPayVisible(mainWindow, automation);
        ClickByAutomationId(mainWindow, automation, "checkout.coupon");
        ClickByAutomationId(mainWindow, automation, "checkout.pay");

        Retry.WhileNull(
            () => mainWindow.FindFirstDescendant(cf => cf.ByName("Confirmation")),
            timeout: TimeSpan.FromSeconds(15));
    }

    public static void ScrollCheckoutUntilPayVisible(Window mainWindow, UIA3Automation automation)
    {
        var pay = Retry.WhileNull(
            () => mainWindow.FindFirstDescendant(cf => cf.ByAutomationId("checkout.pay")),
            timeout: TimeSpan.FromSeconds(10)).Result;
        if (pay is null)
        {
            throw new InvalidOperationException("Pay button not found on checkout screen.");
        }

        for (var attempt = 0; attempt < 12 && !pay.IsEnabled; attempt++)
        {
            var scrollPattern = mainWindow.Patterns.Scroll.PatternOrDefault;
            scrollPattern?.Scroll(ScrollAmount.NoAmount, ScrollAmount.LargeIncrement);
            Thread.Sleep(150);
            pay = mainWindow.FindFirstDescendant(cf => cf.ByAutomationId("checkout.pay"));
            if (pay is null)
            {
                break;
            }
        }
    }

    public static void ClickButton(Window window, UIA3Automation automation, string name)
    {
        var button = Retry.WhileNull(
            () => window.FindFirstDescendant(cf => cf.ByName(name).And(cf.ByControlType(ControlType.Button))),
            timeout: TimeSpan.FromSeconds(10)).Result
            ?? throw new InvalidOperationException($"Button '{name}' not found.");
        button.AsButton().Invoke();
        Thread.Sleep(200);
    }

    public static void ClickByAutomationId(Window window, UIA3Automation automation, string automationId)
    {
        var button = Retry.WhileNull(
            () => window.FindFirstDescendant(cf => cf.ByAutomationId(automationId)),
            timeout: TimeSpan.FromSeconds(10)).Result
            ?? throw new InvalidOperationException($"Control '{automationId}' not found.");
        button.AsButton().Invoke();
        Thread.Sleep(250);
    }

    public static Window OpenAuxiliaryWindow(Application app, Window mainWindow, UIA3Automation automation)
    {
        ClickByAutomationId(mainWindow, automation, "e2e.open.aux");
        var aux = Retry.WhileNull(
            () => app.GetAllTopLevelWindows(automation)
                .FirstOrDefault(w => w.Name.Contains("E2E auxiliary", StringComparison.OrdinalIgnoreCase)),
            timeout: TimeSpan.FromSeconds(10)).Result
            ?? throw new InvalidOperationException("Auxiliary window did not open.");
        aux.Focus();
        return aux;
    }
}

using System.Windows;
using System.Windows.Automation;
using System.Windows.Controls;
using System.Windows.Media;
using Heatmap.Sdk.Capture;

namespace Heatmap.Sdk.Wpf;

internal sealed class WpfCaptureElement : ICaptureElement
{
    private readonly DependencyObject _node;

    internal WpfCaptureElement(DependencyObject node)
    {
        _node = node;
    }

    public string? AutomationId =>
        NonEmptyTrimmed(AutomationProperties.GetAutomationId(_node));

    public ICaptureElement? CaptureParent
    {
        get
        {
            var parent = VisualTreeHelper.GetParent(_node) ?? LogicalTreeHelper.GetParent(_node);
            return parent is null ? null : new WpfCaptureElement(parent);
        }
    }

    public bool IsPasswordBox => _node is PasswordBox;

    private static string? NonEmptyTrimmed(string? raw)
    {
        if (string.IsNullOrWhiteSpace(raw))
        {
            return null;
        }

        var trimmed = raw.Trim();
        return trimmed.Length == 0 ? null : trimmed;
    }
}

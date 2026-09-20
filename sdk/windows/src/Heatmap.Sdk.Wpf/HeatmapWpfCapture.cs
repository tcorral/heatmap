using System.Windows;
using System.Windows.Controls;
using System.Windows.Input;
using System.Windows.Media;
using Heatmap.Sdk.Capture;

namespace Heatmap.Sdk.Wpf;

/// <summary>WPF routed-event capture for clicks and scrolls; identity from AutomationId only.</summary>
public sealed class HeatmapWpfCapture
{
    private readonly IWpfCaptureBackend _backend;
    private readonly WpfLifecycle _lifecycle;
    private Window? _window;
    private readonly List<ScrollViewer> _scrollTargets = [];

    public HeatmapWpfCapture(IWpfCaptureBackend backend)
    {
        _backend = backend;
        _lifecycle = new WpfLifecycle(backend);
    }

    public static HeatmapWpfCapture Install(Window window, IWpfCaptureBackend backend)
    {
        var capture = new HeatmapWpfCapture(backend);
        capture.Install(window);
        return capture;
    }

    public void Install(Window window)
    {
        Uninstall();
        _window = window;
        RecordScreenView(window);
        window.PreviewMouseLeftButtonDown += OnPreviewMouseLeftButtonDown;
        if (Application.Current is not null)
        {
            Application.Current.Deactivated += OnApplicationDeactivated;
            Application.Current.Activated += OnApplicationActivated;
        }
        if (window.Content is DependencyObject root)
        {
            AttachScrollHandlers(root, window);
        }
    }

    public void Uninstall()
    {
        if (_window is null)
        {
            return;
        }

        _window.PreviewMouseLeftButtonDown -= OnPreviewMouseLeftButtonDown;
        if (Application.Current is not null)
        {
            Application.Current.Deactivated -= OnApplicationDeactivated;
            Application.Current.Activated -= OnApplicationActivated;
        }
        foreach (var scroll in _scrollTargets)
        {
            scroll.ScrollChanged -= OnScrollChanged;
        }

        _scrollTargets.Clear();
        _window = null;
    }

    private void OnPreviewMouseLeftButtonDown(object sender, MouseButtonEventArgs e)
    {
        if (sender is not Window window)
        {
            return;
        }

        var position = e.GetPosition(window);
        var hitElement = HitTestElement(window, position);
        var windowId = WpfWindowIdentity.OpaqueId(window);
        var width = window.ActualWidth;
        var height = window.ActualHeight;
        var payload = WpfActivationPayloadBuilder.BuildClick(
            hitElement,
            position.X,
            position.Y,
            width,
            height,
            windowId,
            DateTimeOffset.UtcNow.ToUnixTimeMilliseconds());
        if (payload is not null)
        {
            _backend.RecordActivationClick(payload);
        }
    }

    private void OnScrollChanged(object sender, ScrollChangedEventArgs e)
    {
        if (_window is not null)
        {
            RecordScroll(_window);
        }
    }

    private void OnApplicationDeactivated(object? sender, EventArgs e) => _lifecycle.OnBackground();

    private void OnApplicationActivated(object? sender, EventArgs e) => _lifecycle.OnForeground();

    private void RecordScreenView(Window window)
    {
        var width = (uint)Math.Max(0, Math.Floor(window.ActualWidth));
        var height = (uint)Math.Max(0, Math.Floor(window.ActualHeight));
        if (width == 0 || height == 0)
        {
            return;
        }

        _backend.RecordScreenView(width, height, WpfWindowIdentity.OpaqueId(window));
    }

    private void RecordScroll(Window window)
    {
        var width = (uint)Math.Max(0, Math.Floor(window.ActualWidth));
        var height = (uint)Math.Max(0, Math.Floor(window.ActualHeight));
        if (width == 0 || height == 0)
        {
            return;
        }

        _backend.RecordScroll(width, height, WpfWindowIdentity.OpaqueId(window));
    }

    private void AttachScrollHandlers(DependencyObject node, Window window)
    {
        if (node is ScrollViewer scrollViewer)
        {
            scrollViewer.ScrollChanged += OnScrollChanged;
            _scrollTargets.Add(scrollViewer);
        }

        var childCount = VisualTreeHelper.GetChildrenCount(node);
        for (var i = 0; i < childCount; i++)
        {
            AttachScrollHandlers(VisualTreeHelper.GetChild(node, i), window);
        }
    }

    private static ICaptureElement? HitTestElement(Window window, Point positionInWindow)
    {
        if (window.Content is not UIElement root)
        {
            return null;
        }

        var pointInRoot = window.TranslatePoint(positionInWindow, root);

        var hit = VisualTreeHelper.HitTest(root, pointInRoot)?.VisualHit;
        return hit is null ? null : new WpfCaptureElement(hit);
    }
}

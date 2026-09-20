using System.Runtime.InteropServices;
using Heatmap.Sdk.Capture;
using Heatmap.Sdk.Internal;

namespace Heatmap.Sdk.Wpf;

/// <summary>Forwards adapter payloads to the C ABI (not exposed on <see cref="HeatmapClient"/>).</summary>
public sealed class HeatmapWpfRustCaptureBackend : IWpfCaptureBackend
{
    private readonly HeatmapClient _client;
    private readonly string _osVersion;
    private readonly string _appVersion;

    public HeatmapWpfRustCaptureBackend(HeatmapClient client, string osVersion, string appVersion)
    {
        _client = client;
        _osVersion = osVersion;
        _appVersion = appVersion;
    }

    public HeatmapWpfRustCaptureBackend(HeatmapClient client)
        : this(client, Environment.OSVersion.VersionString, "0.0.0")
    {
    }

    public void RecordActivationClick(WpfActivationClickPayload payload)
    {
        var elementId = payload.ElementId;
        var windowId = payload.WindowId;
        var x = payload.Position.X;
        var y = payload.Position.Y;
        var viewportWidth = payload.ViewportWidth;
        var viewportHeight = payload.ViewportHeight;
        var timestampMs = payload.TimestampMs;
        _client.EnqueueNative(() =>
            RecordActivationClickNative(
                x, y, elementId, viewportWidth, viewportHeight, timestampMs, windowId));
    }

    public void RecordScreenView(uint viewportWidth, uint viewportHeight, string windowId)
    {
        _client.EnqueueNative(() => RecordScreenViewNative(viewportWidth, viewportHeight, windowId));
    }

    public void RecordScroll(uint viewportWidth, uint viewportHeight, string windowId)
    {
        _client.EnqueueNative(() => RecordScrollNative(viewportWidth, viewportHeight, windowId));
    }

    public void OnBackground()
    {
        _client.EnqueueNative(() =>
        {
            HeatmapNative.EnsureLoaded();
            HeatmapNative.Check(
                HeatmapNativeAdapter.heatmap_sdk_adapter_on_background(_client.Handle),
                "onBackground");
        });
    }

    public void OnForeground()
    {
        _client.EnqueueNative(() =>
        {
            HeatmapNative.EnsureLoaded();
            HeatmapNative.Check(
                HeatmapNativeAdapter.heatmap_sdk_adapter_on_foreground(_client.Handle),
                "onForeground");
        });
    }

    private void RecordActivationClickNative(
        double x,
        double y,
        string? elementId,
        uint viewportWidth,
        uint viewportHeight,
        long timestampMs,
        string windowId)
    {
        HeatmapNative.EnsureLoaded();
        var elementPtr = elementId is null ? IntPtr.Zero : Marshal.StringToCoTaskMemUTF8(elementId);
        var windowPtr = Marshal.StringToCoTaskMemUTF8(windowId);
        var osPtr = Marshal.StringToCoTaskMemUTF8(_osVersion);
        var appPtr = Marshal.StringToCoTaskMemUTF8(_appVersion);
        try
        {
            HeatmapNative.Check(
                HeatmapNativeAdapter.heatmap_sdk_adapter_record_windows_activation_click(
                    _client.Handle,
                    x,
                    y,
                    elementPtr,
                    viewportWidth,
                    viewportHeight,
                    timestampMs,
                    windowPtr,
                    osPtr,
                    appPtr),
                "recordWindowsActivationClick");
        }
        finally
        {
            if (elementPtr != IntPtr.Zero)
            {
                Marshal.FreeCoTaskMem(elementPtr);
            }

            Marshal.FreeCoTaskMem(windowPtr);
            Marshal.FreeCoTaskMem(osPtr);
            Marshal.FreeCoTaskMem(appPtr);
        }
    }

    private void RecordScreenViewNative(uint viewportWidth, uint viewportHeight, string windowId)
    {
        HeatmapNative.EnsureLoaded();
        var windowPtr = Marshal.StringToCoTaskMemUTF8(windowId);
        var osPtr = Marshal.StringToCoTaskMemUTF8(_osVersion);
        var appPtr = Marshal.StringToCoTaskMemUTF8(_appVersion);
        try
        {
            HeatmapNative.Check(
                HeatmapNativeAdapter.heatmap_sdk_adapter_record_windows_screen_view(
                    _client.Handle,
                    viewportWidth,
                    viewportHeight,
                    windowPtr,
                    osPtr,
                    appPtr),
                "recordWindowsScreenView");
        }
        finally
        {
            Marshal.FreeCoTaskMem(windowPtr);
            Marshal.FreeCoTaskMem(osPtr);
            Marshal.FreeCoTaskMem(appPtr);
        }
    }

    private void RecordScrollNative(uint viewportWidth, uint viewportHeight, string windowId)
    {
        HeatmapNative.EnsureLoaded();
        var windowPtr = Marshal.StringToCoTaskMemUTF8(windowId);
        var osPtr = Marshal.StringToCoTaskMemUTF8(_osVersion);
        var appPtr = Marshal.StringToCoTaskMemUTF8(_appVersion);
        try
        {
            HeatmapNative.Check(
                HeatmapNativeAdapter.heatmap_sdk_adapter_record_windows_scroll(
                    _client.Handle,
                    viewportWidth,
                    viewportHeight,
                    windowPtr,
                    osPtr,
                    appPtr),
                "recordWindowsScroll");
        }
        finally
        {
            Marshal.FreeCoTaskMem(windowPtr);
            Marshal.FreeCoTaskMem(osPtr);
            Marshal.FreeCoTaskMem(appPtr);
        }
    }
}

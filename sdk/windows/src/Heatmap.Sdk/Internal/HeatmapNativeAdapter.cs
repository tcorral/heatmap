using System.Runtime.InteropServices;

namespace Heatmap.Sdk.Internal;

internal static class HeatmapNativeAdapter
{
    private const string LibraryName = "heatmap_ffi";

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
    internal static extern int heatmap_sdk_adapter_on_background(IntPtr handle);

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
    internal static extern int heatmap_sdk_adapter_on_foreground(IntPtr handle);

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
    internal static extern int heatmap_sdk_adapter_record_windows_activation_click(
        IntPtr handle,
        double x,
        double y,
        IntPtr elementId,
        uint viewportWidth,
        uint viewportHeight,
        long timestampMs,
        IntPtr windowId,
        IntPtr osVersion,
        IntPtr appVersion);

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
    internal static extern int heatmap_sdk_adapter_record_windows_screen_view(
        IntPtr handle,
        uint viewportWidth,
        uint viewportHeight,
        IntPtr windowId,
        IntPtr osVersion,
        IntPtr appVersion);

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
    internal static extern int heatmap_sdk_adapter_record_windows_scroll(
        IntPtr handle,
        uint viewportWidth,
        uint viewportHeight,
        IntPtr windowId,
        IntPtr osVersion,
        IntPtr appVersion);
}

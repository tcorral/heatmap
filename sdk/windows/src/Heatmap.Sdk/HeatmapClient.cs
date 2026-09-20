using System.IO;
using System.Runtime.InteropServices;
using System.Text.Json;
using System.Threading;
using Heatmap.Sdk.Internal;

namespace Heatmap.Sdk;

/// <summary>
/// App-facing Heatmap SDK client over the stable C ABI (<c>heatmap_ffi</c>).
/// Integrators call init, identify, signOut, screen, setTrackingEnabled, and flush only.
/// Activation capture (wire type <see cref="Internal.WireActivationType.Click"/>) is adapter-owned.
/// </summary>
public sealed class HeatmapClient : IDisposable
{
    private IntPtr _handle;
    private bool _disposed;
    private readonly HeatmapFlushScheduler _flushScheduler;

    /// <summary>Initialize from JSON matching the core <c>HeatmapConfig</c> shape.</summary>
    public HeatmapClient(string configJson)
    {
        ArgumentException.ThrowIfNullOrEmpty(configJson);
        HeatmapNative.EnsureLoaded();
        var configPtr = Marshal.StringToCoTaskMemUTF8(configJson);
        try
        {
            var code = HeatmapNative.heatmap_sdk_init(configPtr, out _handle);
            HeatmapNative.Check(code, "init");
        }
        finally
        {
            Marshal.FreeCoTaskMem(configPtr);
        }
        _flushScheduler = StartFlushScheduler();
    }

    /// <summary>Initialize with required fields and core defaults for optional settings.</summary>
    public HeatmapClient(string projectKey, string endpoint)
        : this(BuildDefaultConfigJson(projectKey, endpoint))
    {
    }

    /// <summary>Share one native handle with a future WPF capture adapter (Step 37).</summary>
    internal HeatmapClient(IntPtr handle)
    {
        if (handle == IntPtr.Zero)
        {
            throw new ArgumentException("SDK handle must not be zero.", nameof(handle));
        }

        _handle = handle;
        _flushScheduler = StartFlushScheduler();
    }

    /// <summary>Opaque C ABI handle for adapter-only use.</summary>
    internal IntPtr Handle => _handle;

    public void Identify(string userId)
    {
        ThrowIfDisposed();
        ArgumentException.ThrowIfNullOrEmpty(userId);
        var userPtr = Marshal.StringToCoTaskMemUTF8(userId);
        try
        {
            HeatmapNative.Check(HeatmapNative.heatmap_sdk_identify(_handle, userPtr), "identify");
        }
        finally
        {
            Marshal.FreeCoTaskMem(userPtr);
        }
    }

    public void SignOut()
    {
        ThrowIfDisposed();
        HeatmapNative.Check(HeatmapNative.heatmap_sdk_sign_out(_handle), "signOut");
    }

    public void Screen(string screenId, string? layoutVersion = null)
    {
        ThrowIfDisposed();
        ArgumentException.ThrowIfNullOrEmpty(screenId);
        var screenPtr = Marshal.StringToCoTaskMemUTF8(screenId);
        var layoutPtr = layoutVersion is null
            ? IntPtr.Zero
            : Marshal.StringToCoTaskMemUTF8(layoutVersion);
        try
        {
            HeatmapNative.Check(
                HeatmapNative.heatmap_sdk_screen(_handle, screenPtr, layoutPtr),
                "screen");
        }
        finally
        {
            Marshal.FreeCoTaskMem(screenPtr);
            if (layoutPtr != IntPtr.Zero)
            {
                Marshal.FreeCoTaskMem(layoutPtr);
            }
        }
    }

    public void SetTrackingEnabled(bool enabled)
    {
        ThrowIfDisposed();
        HeatmapNative.Check(
            HeatmapNative.heatmap_sdk_set_tracking_enabled(_handle, enabled ? (byte)1 : (byte)0),
            "setTrackingEnabled");
    }

    public void Flush()
    {
        ThrowIfDisposed();
        _flushScheduler.Perform(() =>
        {
            HeatmapNative.Check(HeatmapNative.heatmap_sdk_flush(_handle), "flush");
        });
    }

    public void Dispose()
    {
        if (_disposed)
        {
            return;
        }

        _disposed = true;
        _flushScheduler.Dispose();
        if (_handle != IntPtr.Zero)
        {
            HeatmapNative.heatmap_sdk_free(_handle);
            _handle = IntPtr.Zero;
        }
    }

    private HeatmapFlushScheduler StartFlushScheduler()
    {
        return new HeatmapFlushScheduler(TimeSpan.FromSeconds(10), PollFlush);
    }

    private void PollFlush()
    {
        if (_disposed || _handle == IntPtr.Zero)
        {
            return;
        }
        HeatmapNative.Check(HeatmapNative.heatmap_sdk_poll_flush(_handle), "pollFlush");
    }

    private void ThrowIfDisposed()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
    }

    private static string BuildDefaultConfigJson(string projectKey, string endpoint)
    {
        var payload = new
        {
            project_key = projectKey,
            endpoint,
            session_timeout_ms = 1_800_000UL,
            sample_rate = 1.0,
            exclude_elements = Array.Empty<string>(),
            exclude_screens = Array.Empty<string>(),
            mask_elements = Array.Empty<string>(),
            storage_directory = Path.Combine(
                Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData),
                "HeatmapSdk"),
            app_version = "1.0.0",
        };
        return JsonSerializer.Serialize(payload);
    }
}

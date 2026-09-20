using System.Reflection;
using System.Runtime.InteropServices;

namespace Heatmap.Sdk.Internal;

internal static class HeatmapNative
{
    private const string LibraryName = "heatmap_ffi";

    static HeatmapNative()
    {
        NativeLibrary.SetDllImportResolver(typeof(HeatmapNative).Assembly, ResolveLibrary);
    }

    internal static void EnsureLoaded()
    {
        // Accessing any member runs the static constructor and DllImport resolver.
    }

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
    internal static extern int heatmap_sdk_init(
        IntPtr configJson,
        out IntPtr outHandle);

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
    internal static extern void heatmap_sdk_free(IntPtr handle);

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
    internal static extern int heatmap_sdk_identify(IntPtr handle, IntPtr userId);

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
    internal static extern int heatmap_sdk_sign_out(IntPtr handle);

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
    internal static extern int heatmap_sdk_screen(
        IntPtr handle,
        IntPtr screenId,
        IntPtr layoutVersion);

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
    internal static extern int heatmap_sdk_set_tracking_enabled(IntPtr handle, byte enabled);

    [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
    internal static extern int heatmap_sdk_flush(IntPtr handle);

    internal static void Check(int code, string operation)
    {
        if (code != HeatmapFfiCodes.Ok)
        {
            throw new HeatmapSdkException(code, operation);
        }
    }

    private static IntPtr ResolveLibrary(string libraryName, Assembly assembly, DllImportSearchPath? searchPath)
    {
        if (!string.Equals(libraryName, LibraryName, StringComparison.Ordinal))
        {
            return IntPtr.Zero;
        }

        var overridePath = Environment.GetEnvironmentVariable("HEATMAP_FFI_LIBRARY_PATH");
        if (!string.IsNullOrWhiteSpace(overridePath) && File.Exists(overridePath))
        {
            return NativeLibrary.Load(overridePath);
        }

        foreach (var candidate in CandidateLibraryPaths())
        {
            if (!File.Exists(candidate))
            {
                continue;
            }

            try
            {
                return NativeLibrary.Load(candidate);
            }
            catch (DllNotFoundException)
            {
                // try next candidate
            }
        }

        return NativeLibrary.Load(LibraryName, assembly, searchPath);
    }

    private static IEnumerable<string> CandidateLibraryPaths()
    {
        var fileName = PlatformLibraryFileName();
        var baseDir = AppContext.BaseDirectory;

        yield return Path.Combine(baseDir, fileName);

        if (OperatingSystem.IsWindows())
        {
            yield return Path.Combine(baseDir, "runtimes", "win-x64", "native", fileName);
            yield return Path.Combine(baseDir, "runtimes", "win-arm64", "native", fileName);
        }
        else if (OperatingSystem.IsMacOS())
        {
            yield return Path.Combine(baseDir, "runtimes", "osx-arm64", "native", fileName);
            yield return Path.Combine(baseDir, "runtimes", "osx-x64", "native", fileName);
        }
    }

    private static string PlatformLibraryFileName()
    {
        if (OperatingSystem.IsWindows())
        {
            return "heatmap_ffi.dll";
        }

        if (OperatingSystem.IsMacOS())
        {
            return "libheatmap_ffi.dylib";
        }

        return "libheatmap_ffi.so";
    }
}

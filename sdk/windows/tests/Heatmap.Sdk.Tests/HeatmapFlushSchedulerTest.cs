using Xunit;

namespace Heatmap.Sdk.Tests;

public sealed class HeatmapFlushSchedulerTest
{
    [Fact]
    public void PerformDoesNotRunOnCallingThread()
    {
        using var scheduler = new HeatmapFlushScheduler();
        var caller = Environment.CurrentManagedThreadId;
        var workThread = 0;
        scheduler.Perform(() => workThread = Environment.CurrentManagedThreadId);
        Assert.NotEqual(0, workThread);
        Assert.NotEqual(caller, workThread);
    }

    [Fact]
    public void PollingDoesNotRunOnCallingThread()
    {
        var caller = Environment.CurrentManagedThreadId;
        var pollThread = 0;
        using var started = new ManualResetEventSlim(false);
        using var scheduler = new HeatmapFlushScheduler(
            TimeSpan.FromMilliseconds(50),
            () =>
            {
                pollThread = Environment.CurrentManagedThreadId;
                started.Set();
            });
        Assert.True(started.Wait(TimeSpan.FromSeconds(1)));
        Assert.NotEqual(0, pollThread);
        Assert.NotEqual(caller, pollThread);
    }

    [Fact]
    public void PerformStillRunsWork()
    {
        using var scheduler = new HeatmapFlushScheduler();
        var ran = false;
        scheduler.Perform(() => ran = true);
        Assert.True(ran);
    }
}

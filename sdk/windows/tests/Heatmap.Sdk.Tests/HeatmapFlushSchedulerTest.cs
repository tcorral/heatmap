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
        using var done = new ManualResetEventSlim(false);
        scheduler.Perform(() =>
        {
            workThread = Environment.CurrentManagedThreadId;
            done.Set();
        });
        Assert.True(done.Wait(TimeSpan.FromSeconds(1)));
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
        using var ran = new ManualResetEventSlim(false);
        scheduler.Perform(() => ran.Set());
        Assert.True(ran.Wait(TimeSpan.FromSeconds(1)));
    }

    [Fact]
    public void PerformReturnsBeforeBlockedWorkCompletes()
    {
        using var scheduler = new HeatmapFlushScheduler();
        using var started = new ManualResetEventSlim(false);
        using var finished = new ManualResetEventSlim(false);
        var callerContinued = false;
        scheduler.Perform(() =>
        {
            started.Set();
            Thread.Sleep(200);
            Assert.True(callerContinued);
            finished.Set();
        });
        Assert.True(started.Wait(TimeSpan.FromSeconds(1)));
        callerContinued = true;
        Assert.True(finished.Wait(TimeSpan.FromSeconds(1)));
    }
}

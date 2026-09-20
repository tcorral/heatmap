using System.Collections.Concurrent;

namespace Heatmap.Sdk;

/// <summary>Serializes native flush/poll off the capture/UI thread.</summary>
internal sealed class HeatmapFlushScheduler : IDisposable
{
    private readonly BlockingCollection<Action> _work = new();
    private readonly Thread _thread;
    private readonly Timer? _timer;
    private bool _disposed;

    public HeatmapFlushScheduler()
    {
        _thread = new Thread(ProcessWork)
        {
            IsBackground = true,
            Name = "heatmap-flush",
        };
        _thread.Start();
    }

    public HeatmapFlushScheduler(TimeSpan pollInterval, Action poll)
        : this()
    {
        _timer = new Timer(
            _ =>
            {
                try
                {
                    Enqueue(poll);
                }
                catch (ObjectDisposedException)
                {
                }
                catch (InvalidOperationException)
                {
                }
            },
            null,
            pollInterval,
            pollInterval);
    }

    public void Perform(Action work)
    {
        using var done = new ManualResetEventSlim(false);
        Exception? error = null;
        Enqueue(() => error = RunCaptured(work, done));
        done.Wait();
        if (error is not null)
        {
            throw error;
        }
    }

    public void Dispose()
    {
        if (_disposed)
        {
            return;
        }

        _disposed = true;
        _timer?.Dispose();
        _work.CompleteAdding();
        _thread.Join(TimeSpan.FromSeconds(2));
        _work.Dispose();
    }

    private void Enqueue(Action work)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        _work.Add(work);
    }

    private static Exception? RunCaptured(Action work, ManualResetEventSlim done)
    {
        try
        {
            work();
            return null;
        }
        catch (Exception ex)
        {
            return ex;
        }
        finally
        {
            done.Set();
        }
    }

    private void ProcessWork()
    {
        foreach (var work in _work.GetConsumingEnumerable())
        {
            try
            {
                work();
            }
            catch (Exception)
            {
            }
        }
    }
}

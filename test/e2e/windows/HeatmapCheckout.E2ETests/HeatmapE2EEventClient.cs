using System.Net.Http.Json;
using System.Text.Json;

namespace HeatmapCheckout.E2ETests;

public sealed class HeatmapE2EEventClient
{
    private static readonly HttpClient Http = new() { Timeout = TimeSpan.FromSeconds(10) };

    public async Task<IReadOnlyList<JsonElement>> FetchEventsAsync(CancellationToken cancellationToken = default)
    {
        var url = new Uri(HeatmapE2EConfig.ApiBaseUri, "v1/events");
        using var response = await Http.GetAsync(url, cancellationToken).ConfigureAwait(false);
        response.EnsureSuccessStatusCode();
        var document = await response.Content.ReadFromJsonAsync<JsonElement>(cancellationToken: cancellationToken)
            .ConfigureAwait(false);
        if (document.ValueKind != JsonValueKind.Object
            || !document.TryGetProperty("events", out var events)
            || events.ValueKind != JsonValueKind.Array)
        {
            return Array.Empty<JsonElement>();
        }

        return events.EnumerateArray().ToList();
    }

    public IReadOnlyList<JsonElement> LatestSessionEvents(IReadOnlyList<JsonElement> all)
    {
        JsonElement? lastStart = null;
        foreach (var item in all)
        {
            if (HeatmapE2EEventAssertions.EventType(item) == "session_start")
            {
                lastStart = item;
            }
        }

        if (lastStart is null)
        {
            return all;
        }

        var sessionId = HeatmapE2EEventAssertions.SessionId(lastStart.Value);
        if (sessionId is null)
        {
            return all;
        }

        return all.Where(e => HeatmapE2EEventAssertions.SessionId(e) == sessionId).ToList();
    }

    public async Task<IReadOnlyList<JsonElement>> WaitForEventsAsync(
        int minimumCount,
        TimeSpan? timeout = null,
        CancellationToken cancellationToken = default)
    {
        var deadline = DateTime.UtcNow + (timeout ?? TimeSpan.FromSeconds(45));
        var lastCount = 0;
        while (DateTime.UtcNow < deadline)
        {
            cancellationToken.ThrowIfCancellationRequested();
            var all = await FetchEventsAsync(cancellationToken).ConfigureAwait(false);
            var sessionEvents = LatestSessionEvents(all);
            lastCount = sessionEvents.Count;
            if (sessionEvents.Count >= minimumCount)
            {
                return sessionEvents;
            }

            await Task.Delay(TimeSpan.FromMilliseconds(500), cancellationToken).ConfigureAwait(false);
        }

        throw new TimeoutException(
            $"Timed out waiting for at least {minimumCount} events in latest session (last count: {lastCount}).");
    }
}

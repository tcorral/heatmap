using System.Text.Json;
using Xunit;

namespace HeatmapCheckout.E2ETests;

public static class HeatmapE2EEventAssertions
{
    public static string? EventType(JsonElement eventObject)
    {
        if (!eventObject.TryGetProperty("event", out var nested)
            || nested.ValueKind != JsonValueKind.Object
            || !nested.TryGetProperty("type", out var type))
        {
            return null;
        }

        return type.GetString();
    }

    public static string? ElementId(JsonElement eventObject)
    {
        if (!eventObject.TryGetProperty("event", out var nested)
            || nested.ValueKind != JsonValueKind.Object
            || !nested.TryGetProperty("element_id", out var id))
        {
            return null;
        }

        return id.GetString();
    }

    public static string? ScreenId(JsonElement eventObject)
    {
        if (!eventObject.TryGetProperty("screen", out var nested)
            || nested.ValueKind != JsonValueKind.Object
            || !nested.TryGetProperty("id", out var id))
        {
            return null;
        }

        return id.GetString();
    }

    public static string? SessionId(JsonElement eventObject)
    {
        return eventObject.TryGetProperty("session_id", out var id) ? id.GetString() : null;
    }

    public static string? WindowId(JsonElement eventObject)
    {
        return eventObject.TryGetProperty("window_id", out var id) ? id.GetString() : null;
    }

    public static (double X, double Y)? ClickPosition(JsonElement eventObject)
    {
        if (EventType(eventObject) != "click"
            || !eventObject.TryGetProperty("position", out var position)
            || position.ValueKind != JsonValueKind.Object
            || !position.TryGetProperty("x", out var xEl)
            || !position.TryGetProperty("y", out var yEl))
        {
            return null;
        }

        return (xEl.GetDouble(), yEl.GetDouble());
    }

    public static (uint Width, uint Height)? Viewport(JsonElement eventObject)
    {
        if (!eventObject.TryGetProperty("viewport", out var viewport)
            || viewport.ValueKind != JsonValueKind.Object
            || !viewport.TryGetProperty("width", out var w)
            || !viewport.TryGetProperty("height", out var h))
        {
            return null;
        }

        return ((uint)w.GetUInt32(), (uint)h.GetUInt32());
    }

    public static void AssertSharedCheckoutSequence(IReadOnlyList<JsonElement> events)
    {
        var types = events.Select(EventType).Where(t => t is not null).Cast<string>().ToList();
        Assert.Contains("session_start", types);
        Assert.Contains("scroll", types);
        Assert.True(types.Count(t => t == "click") >= 3);

        var screenViews = events
            .Where(e => EventType(e) == "screen_view")
            .Select(e => ScreenId(e)?.ToLowerInvariant())
            .Where(s => s is not null)
            .Cast<string>()
            .ToList();
        foreach (var expected in new[] { "home", "checkout", "confirmation" })
        {
            Assert.Contains(expected, screenViews);
        }

        var clickIds = events
            .Where(e => EventType(e) == "click")
            .Select(ElementId)
            .Where(id => id is not null)
            .Cast<string>()
            .ToList();
        foreach (var expected in new[] { "product", "checkout.coupon", "checkout.pay" })
        {
            Assert.Contains(expected, clickIds);
        }
    }

    public static void AssertNormalizedClickCoordinates(IReadOnlyList<JsonElement> events)
    {
        var positions = events.Select(ClickPosition).Where(p => p.HasValue).Select(p => p!.Value).ToList();
        Assert.NotEmpty(positions);
        foreach (var (x, y) in positions)
        {
            Assert.InRange(x, 0, 1);
            Assert.InRange(y, 0, 1);
        }
    }

    public static void AssertSingleSession(IReadOnlyList<JsonElement> events)
    {
        var sessionIds = events.Select(SessionId).Where(s => s is not null).Cast<string>().Distinct().ToList();
        Assert.Single(sessionIds);
    }

    public static void AssertDistinctWindowIdsForClicks(
        IReadOnlyList<JsonElement> primaryClicks,
        IReadOnlyList<JsonElement> secondaryClicks)
    {
        var primaryIds = primaryClicks.Select(WindowId).Where(w => w is not null).Cast<string>().Distinct().ToList();
        var secondaryIds = secondaryClicks.Select(WindowId).Where(w => w is not null).Cast<string>().Distinct().ToList();
        Assert.NotEmpty(primaryIds);
        Assert.NotEmpty(secondaryIds);
        Assert.DoesNotContain(primaryIds[0], secondaryIds);
    }
}

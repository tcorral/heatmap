package com.heatmap.checkout.e2e

import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue

object HeatmapE2EEventAssertions {
    fun eventType(event: Map<String, Any?>): String? {
        val nested = event["event"] as? Map<*, *> ?: return null
        return nested["type"] as? String
    }

    fun elementId(event: Map<String, Any?>): String? {
        val nested = event["event"] as? Map<*, *> ?: return null
        return nested["element_id"] as? String
    }

    fun screenId(event: Map<String, Any?>): String? {
        val nested = event["screen"] as? Map<*, *> ?: return null
        return nested["id"] as? String
    }

    fun sessionId(event: Map<String, Any?>): String? = event["session_id"] as? String

    fun clickPosition(event: Map<String, Any?>): Pair<Double, Double>? {
        if (eventType(event) != "click") return null
        val position = event["position"] as? Map<*, *> ?: return null
        val x = (position["x"] as? Number)?.toDouble() ?: return null
        val y = (position["y"] as? Number)?.toDouble() ?: return null
        return x to y
    }

    fun assertSharedCheckoutSequence(events: List<Map<String, Any?>>) {
        val types = events.mapNotNull { eventType(it) }
        assertTrue(types.contains("session_start"))
        assertTrue(types.contains("scroll"))
        assertTrue(types.count { it == "click" } >= 3)

        val screenViews =
            events
                .filter { eventType(it) == "screen_view" }
                .mapNotNull { screenId(it)?.lowercase() }
        for (expected in listOf("home", "checkout", "confirmation")) {
            assertTrue("Missing screen_view $expected", screenViews.contains(expected))
        }

        val clickIds =
            events
                .filter { eventType(it) == "click" }
                .mapNotNull { elementId(it) }
        for (expected in listOf("product", "checkout.coupon", "checkout.pay")) {
            assertTrue("Missing click $expected", clickIds.contains(expected))
        }
    }

    fun assertNormalizedClickCoordinates(events: List<Map<String, Any?>>) {
        val positions = events.mapNotNull { clickPosition(it) }
        assertFalse("Expected click positions", positions.isEmpty())
        positions.forEach { (x, y) ->
            assertTrue(x in 0.0..1.0)
            assertTrue(y in 0.0..1.0)
        }
    }

    fun assertSingleSession(events: List<Map<String, Any?>>) {
        val sessionIds = events.mapNotNull { sessionId(it) }.toSet()
        assertTrue("Expected one session_id, got $sessionIds", sessionIds.size == 1)
    }
}

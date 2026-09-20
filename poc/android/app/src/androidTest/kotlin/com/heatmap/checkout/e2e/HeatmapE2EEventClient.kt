package com.heatmap.checkout.e2e

import org.json.JSONObject
import java.net.HttpURLConnection
import java.net.URL

class HeatmapE2EEventClient {
    fun fetchEvents(): List<Map<String, Any?>> {
        val url = URL("${HeatmapE2EConfig.apiBaseUrl}/v1/events")
        val connection = (url.openConnection() as HttpURLConnection).apply {
            requestMethod = "GET"
            connectTimeout = 5_000
            readTimeout = 5_000
        }
        connection.use {
            check(it.responseCode == HttpURLConnection.HTTP_OK) {
                "GET /v1/events failed: ${it.responseCode}"
            }
            val body = it.inputStream.bufferedReader().readText()
            val json = JSONObject(body)
            val events = json.optJSONArray("events") ?: return emptyList()
            return (0 until events.length()).map { index ->
                jsonObjectToMap(events.getJSONObject(index))
            }
        }
    }

    fun latestSessionEvents(all: List<Map<String, Any?>>): List<Map<String, Any?>> {
        val lastStart =
            all.lastOrNull { HeatmapE2EEventAssertions.eventType(it) == "session_start" }
                ?: return all
        val sessionId = HeatmapE2EEventAssertions.sessionId(lastStart) ?: return all
        return all.filter { HeatmapE2EEventAssertions.sessionId(it) == sessionId }
    }

    fun waitForEvents(
        minimumCount: Int,
        timeoutMs: Long = 45_000,
        pollIntervalMs: Long = 500,
    ): List<Map<String, Any?>> {
        val deadline = System.currentTimeMillis() + timeoutMs
        var lastCount = 0
        while (System.currentTimeMillis() < deadline) {
            val allEvents = fetchEvents()
            val events = latestSessionEvents(allEvents)
            lastCount = events.size
            if (events.size >= minimumCount) {
                return events
            }
            Thread.sleep(pollIntervalMs)
        }
        error("Timed out waiting for at least $minimumCount events (last count: $lastCount)")
    }

    fun waitForPayClickCount(
        minimumPayClicks: Int,
        timeoutMs: Long = 45_000,
    ): Int {
        val deadline = System.currentTimeMillis() + timeoutMs
        while (System.currentTimeMillis() < deadline) {
            val count = countPayClickEvents(fetchEvents())
            if (count >= minimumPayClicks) {
                return count
            }
            Thread.sleep(500)
        }
        error("Timed out waiting for $minimumPayClicks checkout.pay clicks")
    }

    fun countPayClickEvents(all: List<Map<String, Any?>>): Int =
        all.count {
            HeatmapE2EEventAssertions.eventType(it) == "click" &&
                HeatmapE2EEventAssertions.elementId(it) == "checkout.pay"
        }

    private fun jsonObjectToMap(json: JSONObject): Map<String, Any?> {
        val map = linkedMapOf<String, Any?>()
        json.keys().forEach { key ->
            map[key] =
                when (val value = json.get(key)) {
                    is JSONObject -> jsonObjectToMap(value)
                    else -> value
                }
        }
        return map
    }
}

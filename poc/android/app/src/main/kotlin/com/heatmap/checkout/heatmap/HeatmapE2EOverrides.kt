package com.heatmap.checkout.heatmap

import android.content.Context

/** Debug-only endpoint override for instrumentation E2E (set before cold start). */
object HeatmapE2EOverrides {
    private const val PREFS = "heatmap_e2e_overrides"
    private const val KEY_ENDPOINT = "endpoint"

    fun readEndpointOverride(context: Context): String? =
        context
            .getSharedPreferences(PREFS, Context.MODE_PRIVATE)
            .getString(KEY_ENDPOINT, null)
            ?.trim()
            ?.takeIf { it.isNotEmpty() }

    fun setEndpointOverride(
        context: Context,
        endpoint: String?,
    ) {
        context.getSharedPreferences(PREFS, Context.MODE_PRIVATE).edit().apply {
            if (endpoint.isNullOrBlank()) {
                remove(KEY_ENDPOINT)
            } else {
                putString(KEY_ENDPOINT, endpoint.trim())
            }
        }.apply()
    }
}

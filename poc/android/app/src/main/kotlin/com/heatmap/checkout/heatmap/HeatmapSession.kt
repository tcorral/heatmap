package com.heatmap.checkout.heatmap

import com.heatmap.sdk.HeatmapClient

class HeatmapSession(
    val client: HeatmapClient,
) {
    fun trackScreen(
        screenId: String,
        layoutVersion: String? = null,
    ) {
        client.screen(screenId, layoutVersion)
    }

    fun flushBatch() {
        client.flush()
    }
}

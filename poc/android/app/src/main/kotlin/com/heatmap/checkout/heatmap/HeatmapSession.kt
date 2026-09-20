package com.heatmap.checkout.heatmap

import android.app.Application
import com.heatmap.sdk.HeatmapClient
import com.heatmap.sdk.capture.view.HeatmapViewCaptureBackend

class HeatmapSession(
    val client: HeatmapClient,
    private val captureBackend: HeatmapViewCaptureBackend,
    private val application: Application,
) {
    fun trackScreen(
        screenId: String,
        layoutVersion: String? = null,
    ) {
        client.screen(screenId, layoutVersion)
        viewportSize()?.let { (width, height) ->
            captureBackend.recordScreenView(width, height)
        }
    }

    fun recordViewportScroll() {
        viewportSize()?.let { (width, height) ->
            captureBackend.recordScroll(width, height)
        }
    }

    fun flushBatch() {
        client.flush()
    }

    private fun viewportSize(): Pair<UInt, UInt>? {
        val metrics = application.resources.displayMetrics
        val width = metrics.widthPixels.coerceAtLeast(0)
        val height = metrics.heightPixels.coerceAtLeast(0)
        if (width <= 0 || height <= 0) {
            return null
        }
        return width.toUInt() to height.toUInt()
    }
}

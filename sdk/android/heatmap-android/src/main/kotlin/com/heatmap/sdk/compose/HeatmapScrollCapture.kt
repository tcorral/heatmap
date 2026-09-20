package com.heatmap.sdk.compose

import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.composed
import androidx.compose.ui.input.nestedscroll.nestedScroll
import com.heatmap.sdk.capture.compose.ComposeScrollObserver
import com.heatmap.sdk.capture.compose.HeatmapComposeNestedScrollConnection
import com.heatmap.sdk.capture.view.HeatmapViewCaptureBackend

/** Adapter-owned Compose scroll capture from nested-scroll consumption. */
fun Modifier.heatmapScrollCapture(
    backend: HeatmapViewCaptureBackend,
    viewportWidth: () -> UInt,
    viewportHeight: () -> UInt,
): Modifier =
    composed {
        val observer = remember { ComposeScrollObserver() }
        val connection =
            remember(backend) {
                HeatmapComposeNestedScrollConnection(
                    backend = backend,
                    observer = observer,
                    viewportWidth = viewportWidth,
                    viewportHeight = viewportHeight,
                )
            }
        nestedScroll(connection)
    }

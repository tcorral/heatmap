package com.heatmap.sdk.capture.compose

import com.heatmap.sdk.capture.view.HeatmapViewCaptureBackend

/** Records Compose viewport scroll from pointer-move without a host scroll reporter. */
internal object ComposeScrollObserver {
    const val ACTION_MOVE: Int = 2

    fun onPointerAction(
        actionMasked: Int,
        viewportWidth: UInt,
        viewportHeight: UInt,
        backend: HeatmapViewCaptureBackend,
    ) {
        if (actionMasked != ACTION_MOVE) {
            return
        }
        if (viewportWidth == 0u || viewportHeight == 0u) {
            return
        }
        backend.recordScroll(viewportWidth, viewportHeight)
    }
}

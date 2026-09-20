package com.heatmap.sdk.capture.compose

import com.heatmap.sdk.capture.view.HeatmapViewCaptureBackend
import kotlin.math.abs

internal data class ComposeScrollSignal(
    val actionMasked: Int,
    val viewportWidth: UInt,
    val viewportHeight: UInt,
    val consumedDeltaX: Float = 0f,
    val consumedDeltaY: Float = 0f,
)

/** Records Compose scroll from consumed nested-scroll deltas, coalesced per gesture. */
internal class ComposeScrollObserver {
    private var recordedThisGesture = false

    fun onPointerAction(
        signal: ComposeScrollSignal,
        backend: HeatmapViewCaptureBackend,
    ) {
        if (isGestureBoundary(signal.actionMasked)) {
            recordedThisGesture = false
            return
        }
        if (!shouldRecord(signal)) {
            return
        }
        backend.recordScroll(signal.viewportWidth, signal.viewportHeight)
        recordedThisGesture = true
    }

    private fun shouldRecord(signal: ComposeScrollSignal): Boolean {
        if (signal.actionMasked != ACTION_MOVE || recordedThisGesture) {
            return false
        }
        if (signal.viewportWidth == 0u || signal.viewportHeight == 0u) {
            return false
        }
        return abs(signal.consumedDeltaX) >= MIN_CONSUMED_PX ||
            abs(signal.consumedDeltaY) >= MIN_CONSUMED_PX
    }

    private fun isGestureBoundary(actionMasked: Int): Boolean =
        actionMasked == ACTION_DOWN || actionMasked == ACTION_UP || actionMasked == ACTION_CANCEL

    companion object {
        const val ACTION_DOWN: Int = 0
        const val ACTION_UP: Int = 1
        const val ACTION_MOVE: Int = 2
        const val ACTION_CANCEL: Int = 3
        const val MIN_CONSUMED_PX: Float = 8f
    }
}

package com.heatmap.sdk.capture.compose

import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.input.nestedscroll.NestedScrollConnection
import androidx.compose.ui.input.nestedscroll.NestedScrollSource
import androidx.compose.ui.unit.Velocity
import com.heatmap.sdk.capture.view.HeatmapViewCaptureBackend

internal class HeatmapComposeNestedScrollConnection(
    private val backend: HeatmapViewCaptureBackend,
    private val observer: ComposeScrollObserver,
    private val viewportWidth: () -> UInt,
    private val viewportHeight: () -> UInt,
) : NestedScrollConnection {
    override fun onPostScroll(
        consumed: Offset,
        available: Offset,
        source: NestedScrollSource,
    ): Offset {
        observer.onPointerAction(
            signal = ComposeScrollSignal(
                actionMasked = ComposeScrollObserver.ACTION_MOVE,
                viewportWidth = viewportWidth(),
                viewportHeight = viewportHeight(),
                consumedDeltaX = consumed.x,
                consumedDeltaY = consumed.y,
            ),
            backend = backend,
        )
        return Offset.Zero
    }

    override suspend fun onPostFling(
        consumed: Velocity,
        available: Velocity,
    ): Velocity {
        observer.onPointerAction(
            signal = ComposeScrollSignal(
                actionMasked = ComposeScrollObserver.ACTION_UP,
                viewportWidth = viewportWidth(),
                viewportHeight = viewportHeight(),
            ),
            backend = backend,
        )
        return Velocity.Zero
    }
}

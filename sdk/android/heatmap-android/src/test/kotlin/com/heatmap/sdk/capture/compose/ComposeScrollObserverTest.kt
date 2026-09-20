package com.heatmap.sdk.capture.compose

import com.heatmap.sdk.capture.view.HeatmapViewCaptureBackend
import com.heatmap.sdk.capture.view.ViewActivationClickPayload
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class ComposeScrollObserverTest {
    @Test
    fun shouldNotRecordScrollWhenPointerMovesWithoutConsumedDelta() {
        val backend = RecordingBackend()
        ComposeScrollObserver().onPointerAction(
            signal = move(consumedDeltaY = 1f),
            backend = backend,
        )
        assertTrue(backend.scrolls.isEmpty())
    }

    @Test
    fun shouldRecordScrollWhenConsumedDeltaExceedsThreshold() {
        val backend = RecordingBackend()
        ComposeScrollObserver().onPointerAction(
            signal = move(consumedDeltaY = 16f),
            backend = backend,
        )
        assertEquals(listOf(800u to 600u), backend.scrolls)
        assertTrue(backend.clicks.isEmpty())
    }

    @Test
    fun shouldCoalesceScrollsInTheSameGesture() {
        val backend = RecordingBackend()
        val observer = ComposeScrollObserver()
        observer.onPointerAction(signal = move(consumedDeltaY = 16f), backend = backend)
        observer.onPointerAction(signal = move(consumedDeltaY = 16f), backend = backend)
        assertEquals(1, backend.scrolls.size)
    }

    @Test
    fun shouldRecordScrollAgainAfterGestureEnds() {
        val backend = RecordingBackend()
        val observer = ComposeScrollObserver()
        observer.onPointerAction(signal = move(consumedDeltaY = 16f), backend = backend)
        observer.onPointerAction(
            signal = ComposeScrollSignal(
                actionMasked = ComposeScrollObserver.ACTION_UP,
                viewportWidth = 800u,
                viewportHeight = 600u,
            ),
            backend = backend,
        )
        observer.onPointerAction(signal = move(consumedDeltaY = 16f), backend = backend)
        assertEquals(2, backend.scrolls.size)
    }

    @Test
    fun shouldNotRecordScrollOnPointerUp() {
        val backend = RecordingBackend()
        ComposeScrollObserver().onPointerAction(
            signal = ComposeScrollSignal(
                actionMasked = ComposeScrollObserver.ACTION_UP,
                viewportWidth = 800u,
                viewportHeight = 600u,
            ),
            backend = backend,
        )
        assertTrue(backend.scrolls.isEmpty())
    }

    @Test
    fun shouldNotRecordScrollWhenViewportIsZero() {
        val backend = RecordingBackend()
        ComposeScrollObserver().onPointerAction(
            signal = ComposeScrollSignal(
                actionMasked = ComposeScrollObserver.ACTION_MOVE,
                viewportWidth = 0u,
                viewportHeight = 600u,
                consumedDeltaY = 16f,
            ),
            backend = backend,
        )
        assertTrue(backend.scrolls.isEmpty())
    }

    private fun move(consumedDeltaY: Float): ComposeScrollSignal =
        ComposeScrollSignal(
            actionMasked = ComposeScrollObserver.ACTION_MOVE,
            viewportWidth = 800u,
            viewportHeight = 600u,
            consumedDeltaY = consumedDeltaY,
        )

    private class RecordingBackend : HeatmapViewCaptureBackend {
        val scrolls = mutableListOf<Pair<UInt, UInt>>()
        val clicks = mutableListOf<ViewActivationClickPayload>()

        override fun recordActivationClick(payload: ViewActivationClickPayload) {
            clicks.add(payload)
        }

        override fun recordScreenView(
            viewportWidth: UInt,
            viewportHeight: UInt,
        ) {
        }

        override fun recordScroll(
            viewportWidth: UInt,
            viewportHeight: UInt,
        ) {
            scrolls.add(viewportWidth to viewportHeight)
        }

        override fun onBackground() {}

        override fun onForeground() {}
    }
}

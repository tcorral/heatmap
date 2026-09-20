package com.heatmap.sdk.capture.compose

import com.heatmap.sdk.capture.view.HeatmapViewCaptureBackend
import com.heatmap.sdk.capture.view.ViewActivationClickPayload
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class ComposeScrollObserverTest {
    @Test
    fun shouldRecordScrollWhenPointerMoves() {
        val backend = RecordingBackend()
        ComposeScrollObserver.onPointerAction(
            actionMasked = ComposeScrollObserver.ACTION_MOVE,
            viewportWidth = 800u,
            viewportHeight = 600u,
            backend = backend,
        )
        assertEquals(listOf(800u to 600u), backend.scrolls)
        assertTrue(backend.clicks.isEmpty())
    }

    @Test
    fun shouldNotRecordScrollOnPointerUp() {
        val backend = RecordingBackend()
        ComposeScrollObserver.onPointerAction(
            actionMasked = 1,
            viewportWidth = 800u,
            viewportHeight = 600u,
            backend = backend,
        )
        assertTrue(backend.scrolls.isEmpty())
    }

    @Test
    fun shouldNotRecordScrollWhenViewportIsZero() {
        val backend = RecordingBackend()
        ComposeScrollObserver.onPointerAction(
            actionMasked = ComposeScrollObserver.ACTION_MOVE,
            viewportWidth = 0u,
            viewportHeight = 600u,
            backend = backend,
        )
        assertTrue(backend.scrolls.isEmpty())
    }

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

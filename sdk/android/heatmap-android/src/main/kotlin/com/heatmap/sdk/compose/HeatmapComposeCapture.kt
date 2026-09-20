package com.heatmap.sdk.compose

import android.view.MotionEvent
import android.view.View
import androidx.compose.ui.platform.AbstractComposeView
import androidx.compose.ui.platform.AndroidComposeView
import androidx.lifecycle.DefaultLifecycleObserver
import androidx.lifecycle.LifecycleOwner
import androidx.lifecycle.ProcessLifecycleOwner
import com.heatmap.sdk.capture.compose.ComposeActivationPayloadBuilder
import com.heatmap.sdk.capture.compose.ComposeScrollObserver
import com.heatmap.sdk.capture.compose.ComposeSemanticsHitTest
import com.heatmap.sdk.capture.view.HeatmapViewCaptureBackend

/**
 * Captures Compose activations via the semantics tree (not View hit-testing for composable identity).
 */
class HeatmapComposeCapture private constructor(
    private val backend: HeatmapViewCaptureBackend,
) {
    private var composeView: AbstractComposeView? = null
    private var touchListenerAttached = false
    private val processObserver =
        object : DefaultLifecycleObserver {
            override fun onStop(owner: LifecycleOwner) {
                backend.onBackground()
            }

            override fun onStart(owner: LifecycleOwner) {
                backend.onForeground()
            }
        }

    companion object {
        @JvmStatic
        fun install(
            composeView: AbstractComposeView,
            backend: HeatmapViewCaptureBackend,
        ): HeatmapComposeCapture {
            val capture = HeatmapComposeCapture(backend)
            capture.install(composeView)
            return capture
        }
    }

    fun install(composeView: AbstractComposeView) {
        uninstall()
        this.composeView = composeView
        composeView.setOnTouchListener(::onComposeTouch)
        touchListenerAttached = true
        ProcessLifecycleOwner.get().lifecycle.addObserver(processObserver)
    }

    fun uninstall() {
        if (touchListenerAttached) {
            composeView?.setOnTouchListener(null)
        }
        touchListenerAttached = false
        composeView = null
        ProcessLifecycleOwner.get().lifecycle.removeObserver(processObserver)
    }

    private fun onComposeTouch(
        _view: View,
        event: MotionEvent,
    ): Boolean {
        val compose = composeView ?: return false
        ComposeScrollObserver.onPointerAction(
            actionMasked = event.actionMasked,
            viewportWidth = compose.rootView.width.toUInt(),
            viewportHeight = compose.rootView.height.toUInt(),
            backend = backend,
        )
        if (event.actionMasked != MotionEvent.ACTION_UP) {
            return false
        }
        return recordComposeClick(event, compose)
    }

    private fun recordComposeClick(
        event: MotionEvent,
        compose: AbstractComposeView,
    ): Boolean {
        val androidCompose = compose as? AndroidComposeView ?: return false
        val root = compose.rootView
        val windowWidth = root.width.toFloat()
        val windowHeight = root.height.toFloat()
        if (windowWidth <= 0f || windowHeight <= 0f) {
            return false
        }
        val locationInWindow = IntArray(2)
        compose.getLocationInWindow(locationInWindow)
        val xInWindow = locationInWindow[0] + event.x
        val yInWindow = locationInWindow[1] + event.y
        val semanticsNode =
            ComposeSemanticsHitTest.findSemanticsNodeAt(
                owner = androidCompose.semanticsOwner,
                xInWindow = xInWindow,
                yInWindow = yInWindow,
            ) ?: return false
        val payload =
            ComposeActivationPayloadBuilder.buildClick(
                hitNode = captureNodeFor(semanticsNode),
                locationInWindowX = xInWindow,
                locationInWindowY = yInWindow,
                windowWidth = windowWidth,
                windowHeight = windowHeight,
                timestampMs = System.currentTimeMillis(),
            ) ?: return false
        backend.recordActivationClick(payload)
        return false
    }
}

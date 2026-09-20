package com.heatmap.sdk.compose

import android.view.MotionEvent
import android.view.View
import android.view.ViewGroup
import androidx.compose.ui.platform.AbstractComposeView
import androidx.compose.ui.platform.ViewRootForTest
import androidx.lifecycle.DefaultLifecycleObserver
import androidx.lifecycle.LifecycleOwner
import androidx.lifecycle.ProcessLifecycleOwner
import com.heatmap.sdk.capture.compose.ComposeActivationPayloadBuilder
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
        if (event.actionMasked != MotionEvent.ACTION_UP) {
            return false
        }
        return recordComposeClick(event, compose)
    }

    private fun recordComposeClick(
        event: MotionEvent,
        compose: AbstractComposeView,
    ): Boolean {
        val androidCompose = findViewRootForTest(compose) ?: return false
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

    /**
     * [AbstractComposeView] is a [ViewGroup] container; the actual Compose root view that
     * implements [ViewRootForTest] (and exposes [ViewRootForTest.semanticsOwner] via the public
     * `RootForTest` interface) is added as a child once content is set. Walk the view subtree to
     * find it rather than assuming [compose] itself implements the interface.
     */
    private fun findViewRootForTest(view: View): ViewRootForTest? {
        if (view is ViewRootForTest) {
            return view
        }
        if (view !is ViewGroup) {
            return null
        }
        for (index in 0 until view.childCount) {
            val found = findViewRootForTest(view.getChildAt(index))
            if (found != null) {
                return found
            }
        }
        return null
    }
}

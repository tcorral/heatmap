package com.heatmap.sdk.view

import android.app.Activity
import android.view.MotionEvent
import android.view.View
import android.view.ViewGroup
import androidx.lifecycle.DefaultLifecycleObserver
import androidx.lifecycle.LifecycleOwner
import androidx.lifecycle.ProcessLifecycleOwner
import com.heatmap.sdk.capture.view.ViewActivationPayloadBuilder

/**
 * Wires View touch capture and process lifecycle (background / foreground) to a [HeatmapViewCaptureBackend].
 */
class HeatmapViewCapture private constructor(
    private val backend: com.heatmap.sdk.capture.view.HeatmapViewCaptureBackend,
) {
    private var decorView: View? = null
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
            activity: Activity,
            backend: com.heatmap.sdk.capture.view.HeatmapViewCaptureBackend,
        ): HeatmapViewCapture {
            val capture = HeatmapViewCapture(backend)
            capture.install(activity)
            return capture
        }
    }

    fun install(activity: Activity) {
        uninstall()
        val decor = activity.window?.decorView ?: return
        decorView = decor
        decor.setOnTouchListener(::onDecorTouch)
        touchListenerAttached = true
        ProcessLifecycleOwner.get().lifecycle.addObserver(processObserver)
    }

    fun uninstall() {
        if (touchListenerAttached) {
            decorView?.setOnTouchListener(null)
        }
        touchListenerAttached = false
        decorView = null
        ProcessLifecycleOwner.get().lifecycle.removeObserver(processObserver)
    }

    private fun onDecorTouch(view: View, event: MotionEvent): Boolean {
        if (event.actionMasked != MotionEvent.ACTION_UP) {
            return false
        }
        val root = decorView ?: return false
        val windowWidth = root.width.toFloat()
        val windowHeight = root.height.toFloat()
        if (windowWidth <= 0f || windowHeight <= 0f) {
            return false
        }
        val hitView = findViewAt(root, event.rawX, event.rawY) ?: return false
        val payload =
            ViewActivationPayloadBuilder.buildClick(
                hitView = captureNodeFor(hitView),
                locationInWindowX = event.x,
                locationInWindowY = event.y,
                windowWidth = windowWidth,
                windowHeight = windowHeight,
                timestampMs = System.currentTimeMillis(),
            ) ?: return false
        backend.recordActivationClick(payload)
        return false
    }

    internal fun findViewAt(root: View, rawX: Float, rawY: Float): View? {
        val location = IntArray(2)
        root.getLocationOnScreen(location)
        val localX = rawX - location[0]
        val localY = rawY - location[1]
        return findViewAtLocal(root, localX, localY)
    }

    private fun findViewAtLocal(view: View, x: Float, y: Float): View? {
        if (view.visibility != View.VISIBLE) {
            return null
        }
        if (x < 0 || y < 0 || x > view.width || y > view.height) {
            return null
        }
        if (view is ViewGroup) {
            for (index in view.childCount - 1 downTo 0) {
                val child = view.getChildAt(index)
                val childHit = findViewAtLocal(child, x - child.left, y - child.top)
                if (childHit != null) {
                    return childHit
                }
            }
        }
        return view
    }
}

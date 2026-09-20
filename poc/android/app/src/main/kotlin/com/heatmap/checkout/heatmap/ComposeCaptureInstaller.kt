package com.heatmap.checkout.heatmap

import android.view.View
import android.view.ViewGroup
import androidx.compose.ui.platform.AbstractComposeView
import com.heatmap.sdk.compose.HeatmapComposeCapture
import com.heatmap.sdk.view.HeatmapAndroidRustCaptureBackend

object ComposeCaptureInstaller {
    fun installWhenReady(
        root: View,
        backend: HeatmapAndroidRustCaptureBackend,
        onInstalled: (HeatmapComposeCapture) -> Unit,
    ) {
        root.viewTreeObserver.addOnGlobalLayoutListener(
            object : android.view.ViewTreeObserver.OnGlobalLayoutListener {
                override fun onGlobalLayout() {
                    val composeView = findComposeView(root) ?: return
                    root.viewTreeObserver.removeOnGlobalLayoutListener(this)
                    onInstalled(HeatmapComposeCapture.install(composeView, backend))
                }
            },
        )
    }

    private fun findComposeView(view: View): AbstractComposeView? {
        if (view is AbstractComposeView) {
            return view
        }
        if (view is ViewGroup) {
            for (index in 0 until view.childCount) {
                val found = findComposeView(view.getChildAt(index))
                if (found != null) {
                    return found
                }
            }
        }
        return null
    }
}

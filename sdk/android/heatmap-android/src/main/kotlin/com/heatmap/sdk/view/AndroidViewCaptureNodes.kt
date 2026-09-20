package com.heatmap.sdk.view

import android.view.View
import android.widget.EditText
import com.heatmap.sdk.capture.view.TextInputCaptureNode
import com.heatmap.sdk.capture.view.ViewCaptureNode

internal class AndroidViewCaptureNode(
    private val view: View,
) : ViewCaptureNode {
    override fun dedicatedHeatmapId(): String? = HeatmapViewIdentity.readDedicatedHeatmapId(view)

    override fun captureParent(): ViewCaptureNode? {
        val parent = view.parent
        return if (parent is View) AndroidViewCaptureNode(parent) else null
    }
}

internal class AndroidTextInputCaptureNode(
    private val field: EditText,
) : TextInputCaptureNode {
    override fun dedicatedHeatmapId(): String? = HeatmapViewIdentity.readDedicatedHeatmapId(field)

    override fun captureParent(): ViewCaptureNode? {
        val parent = field.parent
        return if (parent is View) AndroidViewCaptureNode(parent) else null
    }

    override fun inputTypeFlags(): Int = field.inputType

    override fun autofillHints(): List<String>? {
        val hints = field.autofillHints ?: return null
        return hints.toList()
    }
}

internal fun captureNodeFor(view: View): ViewCaptureNode {
    return if (view is EditText) {
        AndroidTextInputCaptureNode(view)
    } else {
        AndroidViewCaptureNode(view)
    }
}

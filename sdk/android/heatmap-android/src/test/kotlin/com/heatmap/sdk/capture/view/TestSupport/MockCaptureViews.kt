package com.heatmap.sdk.capture.view.TestSupport

import com.heatmap.sdk.capture.view.TextInputCaptureNode
import com.heatmap.sdk.capture.view.ViewCaptureNode

class MockCaptureView(
    private val heatmapId: String? = null,
    private val resourceEntryName: String? = null,
    private val contentDescription: String? = null,
    private val parent: ViewCaptureNode? = null,
    private val onForbiddenRead: ((String) -> Unit)? = null,
) : ViewCaptureNode {
    override fun dedicatedHeatmapId(): String? = heatmapId

    @Suppress("unused")
    fun resourceEntryNameForTest(): String? {
        onForbiddenRead?.invoke("resourceEntryName")
        return resourceEntryName
    }

    @Suppress("unused")
    fun contentDescriptionForTest(): String? {
        onForbiddenRead?.invoke("contentDescription")
        return contentDescription
    }

    override fun captureParent(): ViewCaptureNode? = parent
}

class MockTextInput(
    private val heatmapId: String? = null,
    private val inputType: Int,
    private val autofillHints: List<String>? = null,
    private val parent: ViewCaptureNode? = null,
    private val onTextRead: (() -> Unit)? = null,
) : TextInputCaptureNode {
    override fun dedicatedHeatmapId(): String? = heatmapId

    override fun captureParent(): ViewCaptureNode? = parent

    override fun inputTypeFlags(): Int {
        onTextRead?.invoke()
        return inputType
    }

    override fun autofillHints(): List<String>? = autofillHints
}

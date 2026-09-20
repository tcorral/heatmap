package com.heatmap.sdk.capture.compose.TestSupport

import com.heatmap.sdk.capture.compose.ComposeCaptureNode
import com.heatmap.sdk.capture.compose.ComposeTextFieldCaptureNode

class MockComposeNode(
    private val heatmapId: String? = null,
    private val testTag: String? = null,
    private val textValue: String? = null,
    private val passwordSemantics: Boolean = false,
    private val passwordVisualTransformation: Boolean = false,
    private val parent: ComposeCaptureNode? = null,
    private val onTextRead: (() -> Unit)? = null,
) : ComposeCaptureNode {
    override fun dedicatedHeatmapId(): String? = heatmapId

    override fun testTag(): String? = testTag

    override fun captureParent(): ComposeCaptureNode? = parent

    @Suppress("unused")
    fun enteredTextForTest(): String? {
        onTextRead?.invoke()
        return textValue
    }
}

class MockComposeTextField(
    private val heatmapId: String? = null,
    private val testTag: String? = null,
    private val passwordSemantics: Boolean = false,
    private val passwordVisualTransformation: Boolean = false,
    private val parent: ComposeCaptureNode? = null,
    private val onTextRead: (() -> Unit)? = null,
) : ComposeTextFieldCaptureNode {
    override fun dedicatedHeatmapId(): String? = heatmapId

    override fun testTag(): String? = testTag

    override fun captureParent(): ComposeCaptureNode? = parent

    override fun hasPasswordSemantics(): Boolean = passwordSemantics

    override fun usesPasswordVisualTransformation(): Boolean = passwordVisualTransformation

    @Suppress("unused")
    fun enteredTextForTest(): String? {
        onTextRead?.invoke()
        return "secret"
    }
}

package com.heatmap.sdk.compose

import androidx.compose.ui.semantics.SemanticsNode
import androidx.compose.ui.semantics.SemanticsProperties
import androidx.compose.ui.semantics.getOrNull
import com.heatmap.sdk.capture.compose.ComposeCaptureNode
import com.heatmap.sdk.capture.compose.ComposeTextFieldCaptureNode

internal class AndroidComposeCaptureNode(
    private val node: SemanticsNode,
) : ComposeCaptureNode {
    override fun dedicatedHeatmapId(): String? = HeatmapComposeIdentity.readDedicatedHeatmapId(node.config)

    override fun testTag(): String? = node.config.getOrNull(SemanticsProperties.TestTag)

    override fun captureParent(): ComposeCaptureNode? {
        val parent = node.parent ?: return null
        return captureNodeFor(parent)
    }
}

internal class AndroidComposeTextFieldCaptureNode(
    private val node: SemanticsNode,
) : ComposeTextFieldCaptureNode {
    override fun dedicatedHeatmapId(): String? = HeatmapComposeIdentity.readDedicatedHeatmapId(node.config)

    override fun testTag(): String? = node.config.getOrNull(SemanticsProperties.TestTag)

    override fun captureParent(): ComposeCaptureNode? {
        val parent = node.parent ?: return null
        return captureNodeFor(parent)
    }

    override fun hasPasswordSemantics(): Boolean = node.config.contains(SemanticsProperties.Password)

    override fun usesPasswordVisualTransformation(): Boolean = node.config.contains(SemanticsProperties.Password)
}

internal fun captureNodeFor(node: SemanticsNode): ComposeCaptureNode {
    return if (isComposeTextField(node)) {
        AndroidComposeTextFieldCaptureNode(node)
    } else {
        AndroidComposeCaptureNode(node)
    }
}

private fun isComposeTextField(node: SemanticsNode): Boolean {
    return node.config.contains(SemanticsProperties.EditableText) ||
        node.config.contains(SemanticsProperties.SetText)
}

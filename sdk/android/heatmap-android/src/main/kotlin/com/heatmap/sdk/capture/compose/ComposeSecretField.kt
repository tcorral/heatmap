package com.heatmap.sdk.capture.compose

object ComposeSecretField {
    fun isSecretInteractionTarget(hitNode: ComposeCaptureNode?): Boolean {
        val field = resolveTextField(hitNode) ?: return false
        return field.hasPasswordSemantics() || field.usesPasswordVisualTransformation()
    }

    private fun resolveTextField(hitNode: ComposeCaptureNode?): ComposeTextFieldCaptureNode? {
        var current = hitNode
        while (current != null) {
            if (current is ComposeTextFieldCaptureNode) {
                return current
            }
            current = current.captureParent()
        }
        return null
    }
}

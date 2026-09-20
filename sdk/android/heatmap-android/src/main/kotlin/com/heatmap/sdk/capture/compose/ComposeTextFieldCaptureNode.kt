package com.heatmap.sdk.capture.compose

/** Text-field metadata for secret-field detection; adapters must not read entered text. */
interface ComposeTextFieldCaptureNode : ComposeCaptureNode {
    fun hasPasswordSemantics(): Boolean

    fun usesPasswordVisualTransformation(): Boolean
}

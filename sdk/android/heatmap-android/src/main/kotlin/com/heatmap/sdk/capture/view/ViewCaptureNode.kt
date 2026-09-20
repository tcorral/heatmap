package com.heatmap.sdk.capture.view

/** View hierarchy node for capture logic (real [android.view.View] or test fakes). */
interface ViewCaptureNode {
    fun dedicatedHeatmapId(): String?

    fun captureParent(): ViewCaptureNode?
}

/** Text input metadata for secret-field detection; adapters must not read entered text. */
interface TextInputCaptureNode : ViewCaptureNode {
    fun inputTypeFlags(): Int

    fun autofillHints(): List<String>?
}

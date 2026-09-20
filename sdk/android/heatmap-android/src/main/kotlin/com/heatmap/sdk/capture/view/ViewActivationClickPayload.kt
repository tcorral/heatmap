package com.heatmap.sdk.capture.view

data class ViewActivationClickPayload(
    val elementId: String?,
    val position: NormalizedWindowPosition,
    val viewportWidth: UInt,
    val viewportHeight: UInt,
    val timestampMs: Long,
)

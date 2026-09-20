package com.heatmap.sdk.capture.compose

/** Semantics-tree node for Compose capture logic (real semantics node or test fakes). */
interface ComposeCaptureNode {
    fun dedicatedHeatmapId(): String?

    fun testTag(): String?

    fun captureParent(): ComposeCaptureNode?
}

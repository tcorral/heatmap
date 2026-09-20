package com.heatmap.sdk.capture.view

object ViewTapIdentity {
    /** Stable heatmap identity from the dedicated tag on the hit view or an ancestor. */
    fun elementId(hitView: ViewCaptureNode?): String? {
        var current = hitView
        while (current != null) {
            val raw = current.dedicatedHeatmapId()
            if (raw != null) {
                val trimmed = raw.trim()
                if (trimmed.isNotEmpty()) {
                    return trimmed
                }
            }
            current = current.captureParent()
        }
        return null
    }
}

package com.heatmap.sdk.capture.compose

object ComposeTapIdentity {
    /** Stable heatmap identity: dedicated semantics id on the hit node or an ancestor, then test tag. */
    fun elementId(hitNode: ComposeCaptureNode?): String? {
        val dedicated = firstNonBlankDedicated(hitNode)
        if (dedicated != null) {
            return dedicated
        }
        return firstNonBlankTestTag(hitNode)
    }

    private fun firstNonBlankDedicated(start: ComposeCaptureNode?): String? {
        var current = start
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

    private fun firstNonBlankTestTag(start: ComposeCaptureNode?): String? {
        var current = start
        while (current != null) {
            val raw = current.testTag()
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

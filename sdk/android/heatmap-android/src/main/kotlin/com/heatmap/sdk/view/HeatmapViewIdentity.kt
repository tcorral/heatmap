package com.heatmap.sdk.view

import android.view.View
import com.heatmap.sdk.R

/**
 * Dedicated View heatmap identity (not resource name, content description, or generic tag).
 *
 * Integrators set [setHeatmapId] in code or bind `app:heatmapId` via their own inflation hook.
 */
object HeatmapViewIdentity {
    @JvmStatic
    fun setHeatmapId(view: View, elementId: String) {
        view.setTag(R.id.heatmap_sdk_view_identity, elementId)
    }

    internal fun readDedicatedHeatmapId(view: View): String? {
        val tagged = view.getTag(R.id.heatmap_sdk_view_identity) as? String ?: return null
        val trimmed = tagged.trim()
        return trimmed.ifEmpty { null }
    }
}

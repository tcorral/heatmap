package com.heatmap.sdk.compose

import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.SemanticsPropertyKey
import androidx.compose.ui.semantics.getOrNull
import androidx.compose.ui.semantics.semantics

/**
 * Dedicated Compose heatmap identity (not label text, content description, or generated paths).
 *
 * When absent on the hit composable or an ancestor, the Compose adapter falls back to [androidx.compose.ui.platform.testTag].
 */
object HeatmapComposeIdentity {
    val HeatmapIdSemanticsKey: SemanticsPropertyKey<String> =
        SemanticsPropertyKey(
            name = "HeatmapId",
            mergePolicy = { parent, child -> child ?: parent },
        )

    internal fun readDedicatedHeatmapId(properties: androidx.compose.ui.semantics.SemanticsConfiguration): String? {
        val tagged = properties.getOrNull(HeatmapIdSemanticsKey) ?: return null
        val trimmed = tagged.trim()
        return trimmed.ifEmpty { null }
    }
}

fun Modifier.heatmapId(elementId: String): Modifier =
    semantics {
        set(HeatmapComposeIdentity.HeatmapIdSemanticsKey, elementId)
    }

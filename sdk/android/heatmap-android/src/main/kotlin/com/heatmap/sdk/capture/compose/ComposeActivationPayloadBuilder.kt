package com.heatmap.sdk.capture.compose

import com.heatmap.sdk.capture.view.ViewActivationClickPayload
import com.heatmap.sdk.capture.view.WindowGeometry

object ComposeActivationPayloadBuilder {
    fun buildClick(
        hitNode: ComposeCaptureNode?,
        locationInWindowX: Float,
        locationInWindowY: Float,
        windowWidth: Float,
        windowHeight: Float,
        timestampMs: Long,
    ): ViewActivationClickPayload? {
        if (ComposeSecretField.isSecretInteractionTarget(hitNode)) {
            return null
        }
        val position =
            WindowGeometry.normalizeWindowPosition(
                locationInWindowX,
                locationInWindowY,
                windowWidth,
                windowHeight,
            ) ?: return null
        val width = windowWidth.toInt().coerceAtLeast(0)
        val height = windowHeight.toInt().coerceAtLeast(0)
        if (width == 0 || height == 0) {
            return null
        }
        val elementId = ComposeTapIdentity.elementId(hitNode)
        return ViewActivationClickPayload(
            elementId = elementId,
            position = position,
            viewportWidth = width.toUInt(),
            viewportHeight = height.toUInt(),
            timestampMs = timestampMs,
        )
    }
}

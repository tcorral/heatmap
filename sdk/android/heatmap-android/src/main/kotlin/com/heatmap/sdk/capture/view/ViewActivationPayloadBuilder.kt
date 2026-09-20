package com.heatmap.sdk.capture.view

object ViewActivationPayloadBuilder {
    fun buildClick(
        hitView: ViewCaptureNode?,
        locationInWindowX: Float,
        locationInWindowY: Float,
        windowWidth: Float,
        windowHeight: Float,
        timestampMs: Long,
    ): ViewActivationClickPayload? {
        if (ViewSecretField.isSecretInteractionTarget(hitView)) {
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
        val elementId = ViewTapIdentity.elementId(hitView)
        return ViewActivationClickPayload(
            elementId = elementId,
            position = position,
            viewportWidth = width.toUInt(),
            viewportHeight = height.toUInt(),
            timestampMs = timestampMs,
        )
    }
}

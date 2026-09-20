package com.heatmap.sdk.capture.view

data class NormalizedWindowPosition(
    val x: Double,
    val y: Double,
)

object WindowGeometry {
    fun normalizeWindowPosition(
        locationInWindowX: Float,
        locationInWindowY: Float,
        windowWidth: Float,
        windowHeight: Float,
    ): NormalizedWindowPosition? {
        if (windowWidth <= 0f || windowHeight <= 0f) {
            return null
        }
        val x = clampFraction((locationInWindowX / windowWidth).toDouble())
        val y = clampFraction((locationInWindowY / windowHeight).toDouble())
        return NormalizedWindowPosition(x = x, y = y)
    }

    private fun clampFraction(value: Double): Double =
        when {
            value < 0.0 -> 0.0
            value > 1.0 -> 1.0
            else -> value
        }
}

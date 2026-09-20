package com.heatmap.sdk.capture.view

interface HeatmapViewCaptureBackend {
    fun recordActivationClick(payload: ViewActivationClickPayload)

    fun recordScreenView(viewportWidth: UInt, viewportHeight: UInt)

    fun onBackground()

    fun onForeground()
}

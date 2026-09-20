package com.heatmap.sdk.capture.view

interface HeatmapViewCaptureBackend {
    fun recordActivationClick(payload: ViewActivationClickPayload)

    fun onBackground()

    fun onForeground()
}

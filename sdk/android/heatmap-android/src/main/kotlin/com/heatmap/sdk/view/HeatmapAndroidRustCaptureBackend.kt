package com.heatmap.sdk.view

import com.heatmap.sdk.HeatmapSdk
import com.heatmap.sdk.capture.view.HeatmapViewCaptureBackend
import com.heatmap.sdk.capture.view.ViewActivationClickPayload

/** Forwards View adapter payloads to UniFFI [HeatmapSdk] (not exposed on [HeatmapClient]). */
class HeatmapAndroidRustCaptureBackend(
    private val sdk: HeatmapSdk,
    private val appVersion: String,
    private val osVersion: String = android.os.Build.VERSION.RELEASE?.let { "Android $it" } ?: "Android",
    private val runOffUi: (() -> Unit) -> Unit = { it() },
) : HeatmapViewCaptureBackend {

    constructor(
        sdk: HeatmapSdk,
    ) : this(
        sdk = sdk,
        appVersion = "0.0.0",
    )

    override fun recordActivationClick(payload: ViewActivationClickPayload) {
        runOffUi {
            sdk.recordAndroidActivationClick(
                x = payload.position.x,
                y = payload.position.y,
                elementId = payload.elementId,
                viewportWidth = payload.viewportWidth,
                viewportHeight = payload.viewportHeight,
                timestampMs = payload.timestampMs,
                osVersion = osVersion,
                appVersion = appVersion,
            )
        }
    }

    override fun recordScreenView(viewportWidth: UInt, viewportHeight: UInt) {
        runOffUi {
            sdk.recordAndroidScreenView(
                viewportWidth = viewportWidth,
                viewportHeight = viewportHeight,
                osVersion = osVersion,
                appVersion = appVersion,
            )
        }
    }

    override fun recordScroll(viewportWidth: UInt, viewportHeight: UInt) {
        runOffUi {
            sdk.recordAndroidScroll(
                viewportWidth = viewportWidth,
                viewportHeight = viewportHeight,
                osVersion = osVersion,
                appVersion = appVersion,
            )
        }
    }

    override fun onBackground() {
        runOffUi { sdk.onBackground() }
    }

    override fun onForeground() {
        runOffUi { sdk.onForeground() }
    }
}

package com.heatmap.checkout.heatmap

import com.heatmap.checkout.BuildConfig
import com.heatmap.sdk.HeatmapClient
import com.heatmap.sdk.HeatmapSdk
import com.heatmap.sdk.InitConfig
import com.heatmap.sdk.view.HeatmapAndroidRustCaptureBackend

data class HeatmapRuntime(
    val client: HeatmapClient,
    val captureBackend: HeatmapAndroidRustCaptureBackend,
)

object HeatmapBootstrap {
    fun makeRuntime(config: HeatmapClientConfig): HeatmapRuntime {
        val initConfig =
            InitConfig(
                projectKey = config.projectKey,
                endpoint = config.endpoint,
                sessionTimeoutMs = 1_800_000u,
                sampleRate = 1.0,
                excludeElements = emptyList(),
                excludeScreens = emptyList(),
                maskElements = emptyList(),
            )
        val sdk = HeatmapSdk.init(initConfig)
        val client = HeatmapClient.fromSdk(sdk)
        val backend =
            HeatmapAndroidRustCaptureBackend(
                sdk = sdk,
                appVersion = BuildConfig.VERSION_NAME,
            )
        return HeatmapRuntime(client = client, captureBackend = backend)
    }
}

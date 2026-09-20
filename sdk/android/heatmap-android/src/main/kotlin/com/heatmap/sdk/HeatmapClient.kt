package com.heatmap.sdk

import java.io.File

/**
 * App-facing Heatmap SDK client. Wraps UniFFI [HeatmapSdk] with a stable Kotlin API.
 *
 * Integrators call [init], [identify], [signOut], [screen], [setTrackingEnabled], and [flush] only.
 * Activation capture (wire type [com.heatmap.sdk.internal.WireActivationType.CLICK]) is adapter-owned.
 */
class HeatmapClient private constructor(
    private val sdk: HeatmapSdk,
) : AutoCloseable {
    private val flushScheduler =
        HeatmapFlushScheduler {
            try {
                sdk.pollFlush()
            } catch (_: Exception) {
            }
        }

    /** Initialize from JSON matching the core `HeatmapConfig` shape. */
    constructor(configJson: String) : this(HeatmapSdk.initFromJson(configJson))

    /** Initialize with required fields and core defaults for optional settings. */
    constructor(
        projectKey: String,
        endpoint: String,
    ) : this(
        HeatmapSdk.init(
            InitConfig(
                projectKey = projectKey,
                endpoint = endpoint,
                sessionTimeoutMs = 1_800_000,
                sampleRate = 1.0,
                excludeElements = emptyList(),
                excludeScreens = emptyList(),
                maskElements = emptyList(),
                storageDirectory = File(System.getProperty("java.io.tmpdir"), "heatmap-sdk").absolutePath,
                appVersion = "1.0.0",
            ),
        ),
    )

    /** Share one [HeatmapSdk] with View/Compose capture backends. */
    companion object {
        @JvmStatic
        fun fromSdk(sdk: HeatmapSdk): HeatmapClient = HeatmapClient(sdk)
    }

    fun identify(userId: String) {
        sdk.identify(userId)
    }

    fun signOut() {
        sdk.signOut()
    }

    fun screen(screenId: String, layoutVersion: String? = null) {
        sdk.screen(screenId, layoutVersion)
    }

    fun setTrackingEnabled(enabled: Boolean) {
        sdk.setTrackingEnabled(enabled)
    }

    fun flush() {
        flushScheduler.perform { sdk.flush() }
    }

    override fun close() {
        flushScheduler.close()
        sdk.close()
    }
}

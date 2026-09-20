package com.heatmap.sdk.view

import com.heatmap.sdk.HeatmapSdk
import com.heatmap.sdk.InitConfig
import com.heatmap.sdk.capture.view.NormalizedWindowPosition
import com.heatmap.sdk.capture.view.ViewActivationClickPayload
import org.junit.Assume.assumeTrue
import org.junit.Test

class AndroidSessionIntegrationTest {
    @Test
    fun backgroundThenForegroundKeepsSessionBeforeTimeout() {
        val sdk = tryCreateSdk() ?: return
        val backend =
            HeatmapAndroidRustCaptureBackend(
                sdk = sdk,
                osVersion = "Android 14",
                appVersion = "1.0.0",
            )
        backend.recordActivationClick(
            ViewActivationClickPayload(
                elementId = "checkout.pay",
                position = NormalizedWindowPosition(0.5, 0.5),
                viewportWidth = 800u,
                viewportHeight = 600u,
                timestampMs = 100L,
            ),
        )
        val firstSessionIds = sdk.adapterClickSessionIdsFromBatch()
        check(firstSessionIds.size == 1)
        val sessionId = firstSessionIds[0]

        backend.onBackground()
        backend.onForeground()
        backend.recordActivationClick(
            ViewActivationClickPayload(
                elementId = null,
                position = NormalizedWindowPosition(0.25, 0.5),
                viewportWidth = 800u,
                viewportHeight = 600u,
                timestampMs = 200L,
            ),
        )
        val secondSessionIds = sdk.adapterClickSessionIdsFromBatch()
        check(secondSessionIds.size == 1)
        check(secondSessionIds[0] == sessionId)
        sdk.close()
    }

    private fun tryCreateSdk(): HeatmapSdk? {
        return try {
            HeatmapSdk.init(
                InitConfig(
                    projectKey = "pk_test",
                    endpoint = "https://ingest.example.com",
                    sessionTimeoutMs = 10_000u,
                    sampleRate = 1.0,
                    excludeElements = emptyList(),
                    excludeScreens = emptyList(),
                    maskElements = emptyList(),
                ),
            )
        } catch (e: UnsatisfiedLinkError) {
            assumeTrue(
                "libheatmap_ffi not loaded; build host dylib and set uniffi.component.heatmap_ffi.libraryOverride",
                false,
            )
            null
        } catch (e: ExceptionInInitializerError) {
            if (e.cause is UnsatisfiedLinkError) {
                assumeTrue(
                    "libheatmap_ffi not loaded; build host dylib and set uniffi.component.heatmap_ffi.libraryOverride",
                    false,
                )
                null
            } else {
                throw e
            }
        }
    }
}

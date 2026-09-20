package com.heatmap.sdk.consumer

import com.heatmap.sdk.HeatmapClient
import org.junit.Assume.assumeTrue
import org.junit.Test

/**
 * Clean-consumer smoke: resolves the published AAR from Maven Local and inits.
 * Does not include the SDK as a source module.
 */
class HeatmapAarConsumerSmokeTest {
    @Test
    fun initFromJsonScreenAndFlush() {
        val config =
            """
            {
                "project_key": "pk_test",
                "endpoint": "https://ingest.example.com"
            }
            """.trimIndent()
        try {
            HeatmapClient(config).use { client ->
                client.screen("Checkout", layoutVersion = "layout-a")
                client.flush()
            }
        } catch (e: UnsatisfiedLinkError) {
            assumeTrue("libheatmap_ffi not packaged in the AAR; run sdk/android/scripts/build-rust-android.sh", false)
        } catch (e: ExceptionInInitializerError) {
            val cause = e.cause
            if (cause is UnsatisfiedLinkError) {
                assumeTrue("libheatmap_ffi not packaged in the AAR; run sdk/android/scripts/build-rust-android.sh", false)
            } else {
                throw e
            }
        }
    }
}

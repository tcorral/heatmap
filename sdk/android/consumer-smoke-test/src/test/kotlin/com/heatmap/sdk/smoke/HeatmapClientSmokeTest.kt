package com.heatmap.sdk.smoke

import com.heatmap.sdk.HeatmapClient
import org.junit.Assume.assumeTrue
import org.junit.Test

/**
 * Consumer-style smoke test: depends on the `:heatmap-android` AAR artifact wiring
 * (project dependency in this repo) and exercises the public API end-to-end when native libs load.
 */
class HeatmapClientSmokeTest {
    @Test
    fun initFromJsonScreenIdentifySignOutAndFlush() {
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
                client.identify("user-smoke")
                client.signOut()
                client.setTrackingEnabled(false)
                client.setTrackingEnabled(true)
                client.flush()
            }
        } catch (e: UnsatisfiedLinkError) {
            assumeTrue(
                "libheatmap_ffi not loaded; run scripts/build-rust-android.sh and copy .so into jniLibs",
                false,
            )
        } catch (e: ExceptionInInitializerError) {
            val cause = e.cause
            if (cause is UnsatisfiedLinkError) {
                assumeTrue(
                    "libheatmap_ffi not loaded; run scripts/build-rust-android.sh and copy .so into jniLibs",
                    false,
                )
            } else {
                throw e
            }
        }
    }

    @Test
    fun structuredInitConstructor() {
        try {
            HeatmapClient(
                projectKey = "pk_test",
                endpoint = "https://ingest.example.com",
            ).use { client ->
                client.screen("Checkout")
                client.flush()
            }
        } catch (e: UnsatisfiedLinkError) {
            assumeTrue(
                "libheatmap_ffi not loaded; run scripts/build-rust-android.sh and copy .so into jniLibs",
                false,
            )
        } catch (e: ExceptionInInitializerError) {
            val cause = e.cause
            if (cause is UnsatisfiedLinkError) {
                assumeTrue(
                    "libheatmap_ffi not loaded; run scripts/build-rust-android.sh and copy .so into jniLibs",
                    false,
                )
            } else {
                throw e
            }
        }
    }
}

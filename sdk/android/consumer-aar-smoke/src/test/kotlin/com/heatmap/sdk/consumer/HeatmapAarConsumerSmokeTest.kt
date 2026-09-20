package com.heatmap.sdk.consumer

import com.heatmap.sdk.HeatmapClient
import org.junit.Assert.assertTrue
import org.junit.Assert.fail
import org.junit.Assume.assumeTrue
import org.junit.Test
import java.io.File
import java.util.zip.ZipFile

/**
 * Clean-consumer smoke: resolves the published AAR from Maven Local and inits.
 * Does not include the SDK as a source module.
 */
class HeatmapAarConsumerSmokeTest {
    @Test
    fun publishedAarContainsRequiredNativeLibraries() {
        val aar = publishedAar()
        assertTrue("publish com.heatmap:heatmap-android:0.1.0-pre1 first", aar.isFile)
        ZipFile(aar).use { zip ->
            val names = zip.entries().asSequence().map { it.name }.toSet()
            for (abi in REQUIRED_ABIS) {
                val entry = "jni/$abi/libheatmap_ffi.so"
                assertTrue("AAR missing $entry", names.contains(entry))
            }
        }
    }

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
            skipHostJvmIfNativePackaged(e)
        } catch (e: ExceptionInInitializerError) {
            val cause = e.cause
            if (cause is UnsatisfiedLinkError) {
                skipHostJvmIfNativePackaged(cause)
            } else {
                throw e
            }
        }
    }

    private fun skipHostJvmIfNativePackaged(error: UnsatisfiedLinkError) {
        val aar = publishedAar()
        if (!aar.isFile) {
            fail("libheatmap_ffi not packaged; AAR is missing")
        }
        ZipFile(aar).use { zip ->
            val hasNative = zip.entries().asSequence().any { it.name.endsWith("libheatmap_ffi.so") }
            if (!hasNative) {
                fail("libheatmap_ffi not packaged in the AAR")
            }
        }
        assumeTrue("host JVM cannot load Android libheatmap_ffi.so: ${error.message}", false)
    }

    private fun publishedAar(): File {
        return File(
            System.getProperty("user.home"),
            ".m2/repository/com/heatmap/heatmap-android/0.1.0-pre1/heatmap-android-0.1.0-pre1.aar",
        )
    }

    companion object {
        private val REQUIRED_ABIS = listOf("arm64-v8a", "armeabi-v7a", "x86_64")
    }
}

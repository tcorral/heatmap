package com.heatmap.sdk.view

import com.heatmap.sdk.HeatmapSdk
import com.heatmap.sdk.InitConfig
import com.heatmap.sdk.capture.view.NormalizedWindowPosition
import com.heatmap.sdk.capture.view.ViewActivationClickPayload
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Assume.assumeTrue
import org.junit.Test

/**
 * True only when [error]'s chain proves the JNA native dispatch library (`libjnidispatch`)
 * could not be loaded in this JVM (expected outside a device/emulator; see
 * [AndroidSessionIntegrationTest]'s assumeTrue message). Matches on that specific library
 * name rather than any [UnsatisfiedLinkError]/[ExceptionInInitializerError], so an unrelated
 * native-symbol or ABI mismatch (e.g. `UnsatisfiedLinkError: no implementation found for
 * native method ...`) is rethrown and fails the test instead of being silently skipped.
 *
 * Walks [Throwable.cause] first: this correctly links the first failing test's raw
 * [UnsatisfiedLinkError] and, for later tests in the same JVM worker, the
 * [NoClassDefFoundError] the JVM throws once a class's static init is known to have failed
 * (`Could not initialize class ...`) back to its wrapping [ExceptionInInitializerError].
 * Empirically, [ExceptionInInitializerError.cause] itself is not reliably populated in that
 * retry path, so as a final check its own message (which embeds the original exception's
 * `toString()`) is inspected for the `libjnidispatch` signature it wraps.
 */
internal fun isMissingJnaDispatchLibrary(error: Throwable): Boolean {
    var current: Throwable? = error
    while (current != null) {
        val matchesSignature = current.message?.contains("libjnidispatch") == true
        if (matchesSignature && (current is UnsatisfiedLinkError || current is ExceptionInInitializerError)) {
            return true
        }
        current = current.cause
    }
    return false
}

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

    @Test
    fun recordScreenViewAfterScreenDoesNotThrowWhenNativeLibLoaded() {
        val sdk = tryCreateSdk() ?: return
        val backend =
            HeatmapAndroidRustCaptureBackend(
                sdk = sdk,
                osVersion = "Android 14",
                appVersion = "1.0.0",
            )
        sdk.screen("Home", null)
        backend.recordScreenView(800u, 600u)
        sdk.close()
    }

    @Test
    fun recordScrollAfterScreenDoesNotThrowWhenNativeLibLoaded() {
        val sdk = tryCreateSdk() ?: return
        val backend =
            HeatmapAndroidRustCaptureBackend(
                sdk = sdk,
                osVersion = "Android 14",
                appVersion = "1.0.0",
            )
        sdk.screen("Checkout", "2024-09")
        backend.recordScroll(800u, 600u)
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
                    storageDirectory = java.io.File(
                        System.getProperty("java.io.tmpdir"),
                        java.util.UUID.randomUUID().toString(),
                    ).absolutePath,
                    appVersion = "1.0.0",
                ),
            )
        } catch (e: LinkageError) {
            if (!isMissingJnaDispatchLibrary(e)) {
                throw e
            }
            assumeTrue(
                "libheatmap_ffi not loaded; build host dylib and set uniffi.component.heatmap_ffi.libraryOverride",
                false,
            )
            null
        }
    }

    @Test
    fun isMissingJnaDispatchLibraryDetectsDirectUnsatisfiedLinkError() {
        val error =
            UnsatisfiedLinkError(
                "Native library (com/sun/jna/darwin-aarch64/libjnidispatch.jnilib) not found in resource path",
            )

        assertTrue(isMissingJnaDispatchLibrary(error))
    }

    @Test
    fun isMissingJnaDispatchLibraryDetectsWrappedExceptionInInitializerError() {
        val wrapped =
            ExceptionInInitializerError(
                "Exception java.lang.UnsatisfiedLinkError: " +
                    "Native library (com/sun/jna/darwin-aarch64/libjnidispatch.jnilib) not found in resource path",
            )
        val error = NoClassDefFoundError("Could not initialize class com.sun.jna.Native").apply { initCause(wrapped) }

        assertTrue(isMissingJnaDispatchLibrary(error))
    }

    @Test
    fun isMissingJnaDispatchLibraryRejectsUnrelatedUnsatisfiedLinkError() {
        val error = UnsatisfiedLinkError("no implementation found for native method com.example.Foo.bar()")

        assertFalse(isMissingJnaDispatchLibrary(error))
    }

    @Test
    fun isMissingJnaDispatchLibraryRejectsUnrelatedLinkageError() {
        val error = NoSuchMethodError("com.example.Foo.bar()V")

        assertFalse(isMissingJnaDispatchLibrary(error))
    }
}

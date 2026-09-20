package com.heatmap.sdk.capture.view

import android.text.InputType
import com.heatmap.sdk.capture.view.TestSupport.MockCaptureView
import com.heatmap.sdk.capture.view.TestSupport.MockTextInput
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.fail
import org.junit.Test

class ViewActivationPayloadTest {
    @Test
    fun payMarkedClickPayloadIncludesElementId() {
        val pay = MockCaptureView(heatmapId = "checkout.pay")
        val payload =
            ViewActivationPayloadBuilder.buildClick(
                hitView = pay,
                locationInWindowX = 120f,
                locationInWindowY = 80f,
                windowWidth = 800f,
                windowHeight = 600f,
                timestampMs = 1_726_771_205_000L,
            )
        requireNotNull(payload)
        assertEquals("checkout.pay", payload.elementId)
        assertEquals(0.15, payload.position.x, 0.0001)
        assertEquals(80.0 / 600.0, payload.position.y, 0.0001)
        assertEquals(800u, payload.viewportWidth)
        assertEquals(600u, payload.viewportHeight)
    }

    @Test
    fun unmarkedNonSecretClickPayloadHasCoordinatesOnly() {
        val cancel = MockCaptureView(heatmapId = null)
        val payload =
            ViewActivationPayloadBuilder.buildClick(
                hitView = cancel,
                locationInWindowX = 200f,
                locationInWindowY = 300f,
                windowWidth = 800f,
                windowHeight = 600f,
                timestampMs = 1L,
            )
        requireNotNull(payload)
        assertNull(payload.elementId)
        assertEquals(0.25, payload.position.x, 0.0001)
        assertEquals(0.5, payload.position.y, 0.0001)
    }

    @Test
    fun secretFieldClickProducesNoPayload() {
        val password =
            MockTextInput(
                inputType = InputType.TYPE_CLASS_TEXT or InputType.TYPE_TEXT_VARIATION_PASSWORD,
            )
        val payload =
            ViewActivationPayloadBuilder.buildClick(
                hitView = password,
                locationInWindowX = 40f,
                locationInWindowY = 40f,
                windowWidth = 800f,
                windowHeight = 600f,
                timestampMs = 1L,
            )
        assertNull(payload)
    }

    @Test
    fun builderDoesNotReadEnteredText() {
        val pay = MockCaptureView(heatmapId = "checkout.pay")
        val passwordField =
            MockTextInput(
                heatmapId = null,
                inputType = InputType.TYPE_CLASS_TEXT,
                onTextRead = { fail("adapter must not read text") },
            )
        ViewActivationPayloadBuilder.buildClick(
            hitView = pay,
            locationInWindowX = 10f,
            locationInWindowY = 10f,
            windowWidth = 100f,
            windowHeight = 100f,
            timestampMs = 1L,
        )
        ViewActivationPayloadBuilder.buildClick(
            hitView = passwordField,
            locationInWindowX = 10f,
            locationInWindowY = 10f,
            windowWidth = 100f,
            windowHeight = 100f,
            timestampMs = 1L,
        )
    }
}

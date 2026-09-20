package com.heatmap.sdk.capture.compose

import com.heatmap.sdk.capture.compose.TestSupport.MockComposeNode
import com.heatmap.sdk.capture.compose.TestSupport.MockComposeTextField
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.fail
import org.junit.Test

class ComposeActivationPayloadTest {
    @Test
    fun payMarkedClickPayloadIncludesElementId() {
        val pay = MockComposeNode(heatmapId = "checkout.pay")
        val payload =
            ComposeActivationPayloadBuilder.buildClick(
                hitNode = pay,
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
    fun payWithTestTagFallbackIncludesElementId() {
        val pay = MockComposeNode(testTag = "checkout.pay")
        val payload =
            ComposeActivationPayloadBuilder.buildClick(
                hitNode = pay,
                locationInWindowX = 400f,
                locationInWindowY = 300f,
                windowWidth = 800f,
                windowHeight = 600f,
                timestampMs = 2L,
            )
        requireNotNull(payload)
        assertEquals("checkout.pay", payload.elementId)
    }

    @Test
    fun unmarkedNonSecretClickPayloadHasCoordinatesOnly() {
        val cancel = MockComposeNode()
        val payload =
            ComposeActivationPayloadBuilder.buildClick(
                hitNode = cancel,
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
    fun passwordSemanticsFieldProducesNoPayload() {
        val password =
            MockComposeTextField(
                passwordSemantics = true,
            )
        val payload =
            ComposeActivationPayloadBuilder.buildClick(
                hitNode = password,
                locationInWindowX = 40f,
                locationInWindowY = 40f,
                windowWidth = 800f,
                windowHeight = 600f,
                timestampMs = 1L,
            )
        assertNull(payload)
    }

    @Test
    fun passwordVisualTransformationFieldProducesNoPayload() {
        val password =
            MockComposeTextField(
                passwordVisualTransformation = true,
            )
        val payload =
            ComposeActivationPayloadBuilder.buildClick(
                hitNode = password,
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
        val pay = MockComposeNode(heatmapId = "checkout.pay")
        val passwordField =
            MockComposeTextField(
                onTextRead = { fail("adapter must not read text") },
            )
        ComposeActivationPayloadBuilder.buildClick(
            hitNode = pay,
            locationInWindowX = 10f,
            locationInWindowY = 10f,
            windowWidth = 100f,
            windowHeight = 100f,
            timestampMs = 1L,
        )
        ComposeActivationPayloadBuilder.buildClick(
            hitNode = passwordField,
            locationInWindowX = 10f,
            locationInWindowY = 10f,
            windowWidth = 100f,
            windowHeight = 100f,
            timestampMs = 1L,
        )
    }
}

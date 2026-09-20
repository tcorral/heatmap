package com.heatmap.sdk.capture.compose

import com.heatmap.sdk.capture.compose.TestSupport.MockComposeNode
import com.heatmap.sdk.capture.compose.TestSupport.MockComposeTextField
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class ComposeSecretFieldTest {
    @Test
    fun passwordSemanticsDroppedWhenUnmarked() {
        val field = MockComposeTextField(passwordSemantics = true)
        assertTrue(ComposeSecretField.isSecretInteractionTarget(field))
    }

    @Test
    fun passwordVisualTransformationDroppedWhenUnmarked() {
        val field = MockComposeTextField(passwordVisualTransformation = true)
        assertTrue(ComposeSecretField.isSecretInteractionTarget(field))
    }

    @Test
    fun childInsidePasswordFieldIsDropped() {
        val field = MockComposeTextField(passwordSemantics = true)
        val icon = MockComposeNode(parent = field)
        assertTrue(ComposeSecretField.isSecretInteractionTarget(icon))
    }

    @Test
    fun markedNonSecretControlIsNotDropped() {
        val button = MockComposeNode(heatmapId = "checkout.pay")
        assertFalse(ComposeSecretField.isSecretInteractionTarget(button))
    }
}

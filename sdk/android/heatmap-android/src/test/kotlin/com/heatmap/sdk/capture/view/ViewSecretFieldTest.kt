package com.heatmap.sdk.capture.view

import android.text.InputType
import com.heatmap.sdk.capture.view.TestSupport.MockCaptureView
import com.heatmap.sdk.capture.view.TestSupport.MockTextInput
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class ViewSecretFieldTest {
    @Test
    fun passwordInputTypeDroppedWhenUnmarked() {
        val field =
            MockTextInput(
                inputType = InputType.TYPE_CLASS_TEXT or InputType.TYPE_TEXT_VARIATION_PASSWORD,
            )
        assertTrue(ViewSecretField.isSecretInteractionTarget(field))
    }

    @Test
    fun passwordAutofillHintDroppedWhenUnmarked() {
        val field =
            MockTextInput(
                inputType = InputType.TYPE_CLASS_TEXT,
                autofillHints = listOf("password"),
            )
        assertTrue(ViewSecretField.isSecretInteractionTarget(field))
    }

    @Test
    fun paymentAutofillHintsDroppedWhenUnmarked() {
        val hints =
            listOf(
                "credit_card_number",
                "credit_card_expiration_date",
                "credit_card_security_code",
                "new_password",
            )
        for (hint in hints) {
            val field =
                MockTextInput(
                    inputType = InputType.TYPE_CLASS_TEXT,
                    autofillHints = listOf(hint),
                )
            assertTrue(
                "expected secret for $hint",
                ViewSecretField.isSecretInteractionTarget(field),
            )
        }
    }

    @Test
    fun childInsidePasswordFieldIsDropped() {
        val field =
            MockTextInput(
                inputType = InputType.TYPE_CLASS_TEXT or InputType.TYPE_TEXT_VARIATION_PASSWORD,
            )
        val icon = MockCaptureView(parent = field)
        assertTrue(ViewSecretField.isSecretInteractionTarget(icon))
    }

    @Test
    fun markedNonSecretControlIsNotDropped() {
        val button = MockCaptureView(heatmapId = "checkout.pay")
        assertFalse(ViewSecretField.isSecretInteractionTarget(button))
    }
}

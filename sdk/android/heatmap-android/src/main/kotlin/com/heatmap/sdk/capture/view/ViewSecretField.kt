package com.heatmap.sdk.capture.view

import android.text.InputType

object ViewSecretField {
    fun isSecretInteractionTarget(hitView: ViewCaptureNode?): Boolean {
        val field = resolveTextInput(hitView) ?: return false
        return isSecretTextInput(field)
    }

    private fun resolveTextInput(hitView: ViewCaptureNode?): TextInputCaptureNode? {
        var current = hitView
        while (current != null) {
            if (current is TextInputCaptureNode) {
                return current
            }
            current = current.captureParent()
        }
        return null
    }

    private fun isSecretTextInput(field: TextInputCaptureNode): Boolean {
        if (isPasswordInputType(field.inputTypeFlags())) {
            return true
        }
        val hints = field.autofillHints() ?: return false
        return hints.any { hint -> isSecretAutofillHint(hint) }
    }

    internal fun isPasswordInputType(inputType: Int): Boolean {
        val variation = inputType and InputType.TYPE_MASK_VARIATION
        return variation == InputType.TYPE_TEXT_VARIATION_PASSWORD ||
            variation == InputType.TYPE_TEXT_VARIATION_VISIBLE_PASSWORD ||
            variation == InputType.TYPE_TEXT_VARIATION_WEB_PASSWORD ||
            variation == InputType.TYPE_NUMBER_VARIATION_PASSWORD
    }

    internal fun isSecretAutofillHint(hint: String): Boolean {
        return when (hint.trim().lowercase()) {
            "password",
            "new_password",
            "new-password",
            "credit_card_number",
            "credit_card_expiration_date",
            "credit_card_security_code",
            -> true
            else -> false
        }
    }
}

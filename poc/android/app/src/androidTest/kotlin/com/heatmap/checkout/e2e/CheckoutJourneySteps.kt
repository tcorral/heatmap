package com.heatmap.checkout.e2e

import androidx.compose.ui.test.junit4.AndroidComposeTestRule
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performTouchInput
import androidx.compose.ui.test.swipeUp
import androidx.test.ext.junit.rules.ActivityScenarioRule
import androidx.test.uiautomator.By
import androidx.test.uiautomator.Until
import com.heatmap.checkout.MainActivity

object CheckoutJourneySteps {
    fun runComposeCheckout(
        composeRule: AndroidComposeTestRule<ActivityScenarioRule<MainActivity>, MainActivity>,
    ) {
        composeRule.onNodeWithText("Browse products").performClick()
        composeRule.onNodeWithText("Activate product (Compose checkout)").performClick()
        completeComposeCheckoutFromCheckoutScreen(composeRule)
    }

    fun runViewCheckout(
        composeRule: AndroidComposeTestRule<ActivityScenarioRule<MainActivity>, MainActivity>,
    ) {
        composeRule.onNodeWithText("Browse products").performClick()
        composeRule.onNodeWithText("Activate product (View checkout)").performClick()
        completeViewCheckoutOnDevice()
    }

    fun completeComposeCheckoutFromCheckoutScreen(
        composeRule: AndroidComposeTestRule<ActivityScenarioRule<MainActivity>, MainActivity>,
        includePay: Boolean = true,
    ) {
        composeRule.waitForIdle()
        scrollComposeCheckoutUntilPayVisible(composeRule)
        composeRule.onNodeWithText("Apply coupon").performClick()
        if (includePay) {
            composeRule.onNodeWithText("Pay").performClick()
            composeRule.onNodeWithText("Confirmation").assertIsDisplayed()
        }
    }

    private fun scrollComposeCheckoutUntilPayVisible(
        composeRule: AndroidComposeTestRule<ActivityScenarioRule<MainActivity>, MainActivity>,
    ) {
        repeat(8) {
            val payVisible =
                runCatching {
                    composeRule.onNodeWithText("Pay").assertIsDisplayed()
                    true
                }.getOrDefault(false)
            if (payVisible) return
            composeRule.onNodeWithText("Checkout").performTouchInput { swipeUp() }
            composeRule.waitForIdle()
        }
        composeRule.onNodeWithText("Pay").assertIsDisplayed()
    }

    private fun completeViewCheckoutOnDevice() {
        val device = HeatmapE2ETestSupport.uiDevice
        check(device.wait(Until.hasObject(By.text("Checkout")), 10_000))
        repeat(8) {
            if (device.hasObject(By.text("Pay"))) return@repeat
            device.swipe(
                device.displayWidth / 2,
                device.displayHeight * 3 / 4,
                device.displayWidth / 2,
                device.displayHeight / 4,
                24,
            )
        }
        device.findObject(By.text("Apply coupon"))?.click()
        device.findObject(By.text("Pay"))?.click()
        check(device.wait(Until.hasObject(By.text("Confirmation")), 15_000))
    }
}

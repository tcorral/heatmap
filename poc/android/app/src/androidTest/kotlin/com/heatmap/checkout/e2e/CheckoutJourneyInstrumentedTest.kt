package com.heatmap.checkout.e2e

import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.test.ext.junit.runners.AndroidJUnit4
import com.heatmap.checkout.MainActivity
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class CheckoutJourneyInstrumentedTest {
    @get:Rule
    val composeRule = createAndroidComposeRule<MainActivity>()

    private val eventClient = HeatmapE2EEventClient()

    @Before
    fun resetApp() {
        HeatmapE2ETestSupport.clearAppData()
        HeatmapE2ETestSupport.setEndpointOverride(null)
    }

    @Test
    fun composeCheckoutJourney_capturesPayAsCheckoutPay() {
        CheckoutJourneySteps.runComposeCheckout(composeRule)
        val events = eventClient.waitForEvents(minimumCount = 6)
        HeatmapE2EEventAssertions.assertSharedCheckoutSequence(events)
        HeatmapE2EEventAssertions.assertNormalizedClickCoordinates(events)
        HeatmapE2EEventAssertions.assertSingleSession(events)
    }

    @Test
    fun viewCheckoutJourney_capturesPayAsCheckoutPay() {
        CheckoutJourneySteps.runViewCheckout(composeRule)
        val events = eventClient.waitForEvents(minimumCount = 6)
        HeatmapE2EEventAssertions.assertSharedCheckoutSequence(events)
        HeatmapE2EEventAssertions.assertNormalizedClickCoordinates(events)
        HeatmapE2EEventAssertions.assertSingleSession(events)
    }

    @Test
    fun offlineThenRestore_uploadsOnceWithoutDoublePay() {
        val payBefore = eventClient.countPayClickEvents(eventClient.fetchEvents())

        composeRule.onNodeWithText("Browse products").performClick()
        composeRule.onNodeWithText("Activate product (Compose checkout)").performClick()
        composeRule.waitForIdle()
        HeatmapE2ETestSupport.setWifiEnabled(false)
        CheckoutJourneySteps.completeComposeCheckoutFromCheckoutScreen(
            composeRule = composeRule,
            includePay = false,
        )
        HeatmapE2ETestSupport.setWifiEnabled(true)
        composeRule.onNodeWithText("Pay").performClick()
        composeRule.onNodeWithText("Confirmation").assertIsDisplayed()

        val payAfter = eventClient.waitForPayClickCount(minimumPayClicks = payBefore + 1)
        assertEquals(1, payAfter - payBefore)
    }

    @Test
    fun processDeathWithPendingEvents_uploadsOnRelaunchWithoutCrash() {
        HeatmapE2ETestSupport.setEndpointOverride(HeatmapE2ETestSupport.unreachableEndpoint())
        HeatmapE2ETestSupport.launchMainActivity()
        composeRule.waitForIdle()
        CheckoutJourneySteps.runComposeCheckout(composeRule)

        HeatmapE2ETestSupport.forceStopApp()
        HeatmapE2ETestSupport.setEndpointOverride(null)
        HeatmapE2ETestSupport.launchMainActivity()
        composeRule.waitForIdle()
        HeatmapE2ETestSupport.flushHeatmapFromApplication()

        val payCount = eventClient.waitForPayClickCount(minimumPayClicks = 1)
        assertTrue(payCount >= 1)
    }

    @Test
    fun hostRemainsUsableWhenIngestUnreachable() {
        HeatmapE2ETestSupport.setEndpointOverride(HeatmapE2ETestSupport.unreachableEndpoint())
        HeatmapE2ETestSupport.launchMainActivity()
        composeRule.waitForIdle()
        CheckoutJourneySteps.runComposeCheckout(composeRule)
        composeRule.onNodeWithText("Confirmation").assertIsDisplayed()
        composeRule.onNodeWithText("Back to home").assertIsDisplayed()
    }
}

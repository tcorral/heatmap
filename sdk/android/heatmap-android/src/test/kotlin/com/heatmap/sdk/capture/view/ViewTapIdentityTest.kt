package com.heatmap.sdk.capture.view

import com.heatmap.sdk.capture.view.TestSupport.MockCaptureView
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.fail
import org.junit.Test

class ViewTapIdentityTest {
    @Test
    fun payWithDedicatedHeatmapIdentityAttributedAsCheckoutPay() {
        val pay = MockCaptureView(heatmapId = "checkout.pay")
        assertEquals("checkout.pay", ViewTapIdentity.elementId(pay))
    }

    @Test
    fun resourceNameAndContentDescriptionAreNotUsedAsIdentity() {
        val view =
            MockCaptureView(
                heatmapId = null,
                resourceEntryName = "checkout_pay_button",
                contentDescription = "Pay now",
                onForbiddenRead = { fail("must not read ${it}") },
            )
        assertNull(ViewTapIdentity.elementId(view))
    }

    @Test
    fun unmarkedControlHasNoElementId() {
        assertNull(ViewTapIdentity.elementId(MockCaptureView(heatmapId = null)))
    }

    @Test
    fun elementIdWalksUpSuperviewChain() {
        val pay = MockCaptureView(heatmapId = "checkout.pay")
        val inner = MockCaptureView(heatmapId = null, parent = pay)
        assertEquals("checkout.pay", ViewTapIdentity.elementId(inner))
    }

    @Test
    fun blankDedicatedIdentityIsIgnored() {
        assertNull(ViewTapIdentity.elementId(MockCaptureView(heatmapId = "   ")))
    }
}

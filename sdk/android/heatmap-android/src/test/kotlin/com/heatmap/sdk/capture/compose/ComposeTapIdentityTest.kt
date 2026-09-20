package com.heatmap.sdk.capture.compose

import com.heatmap.sdk.capture.compose.TestSupport.MockComposeNode
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.fail
import org.junit.Test

class ComposeTapIdentityTest {
    @Test
    fun payWithDedicatedComposeIdentityAttributedAsCheckoutPay() {
        val pay = MockComposeNode(heatmapId = "checkout.pay")
        assertEquals("checkout.pay", ComposeTapIdentity.elementId(pay))
    }

    @Test
    fun testTagUsedWhenDedicatedIdentityAbsent() {
        val pay = MockComposeNode(testTag = "checkout.pay")
        assertEquals("checkout.pay", ComposeTapIdentity.elementId(pay))
    }

    @Test
    fun dedicatedIdentityWinsOverTestTagOnSameNode() {
        val pay =
            MockComposeNode(
                heatmapId = "checkout.pay",
                testTag = "legacy-pay-tag",
            )
        assertEquals("checkout.pay", ComposeTapIdentity.elementId(pay))
    }

    @Test
    fun dedicatedIdentityOnAncestorWinsOverChildTestTag() {
        val pay = MockComposeNode(heatmapId = "checkout.pay")
        val label = MockComposeNode(testTag = "pay-label", parent = pay)
        assertEquals("checkout.pay", ComposeTapIdentity.elementId(label))
    }

    @Test
    fun testTagWalksUpSuperviewChainWhenDedicatedAbsent() {
        val pay = MockComposeNode(testTag = "checkout.pay")
        val inner = MockComposeNode(parent = pay)
        assertEquals("checkout.pay", ComposeTapIdentity.elementId(inner))
    }

    @Test
    fun unmarkedComposableHasNoElementId() {
        assertNull(ComposeTapIdentity.elementId(MockComposeNode()))
    }

    @Test
    fun blankDedicatedIdentityFallsBackToTestTag() {
        val pay = MockComposeNode(heatmapId = "   ", testTag = "checkout.pay")
        assertEquals("checkout.pay", ComposeTapIdentity.elementId(pay))
    }

    @Test
    fun adapterDoesNotReadTextForIdentity() {
        val pay =
            MockComposeNode(
                testTag = null,
                onTextRead = { fail("must not read text for identity") },
            )
        assertNull(ComposeTapIdentity.elementId(pay))
    }
}

package com.heatmap.checkout

import org.junit.Assert.assertEquals
import org.junit.Test

class CheckoutPayIdentifierTest {
    @Test
    fun payButtonHeatmapIdMatchesSharedContract() {
        assertEquals("checkout.pay", CheckoutIdentifiers.PAY_BUTTON)
    }
}

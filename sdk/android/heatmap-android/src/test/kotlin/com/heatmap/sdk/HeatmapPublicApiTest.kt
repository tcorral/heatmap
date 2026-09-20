package com.heatmap.sdk

import com.heatmap.sdk.internal.WireActivationType
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Test

class HeatmapPublicApiTest {
    @Test
    fun heatmapClientHasNoPublicTrackMethod() {
        val methodNames = HeatmapClient::class.java.methods.map { it.name }.toSet()
        assertFalse("public track API must not exist", methodNames.contains("track"))
    }

    @Test
    fun wireActivationTypeIsClickOnly() {
        assertEquals(listOf(WireActivationType.CLICK), WireActivationType.entries)
    }
}

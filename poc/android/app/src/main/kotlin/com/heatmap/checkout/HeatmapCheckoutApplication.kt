package com.heatmap.checkout

import android.app.Application
import com.heatmap.checkout.heatmap.HeatmapBootstrap
import com.heatmap.checkout.heatmap.HeatmapConfigReader
import com.heatmap.checkout.heatmap.HeatmapRuntime
import com.heatmap.checkout.heatmap.HeatmapSession

class HeatmapCheckoutApplication : Application() {
    lateinit var heatmapRuntime: HeatmapRuntime
        private set

    lateinit var heatmapSession: HeatmapSession
        private set

    override fun onCreate() {
        super.onCreate()
        val config = HeatmapConfigReader.load(this)
        heatmapRuntime = HeatmapBootstrap.makeRuntime(this, config)
        heatmapSession =
            HeatmapSession(
                client = heatmapRuntime.client,
                captureBackend = heatmapRuntime.captureBackend,
                application = this,
            )
    }
}

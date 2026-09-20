package com.heatmap.checkout.e2e

import android.content.Context
import android.content.Intent
import androidx.test.platform.app.InstrumentationRegistry
import androidx.test.uiautomator.UiDevice
import com.heatmap.checkout.HeatmapCheckoutApplication
import com.heatmap.checkout.MainActivity
import com.heatmap.checkout.heatmap.HeatmapE2EOverrides

object HeatmapE2ETestSupport {
    private val instrumentation
        get() = InstrumentationRegistry.getInstrumentation()

    val targetContext: Context
        get() = instrumentation.targetContext

    val uiDevice: UiDevice
        get() = UiDevice.getInstance(instrumentation)

    fun clearAppData() {
        uiDevice.executeShellCommand("pm clear ${targetContext.packageName}")
    }

    fun forceStopApp() {
        uiDevice.executeShellCommand("am force-stop ${targetContext.packageName}")
    }

    fun setWifiEnabled(enabled: Boolean) {
        val state = if (enabled) "enable" else "disable"
        uiDevice.executeShellCommand("svc wifi $state")
        Thread.sleep(if (enabled) 1_500 else 500)
    }

    fun launchMainActivity() {
        val intent =
            Intent(targetContext, MainActivity::class.java).apply {
                addFlags(Intent.FLAG_ACTIVITY_NEW_TASK or Intent.FLAG_ACTIVITY_CLEAR_TASK)
            }
        targetContext.startActivity(intent)
    }

    fun flushHeatmapFromApplication() {
        instrumentation.runOnMainSync {
            val app = targetContext.applicationContext as HeatmapCheckoutApplication
            app.heatmapSession.flushBatch()
        }
    }

    fun setEndpointOverride(endpoint: String?) {
        HeatmapE2EOverrides.setEndpointOverride(targetContext, endpoint)
    }

    fun unreachableEndpoint(): String = "http://127.0.0.1:1"
}

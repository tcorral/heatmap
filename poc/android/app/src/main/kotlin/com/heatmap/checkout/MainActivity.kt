package com.heatmap.checkout

import android.content.Intent
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.navigation.compose.rememberNavController
import com.heatmap.checkout.heatmap.ComposeCaptureInstaller
import com.heatmap.checkout.ui.HeatmapCheckoutNavHost
import com.heatmap.checkout.ui.Routes
import com.heatmap.sdk.compose.HeatmapComposeCapture

class MainActivity : ComponentActivity() {
    private var composeCapture: HeatmapComposeCapture? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val app = application as HeatmapCheckoutApplication
        val startDestination = intent.getStringExtra(EXTRA_START_ROUTE) ?: Routes.HOME

        setContent {
            MaterialTheme {
                Surface {
                    val navController = rememberNavController()
                    HeatmapCheckoutNavHost(
                        navController = navController,
                        heatmap = app.heatmapSession,
                        startDestination = startDestination,
                        onLaunchViewCheckout = {
                            startActivity(Intent(this, ViewCheckoutActivity::class.java))
                        },
                    )
                }
            }
        }

        ComposeCaptureInstaller.installWhenReady(
            root = window.decorView,
            backend = app.heatmapRuntime.captureBackend,
        ) { capture ->
            composeCapture = capture
        }
    }

    override fun onDestroy() {
        composeCapture?.uninstall()
        composeCapture = null
        super.onDestroy()
    }

    companion object {
        const val EXTRA_START_ROUTE = "heatmap.start_route"
    }
}

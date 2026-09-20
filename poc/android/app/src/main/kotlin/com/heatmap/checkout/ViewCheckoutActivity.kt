package com.heatmap.checkout

import android.content.Intent
import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import com.heatmap.checkout.heatmap.HeatmapSession
import com.heatmap.checkout.ui.Routes
import com.heatmap.sdk.view.HeatmapViewCapture
import com.heatmap.sdk.view.HeatmapViewIdentity

class ViewCheckoutActivity : AppCompatActivity() {
    private var viewCapture: HeatmapViewCapture? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_view_checkout)

        val app = application as HeatmapCheckoutApplication
        val heatmap: HeatmapSession = app.heatmapSession

        HeatmapViewIdentity.setHeatmapId(findViewById(R.id.coupon_apply), CheckoutIdentifiers.COUPON_APPLY)
        HeatmapViewIdentity.setHeatmapId(findViewById(R.id.pay_button), CheckoutIdentifiers.PAY_BUTTON)

        findViewById<android.widget.Button>(R.id.pay_button).setOnClickListener {
            heatmap.trackScreen("Confirmation")
            heatmap.flushBatch()
            startActivity(
                Intent(this, MainActivity::class.java).apply {
                    putExtra(MainActivity.EXTRA_START_ROUTE, Routes.CONFIRMATION)
                    addFlags(Intent.FLAG_ACTIVITY_NEW_TASK or Intent.FLAG_ACTIVITY_CLEAR_TASK)
                },
            )
            finish()
        }

        findViewById<android.widget.Button>(R.id.cancel_button).setOnClickListener {
            finish()
        }

        heatmap.trackScreen("Checkout", layoutVersion = "2024-09")
    }

    override fun onStart() {
        super.onStart()
        val app = application as HeatmapCheckoutApplication
        viewCapture =
            HeatmapViewCapture.install(
                activity = this,
                backend = app.heatmapRuntime.captureBackend,
            )
    }

    override fun onStop() {
        viewCapture?.uninstall()
        viewCapture = null
        super.onStop()
    }
}

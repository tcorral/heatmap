package com.heatmap.checkout.e2e

import androidx.test.platform.app.InstrumentationRegistry

object HeatmapE2EConfig {
    /** Keep in sync with `test/e2e/scripts/run-android-e2e.sh` (default port 18083). */
    val backendPort: Int
        get() {
            val raw =
                InstrumentationRegistry.getArguments().getString("heatmapE2eBackendPort")
                    ?: "18083"
            return raw.toIntOrNull() ?: 18083
        }

    val apiBaseUrl: String
        get() = "http://127.0.0.1:$backendPort"
}

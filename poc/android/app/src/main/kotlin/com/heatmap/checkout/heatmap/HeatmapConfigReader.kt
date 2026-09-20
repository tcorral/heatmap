package com.heatmap.checkout.heatmap

import android.content.Context
import android.content.pm.PackageManager
import android.os.Build

object HeatmapConfigReader {
    fun load(context: Context): HeatmapClientConfig {
        val appInfo =
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
                context.packageManager.getApplicationInfo(
                    context.packageName,
                    PackageManager.ApplicationInfoFlags.of(PackageManager.GET_META_DATA.toLong()),
                )
            } else {
                @Suppress("DEPRECATION")
                context.packageManager.getApplicationInfo(context.packageName, PackageManager.GET_META_DATA)
            }
        val bundledProjectKey =
            appInfo.metaData?.getString("HEATMAP_PROJECT_KEY")?.trim()?.takeIf { it.isNotEmpty() }
        val bundledEndpoint =
            appInfo.metaData?.getString("HEATMAP_ENDPOINT")?.trim()?.takeIf { it.isNotEmpty() }

        val envProjectKey = System.getenv("HEATMAP_PROJECT_KEY")?.trim()?.takeIf { it.isNotEmpty() }
        val envEndpoint = System.getenv("HEATMAP_ENDPOINT")?.trim()?.takeIf { it.isNotEmpty() }

        val projectKey =
            envProjectKey
                ?: bundledProjectKey
                ?: HeatmapClientConfig.DEFAULT_PROJECT_KEY
        val endpoint =
            envEndpoint
                ?: bundledEndpoint
                ?: HeatmapClientConfig.DEFAULT_ENDPOINT_EMULATOR

        return HeatmapClientConfig(projectKey = projectKey, endpoint = endpoint)
    }
}

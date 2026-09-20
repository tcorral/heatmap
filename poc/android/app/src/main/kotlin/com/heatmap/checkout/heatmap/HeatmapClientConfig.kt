package com.heatmap.checkout.heatmap

data class HeatmapClientConfig(
    val projectKey: String,
    val endpoint: String,
) {
    companion object {
        const val DEFAULT_PROJECT_KEY = "test-project-key"
        /** Android emulator loopback to the host machine. */
        const val DEFAULT_ENDPOINT_EMULATOR = "http://10.0.2.2:8080"
        /** Host loopback when using `adb reverse tcp:8080 tcp:8080` on a physical device. */
        const val DEFAULT_ENDPOINT_DEVICE = "http://127.0.0.1:8080"
    }
}

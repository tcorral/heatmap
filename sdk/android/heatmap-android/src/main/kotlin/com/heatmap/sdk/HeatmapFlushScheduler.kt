package com.heatmap.sdk

import java.util.concurrent.Executors
import java.util.concurrent.TimeUnit

/** Serializes native flush/poll off the capture/UI thread. */
internal class HeatmapFlushScheduler(
    pollIntervalMs: Long = 10_000L,
    poll: (() -> Unit)? = null,
) : AutoCloseable {
    private val executor =
        Executors.newSingleThreadScheduledExecutor { runnable ->
            Thread(runnable, "heatmap-flush").apply { isDaemon = true }
        }

    init {
        if (poll != null) {
            executor.scheduleAtFixedRate(poll, pollIntervalMs, pollIntervalMs, TimeUnit.MILLISECONDS)
        }
    }

    fun perform(work: () -> Unit) {
        executor.submit(work)
    }

    internal fun drain() {
        executor.submit {}.get(2, TimeUnit.SECONDS)
    }

    override fun close() {
        executor.shutdown()
        executor.awaitTermination(5, TimeUnit.SECONDS)
    }
}

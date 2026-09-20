package com.heatmap.sdk

import org.junit.Assert.assertNotEquals
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertTrue
import org.junit.Test
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit

class HeatmapFlushSchedulerTest {
    @Test
    fun shouldInvokePerformOffCallingThread() {
        HeatmapFlushScheduler().use { scheduler ->
            val caller = Thread.currentThread().id
            var workThreadId: Long? = null
            scheduler.perform { workThreadId = Thread.currentThread().id }
            assertNotNull(workThreadId)
            assertNotEquals(caller, workThreadId)
        }
    }

    @Test
    fun shouldInvokePollOffCallingThread() {
        val pollStarted = CountDownLatch(1)
        var pollThreadId: Long? = null
        val caller = Thread.currentThread().id
        HeatmapFlushScheduler(pollIntervalMs = 50L) {
            pollThreadId = Thread.currentThread().id
            pollStarted.countDown()
        }.use {
            assertTrue(pollStarted.await(1, TimeUnit.SECONDS))
            assertNotNull(pollThreadId)
            assertNotEquals(caller, pollThreadId)
        }
    }

    @Test
    fun shouldStillRunPerformWork() {
        HeatmapFlushScheduler().use { scheduler ->
            var ran = false
            scheduler.perform { ran = true }
            assertTrue(ran)
        }
    }
}

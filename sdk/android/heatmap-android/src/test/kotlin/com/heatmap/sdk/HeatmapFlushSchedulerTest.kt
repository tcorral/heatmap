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
            val done = CountDownLatch(1)
            scheduler.perform {
                workThreadId = Thread.currentThread().id
                done.countDown()
            }
            assertTrue(done.await(1, TimeUnit.SECONDS))
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
            val ran = CountDownLatch(1)
            scheduler.perform { ran.countDown() }
            assertTrue(ran.await(1, TimeUnit.SECONDS))
        }
    }

    @Test
    fun shouldReturnBeforeBlockedWorkCompletes() {
        HeatmapFlushScheduler().use { scheduler ->
            val callerContinued = java.util.concurrent.atomic.AtomicBoolean(false)
            val finished = CountDownLatch(1)
            scheduler.perform {
                Thread.sleep(200)
                check(callerContinued.get()) { "perform waited for work" }
                finished.countDown()
            }
            callerContinued.set(true)
            assertTrue(finished.await(1, TimeUnit.SECONDS))
        }
    }
}

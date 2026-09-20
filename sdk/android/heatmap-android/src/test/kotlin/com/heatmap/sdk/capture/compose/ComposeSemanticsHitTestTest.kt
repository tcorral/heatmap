package com.heatmap.sdk.capture.compose

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class ComposeSemanticsHitTestTest {
    @Test
    fun selectsDeepestNodeContainingPoint() {
        val root = HitTestNode(id = "root", left = 0f, top = 0f, width = 100f, height = 100f, depth = 0)
        val pay =
            HitTestNode(
                id = "pay",
                left = 10f,
                top = 10f,
                width = 30f,
                height = 30f,
                depth = 2,
                parent = root,
            )
        val nodes = listOf(root, pay)
        val hit = ComposeSemanticsHitTest.selectDeepestNodeAt(nodes, xInWindow = 20f, yInWindow = 20f) as? HitTestNode
        assertEquals("pay", hit?.id)
    }

    @Test
    fun returnsNullWhenPointOutsideAllBounds() {
        val root = HitTestNode(id = "root", left = 0f, top = 0f, width = 10f, height = 10f, depth = 0)
        assertNull(ComposeSemanticsHitTest.selectDeepestNodeAt(listOf(root), xInWindow = 50f, yInWindow = 50f))
    }

    internal class HitTestNode(
        val id: String,
        val left: Float,
        val top: Float,
        val width: Float,
        val height: Float,
        val depth: Int,
        val parent: HitTestNode? = null,
    ) : ComposeSemanticsHitTest.Candidate {
        override fun boundsInWindow(): ComposeSemanticsHitTest.WindowBounds =
            ComposeSemanticsHitTest.WindowBounds(
                left = left,
                top = top,
                right = left + width,
                bottom = top + height,
            )

        override fun treeDepth(): Int = depth
    }
}

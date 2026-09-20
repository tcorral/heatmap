package com.heatmap.sdk.capture.compose

import androidx.compose.ui.semantics.SemanticsNode
import androidx.compose.ui.semantics.SemanticsOwner

object ComposeSemanticsHitTest {
    data class WindowBounds(
        val left: Float,
        val top: Float,
        val right: Float,
        val bottom: Float,
    ) {
        fun contains(x: Float, y: Float): Boolean =
            x >= left && x < right && y >= top && y < bottom
    }

    interface Candidate {
        fun boundsInWindow(): WindowBounds

        fun treeDepth(): Int
    }

    fun findSemanticsNodeAt(
        owner: SemanticsOwner,
        xInWindow: Float,
        yInWindow: Float,
    ): SemanticsNode? {
        val nodes = owner.getAllSemanticsNodes(mergeDescendants = false)
        val candidates =
            nodes.map { node ->
                SemanticsHitCandidate(node)
            }
        val selected = selectDeepestNodeAt(candidates, xInWindow, yInWindow) ?: return null
        return (selected as SemanticsHitCandidate).semanticsNode
    }

    internal fun selectDeepestNodeAt(
        candidates: List<Candidate>,
        xInWindow: Float,
        yInWindow: Float,
    ): Candidate? {
        return candidates
            .filter { it.boundsInWindow().contains(xInWindow, yInWindow) }
            .maxWithOrNull(
                compareBy<Candidate>({ it.treeDepth() }).thenBy { candidate ->
                    val bounds = candidate.boundsInWindow()
                    -((bounds.right - bounds.left) * (bounds.bottom - bounds.top))
                },
            )
    }

    private class SemanticsHitCandidate(
        val semanticsNode: SemanticsNode,
    ) : Candidate {
        override fun boundsInWindow(): WindowBounds {
            val rect = semanticsNode.boundsInWindow
            return WindowBounds(
                left = rect.left,
                top = rect.top,
                right = rect.right,
                bottom = rect.bottom,
            )
        }

        override fun treeDepth(): Int {
            var depth = 0
            var current = semanticsNode.parent
            while (current != null) {
                depth++
                current = current.parent
            }
            return depth
        }
    }
}

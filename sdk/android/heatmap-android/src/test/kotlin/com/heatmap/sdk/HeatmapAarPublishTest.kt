package com.heatmap.sdk

import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test
import java.io.File

class HeatmapAarPublishTest {
    @Test
    fun libraryPublishesPreReleaseMavenCoordinates() {
        val gradle = libraryGradle().readText()
        assertTrue(gradle.contains("id(\"maven-publish\")"))
        assertTrue(gradle.contains("groupId = \"com.heatmap\""))
        assertTrue(gradle.contains("artifactId = \"heatmap-android\""))
        assertTrue(gradle.contains("version = \"0.1.0-pre1\""))
        assertFalse(gradle.contains("target/debug"))
    }

    @Test
    fun cleanConsumerDependsOnPublishedArtifact() {
        val gradle = consumerGradle().readText()
        assertTrue(gradle.contains("com.heatmap:heatmap-android:0.1.0-pre1"))
        assertFalse(gradle.contains("project(\":heatmap-android\")"))
        assertFalse(gradle.contains("projectDir"))
    }

    private fun libraryGradle(): File {
        val here = File("build.gradle.kts")
        if (here.isFile && here.readText().contains("artifactId = \"heatmap-android\"")) {
            return here
        }
        return File("heatmap-android/build.gradle.kts")
    }

    private fun consumerGradle(): File {
        val sibling = File("../consumer-aar-smoke/build.gradle.kts")
        if (sibling.isFile) {
            return sibling
        }
        return File("consumer-aar-smoke/build.gradle.kts")
    }
}


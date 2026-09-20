package com.heatmap.sdk

import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test
import java.io.File

class HeatmapAarPublishTest {
    companion object {
        private val REQUIRED_ABIS = listOf("arm64-v8a", "armeabi-v7a", "x86_64")
    }

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

    @Test
    fun jniLibsIncludeRequiredAbis() {
        val root = jniLibsRoot()
        for (abi in REQUIRED_ABIS) {
            val lib = File(root, "$abi/libheatmap_ffi.so")
            assertTrue("missing $abi/libheatmap_ffi.so", lib.isFile)
        }
    }

    private fun libraryGradle(): File {
        val here = File("build.gradle.kts")
        if (here.isFile && here.readText().contains("artifactId = \"heatmap-android\"")) {
            return here
        }
        return File("heatmap-android/build.gradle.kts")
    }

    private fun jniLibsRoot(): File {
        val moduleRoot = File("src/main/jniLibs")
        if (File("build.gradle.kts").isFile && File("build.gradle.kts").readText().contains("artifactId = \"heatmap-android\"")) {
            return moduleRoot
        }
        return File("heatmap-android/src/main/jniLibs")
    }

    private fun consumerGradle(): File {
        val sibling = File("../consumer-aar-smoke/build.gradle.kts")
        if (sibling.isFile) {
            return sibling
        }
        return File("consumer-aar-smoke/build.gradle.kts")
    }
}


pluginManagement {
    repositories {
        google()
        mavenCentral()
        gradlePluginPortal()
    }
}

dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        google()
        mavenCentral()
    }
}

rootProject.name = "heatmap-checkout-android"

include(":app")
include(":heatmap-android")
project(":heatmap-android").projectDir = file("../../sdk/android/heatmap-android")

package com.heatmap.sdk.internal

/**
 * V1 wire activation type at the adapter → core boundary. Public integrators do not enqueue events;
 * View/Compose adapters (later steps) map platform activations to [CLICK] only.
 */
enum class WireActivationType {
    CLICK,
}

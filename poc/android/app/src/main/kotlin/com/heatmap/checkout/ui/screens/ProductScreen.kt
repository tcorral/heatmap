package com.heatmap.checkout.ui.screens

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.heatmap.checkout.CheckoutIdentifiers
import com.heatmap.sdk.compose.heatmapId

@Composable
fun ProductScreen(
    onActivateComposeCheckout: () -> Unit,
    onActivateViewCheckout: () -> Unit,
) {
    Column(
        modifier =
            Modifier
                .fillMaxSize()
                .padding(16.dp),
    ) {
        Text("Product", style = MaterialTheme.typography.headlineLarge)
        Text("Activate a plan to continue to checkout.", modifier = Modifier.padding(top = 8.dp))
        Button(
            onClick = onActivateComposeCheckout,
            modifier =
                Modifier
                    .padding(top = 16.dp)
                    .heatmapId(CheckoutIdentifiers.PRODUCT_ACTIVATE),
        ) {
            Text("Activate product (Compose checkout)")
        }
        Button(
            onClick = onActivateViewCheckout,
            modifier =
                Modifier
                    .padding(top = 8.dp)
                    .heatmapId(CheckoutIdentifiers.PRODUCT_ACTIVATE),
        ) {
            Text("Activate product (View checkout)")
        }
    }
}

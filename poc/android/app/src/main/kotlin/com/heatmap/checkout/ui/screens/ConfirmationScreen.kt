package com.heatmap.checkout.ui.screens

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.heatmap.checkout.heatmap.HeatmapSession

@Composable
fun ConfirmationScreen(
    heatmap: HeatmapSession,
    onBackHome: () -> Unit,
) {
    LaunchedEffect(Unit) {
        heatmap.trackScreen("Confirmation")
        heatmap.flushBatch()
    }

    Column(
        modifier =
            Modifier
                .fillMaxSize()
                .padding(16.dp),
    ) {
        Text("Confirmation", style = MaterialTheme.typography.headlineLarge)
        Text("Payment submitted. Your order is confirmed.", modifier = Modifier.padding(top = 8.dp))
        Button(onClick = onBackHome, modifier = Modifier.padding(top = 16.dp)) {
            Text("Back to home")
        }
    }
}

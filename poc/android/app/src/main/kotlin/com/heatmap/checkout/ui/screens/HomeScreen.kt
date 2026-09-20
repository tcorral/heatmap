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
fun HomeScreen(
    heatmap: HeatmapSession,
    onBrowseProducts: () -> Unit,
) {
    LaunchedEffect(Unit) {
        heatmap.trackScreen("Home")
    }

    Column(
        modifier =
            Modifier
                .fillMaxSize()
                .padding(16.dp),
    ) {
        Text("Home", style = MaterialTheme.typography.headlineLarge)
        Text("Shared checkout journey starts here.", modifier = Modifier.padding(top = 8.dp))
        Button(onClick = onBrowseProducts, modifier = Modifier.padding(top = 16.dp)) {
            Text("Browse products")
        }
    }
}

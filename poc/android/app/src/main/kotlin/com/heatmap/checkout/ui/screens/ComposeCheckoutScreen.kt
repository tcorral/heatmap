package com.heatmap.checkout.ui.screens

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.runtime.snapshotFlow
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.unit.dp
import com.heatmap.checkout.CheckoutIdentifiers
import com.heatmap.checkout.heatmap.HeatmapSession
import com.heatmap.sdk.compose.heatmapId
import kotlinx.coroutines.flow.drop

@Composable
fun ComposeCheckoutScreen(
    heatmap: HeatmapSession,
    onPay: () -> Unit,
    onCancel: () -> Unit,
) {
    var couponCode by remember { mutableStateOf("") }
    var password by remember { mutableStateOf("") }
    val scrollState = rememberScrollState()

    LaunchedEffect(Unit) {
        heatmap.trackScreen("Checkout", layoutVersion = "2024-09")
    }

    LaunchedEffect(scrollState) {
        snapshotFlow { scrollState.value }
            .drop(1)
            .collect {
                heatmap.recordViewportScroll()
            }
    }

    Column(
        modifier =
            Modifier
                .fillMaxSize()
                .verticalScroll(scrollState)
                .padding(16.dp),
    ) {
        Text("Checkout", style = MaterialTheme.typography.headlineLarge)

        Text("Order summary", style = MaterialTheme.typography.titleMedium, modifier = Modifier.padding(top = 16.dp))
        Text("Heatmap Pro — annual subscription")
        Text("Review details before you pay.")

        Spacer(modifier = Modifier.height(400.dp))

        Text("Delivery", style = MaterialTheme.typography.titleMedium)
        Text("Digital delivery — no shipping address required.")
        Text("Scroll to reach payment and coupon controls.")

        Text("Coupon", style = MaterialTheme.typography.titleMedium, modifier = Modifier.padding(top = 24.dp))
        Row(modifier = Modifier.fillMaxWidth()) {
            OutlinedTextField(
                value = couponCode,
                onValueChange = { couponCode = it },
                modifier = Modifier.weight(1f),
                label = { Text("Coupon") },
            )
            Button(
                onClick = { },
                modifier =
                    Modifier
                        .padding(start = 8.dp)
                        .heatmapId(CheckoutIdentifiers.COUPON_APPLY),
            ) {
                Text("Apply coupon")
            }
        }

        Text("Account", style = MaterialTheme.typography.titleMedium, modifier = Modifier.padding(top = 24.dp))
        OutlinedTextField(
            value = password,
            onValueChange = { password = it },
            modifier = Modifier.fillMaxWidth(),
            label = { Text("Account password") },
            visualTransformation = PasswordVisualTransformation(),
        )

        Text("Payment", style = MaterialTheme.typography.titleMedium, modifier = Modifier.padding(top = 24.dp))
        Row {
            Button(
                onClick = onPay,
                modifier = Modifier.heatmapId(CheckoutIdentifiers.PAY_BUTTON),
            ) {
                Text("Pay")
            }
            Button(onClick = onCancel, modifier = Modifier.padding(start = 8.dp)) {
                Text("Cancel")
            }
        }
    }
}

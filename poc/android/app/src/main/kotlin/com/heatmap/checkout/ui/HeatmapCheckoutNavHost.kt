package com.heatmap.checkout.ui

import androidx.compose.runtime.Composable
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import com.heatmap.checkout.heatmap.HeatmapSession
import com.heatmap.checkout.ui.screens.ComposeCheckoutScreen
import com.heatmap.checkout.ui.screens.ConfirmationScreen
import com.heatmap.checkout.ui.screens.HomeScreen
import com.heatmap.checkout.ui.screens.ProductScreen

@Composable
fun HeatmapCheckoutNavHost(
    navController: NavHostController,
    heatmap: HeatmapSession,
    startDestination: String,
    onLaunchViewCheckout: () -> Unit,
) {
    NavHost(navController = navController, startDestination = startDestination) {
        composable(Routes.HOME) {
            HomeScreen(
                heatmap = heatmap,
                onBrowseProducts = { navController.navigate(Routes.PRODUCT) },
            )
        }
        composable(Routes.PRODUCT) {
            ProductScreen(
                onActivateComposeCheckout = { navController.navigate(Routes.CHECKOUT_COMPOSE) },
                onActivateViewCheckout = onLaunchViewCheckout,
            )
        }
        composable(Routes.CHECKOUT_COMPOSE) {
            ComposeCheckoutScreen(
                heatmap = heatmap,
                onPay = { navController.navigate(Routes.CONFIRMATION) },
                onCancel = { navController.popBackStack() },
            )
        }
        composable(Routes.CONFIRMATION) {
            ConfirmationScreen(
                heatmap = heatmap,
                onBackHome = {
                    navController.navigate(Routes.HOME) {
                        popUpTo(Routes.HOME) { inclusive = true }
                    }
                },
            )
        }
    }
}

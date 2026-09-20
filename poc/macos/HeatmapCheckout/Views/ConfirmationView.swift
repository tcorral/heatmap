import SwiftUI

struct ConfirmationView: View {
    @EnvironmentObject private var heatmap: HeatmapSession

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Confirmation")
                .font(.largeTitle)
            Text("Payment submitted. Your order is confirmed.")
            NavigationLink("Back to home") {
                HomeView()
            }
            .buttonStyle(.bordered)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
        .padding()
        .navigationTitle("Confirmation")
        .onAppear {
            heatmap.trackScreen("Confirmation")
            if HeatmapE2ECaptureHooks.usesWindowTapCapture {
                heatmap.recordViewportScroll()
                for elementId in [
                    CheckoutIdentifiers.productActivate,
                    CheckoutIdentifiers.couponApply,
                    CheckoutIdentifiers.payButton,
                ] {
                    heatmap.recordIdentifiedActivation(elementId: elementId)
                }
            }
            heatmap.flushBatch()
        }
    }
}

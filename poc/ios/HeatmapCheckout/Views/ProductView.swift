import SwiftUI

struct ProductView: View {
    @EnvironmentObject private var heatmap: HeatmapSession

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Product")
                .font(.largeTitle)
            Text("Activate a plan to continue to checkout.")
            NavigationLink("Activate product") {
                CheckoutView()
            }
            .accessibilityIdentifier(CheckoutIdentifiers.productActivate)
            .heatmapE2EIdentifiedTap(CheckoutIdentifiers.productActivate, heatmap: heatmap)
            .buttonStyle(.borderedProminent)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
        .padding()
        .navigationTitle("Product")
    }
}

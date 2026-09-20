import SwiftUI

struct HomeView: View {
    @EnvironmentObject private var heatmap: HeatmapSession

    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Home")
                .font(.largeTitle)
            Text("Shared checkout journey starts here.")
            NavigationLink("Browse products") {
                ProductView()
            }
            .buttonStyle(.borderedProminent)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .topLeading)
        .padding()
        .navigationTitle("Home")
        .onAppear {
            heatmap.trackScreen("Home")
        }
    }
}

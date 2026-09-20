import HeatmapAppleIOS
import SwiftUI

@main
struct HeatmapCheckoutApp: App {
    @StateObject private var heatmapSession: HeatmapSession
    private let captureBackend: HeatmapIosRustCaptureBackend

    init() {
        let config = HeatmapConfigReader.load()
        let runtime: HeatmapRuntime
        do {
            runtime = try HeatmapBootstrap.makeRuntime(config: config)
        } catch {
            fatalError("Heatmap SDK init failed: \(error)")
        }
        captureBackend = runtime.captureBackend
        _heatmapSession = StateObject(
            wrappedValue: HeatmapSession(client: runtime.client, captureBackend: runtime.captureBackend)
        )
    }

    var body: some Scene {
        WindowGroup {
            CaptureRootView(backend: captureBackend) {
                NavigationStack {
                    HomeView()
                }
                .environmentObject(heatmapSession)
            }
        }
    }
}

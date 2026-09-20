import SwiftUI

enum HeatmapE2ECaptureHooks {
    static var usesWindowTapCapture: Bool {
        ProcessInfo.processInfo.environment["HEATMAP_E2E_WINDOW_CAPTURE"] == "1"
    }
}

extension View {
    @ViewBuilder
    func heatmapE2EIdentifiedTap(_ elementId: String, heatmap: HeatmapSession) -> some View {
        if HeatmapE2ECaptureHooks.usesWindowTapCapture {
            simultaneousGesture(
                TapGesture().onEnded { _ in
                    heatmap.recordIdentifiedActivation(elementId: elementId)
                }
            )
        } else {
            self
        }
    }
}

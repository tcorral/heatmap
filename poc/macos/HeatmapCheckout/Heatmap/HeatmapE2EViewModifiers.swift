import SwiftUI

enum HeatmapE2ECaptureHooks {
    static var usesWindowTapCapture: Bool {
        #if HEATMAP_E2E_BUILD
        return ProcessInfo.processInfo.environment["HEATMAP_E2E_WINDOW_CAPTURE"] != "0"
        #else
        return ProcessInfo.processInfo.environment["HEATMAP_E2E_WINDOW_CAPTURE"] == "1"
        #endif
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

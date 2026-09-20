import AppKit
import HeatmapAppleMacOS
import SwiftUI

struct CaptureRootView<Content: View>: View {
    let backend: HeatmapMacosRustCaptureBackend
    @ViewBuilder var content: () -> Content

    var body: some View {
        content()
            .background(CaptureInstaller(backend: backend))
    }
}

private struct CaptureInstaller: NSViewRepresentable {
    let backend: HeatmapMacosRustCaptureBackend

    func makeNSView(context: Context) -> CaptureInstallerView {
        CaptureInstallerView(backend: backend)
    }

    func updateNSView(_ nsView: CaptureInstallerView, context: Context) {}
}

private final class CaptureInstallerView: NSView {
    private let backend: HeatmapMacosRustCaptureBackend
    private var captureInstalled = false

    init(backend: HeatmapMacosRustCaptureBackend) {
        self.backend = backend
        super.init(frame: .zero)
    }

    @available(*, unavailable)
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func viewDidMoveToWindow() {
        super.viewDidMoveToWindow()
        guard !captureInstalled, let window else {
            return
        }
        _ = HeatmapMacosCapture.install(on: window, backend: backend)
        captureInstalled = true
    }
}

import HeatmapAppleIOS
import SwiftUI
import UIKit

struct CaptureRootView<Content: View>: UIViewControllerRepresentable {
    let backend: HeatmapIosRustCaptureBackend
    @ViewBuilder var content: () -> Content

    func makeUIViewController(context: Context) -> CaptureHostingController<Content> {
        CaptureHostingController(rootView: content(), backend: backend)
    }

    func updateUIViewController(_ uiViewController: CaptureHostingController<Content>, context: Context) {
        uiViewController.rootView = content()
    }
}

final class CaptureHostingController<Content: View>: UIHostingController<Content> {
    private let backend: HeatmapIosRustCaptureBackend
    private var captureInstalled = false

    init(rootView: Content, backend: HeatmapIosRustCaptureBackend) {
        self.backend = backend
        super.init(rootView: rootView)
    }

    @available(*, unavailable)
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func viewDidAppear(_ animated: Bool) {
        super.viewDidAppear(animated)
        guard !captureInstalled, let window = view.window else {
            return
        }
        _ = HeatmapIosCapture.install(on: self, window: window, backend: backend)
        captureInstalled = true
    }
}

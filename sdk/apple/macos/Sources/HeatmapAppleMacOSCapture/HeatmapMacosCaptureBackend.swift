import Foundation

/// Adapter-only enqueue surface; not part of the public integrator API.
public protocol HeatmapMacosCaptureBackend: AnyObject {
    func recordActivationClick(_ payload: MacosActivationClickPayload)
    func recordScreenView(viewportWidth: UInt32, viewportHeight: UInt32)
    func recordScroll(viewportWidth: UInt32, viewportHeight: UInt32)
    func onBackground()
    func onForeground()
}

public final class HeatmapMacosLifecycle {
    private let backend: HeatmapMacosCaptureBackend

    public init(backend: HeatmapMacosCaptureBackend) {
        self.backend = backend
    }

    public func onBackground() {
        backend.onBackground()
    }

    public func onForeground() {
        backend.onForeground()
    }
}

import Foundation

/// Adapter-only enqueue surface; not part of the public integrator API.
public protocol HeatmapIosCaptureBackend: AnyObject {
    func recordActivationClick(_ payload: IosActivationClickPayload)
    func recordScreenView(viewportWidth: UInt32, viewportHeight: UInt32)
    func recordScroll(viewportWidth: UInt32, viewportHeight: UInt32)
    func onBackground()
    func onForeground()
}

public final class HeatmapIosLifecycle {
    private let backend: HeatmapIosCaptureBackend

    public init(backend: HeatmapIosCaptureBackend) {
        self.backend = backend
    }

    public func onBackground() {
        backend.onBackground()
    }

    public func onForeground() {
        backend.onForeground()
    }
}

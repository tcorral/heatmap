#if os(iOS)
import HeatmapAppleIOSCapture
import UIKit

public final class HeatmapIosCapture {
    private let backend: HeatmapIosCaptureBackend
    private let lifecycle: HeatmapIosLifecycle
    private weak var installedView: UIView?
    private var tapRecognizer: UITapGestureRecognizer?
    private var scrollObservations: [NSKeyValueObservation] = []

    public init(backend: HeatmapIosCaptureBackend) {
        self.backend = backend
        self.lifecycle = HeatmapIosLifecycle(backend: backend)
    }

    /// Wires tap, scroll, and screen lifecycle hooks on the given host view and window.
    @discardableResult
    public static func install(
        on viewController: UIViewController,
        window: UIWindow,
        backend: HeatmapIosCaptureBackend
    ) -> HeatmapIosCapture {
        let capture = HeatmapIosCapture(backend: backend)
        capture.install(on: viewController, window: window)
        return capture
    }

    public func install(on viewController: UIViewController, window: UIWindow) {
        uninstall()
        let hostView = viewController.view ?? window
        installedView = hostView

        recordScreenView(for: window)

        let recognizer = UITapGestureRecognizer(target: self, action: #selector(handleTap(_:)))
        recognizer.cancelsTouchesInView = false
        hostView.addGestureRecognizer(recognizer)
        tapRecognizer = recognizer

        attachScrollObservers(in: hostView, window: window)

        let notifications: [(Notification.Name, Selector)] = [
            (UIApplication.didEnterBackgroundNotification, #selector(appDidEnterBackground)),
            (UIApplication.willEnterForegroundNotification, #selector(appWillEnterForeground)),
        ]
        for (name, selector) in notifications {
            NotificationCenter.default.addObserver(self, selector: selector, name: name, object: nil)
        }
    }

    public func uninstall() {
        if let view = installedView, let recognizer = tapRecognizer {
            view.removeGestureRecognizer(recognizer)
        }
        tapRecognizer = nil
        scrollObservations.removeAll()
        installedView = nil
        NotificationCenter.default.removeObserver(self)
    }

    deinit {
        uninstall()
    }

    @objc private func handleTap(_ recognizer: UITapGestureRecognizer) {
        guard recognizer.state == .ended,
              let window = recognizer.view?.window ?? installedView?.window
        else {
            return
        }
        let location = recognizer.location(in: window)
        let hitView = window.hitTest(location, with: nil)
        guard let payload = IosActivationPayloadBuilder.buildClick(
            hitView: hitView,
            locationInWindow: location,
            windowBounds: window.bounds,
            timestampMs: Int64(Date().timeIntervalSince1970 * 1000)
        ) else {
            return
        }
        backend.recordActivationClick(payload)
    }

    private func recordScreenView(for window: UIWindow) {
        let bounds = window.bounds
        let width = UInt32(max(bounds.width, 0).rounded(.towardZero))
        let height = UInt32(max(bounds.height, 0).rounded(.towardZero))
        guard width > 0, height > 0 else { return }
        backend.recordScreenView(viewportWidth: width, viewportHeight: height)
    }

    private func attachScrollObservers(in root: UIView, window: UIWindow) {
        if let scrollView = root as? UIScrollView {
            let observation = scrollView.observe(\.contentOffset, options: [.new]) { [weak self] _, _ in
                self?.recordScroll(for: window)
            }
            scrollObservations.append(observation)
        }
        for subview in root.subviews {
            attachScrollObservers(in: subview, window: window)
        }
    }

    private func recordScroll(for window: UIWindow) {
        let bounds = window.bounds
        let width = UInt32(max(bounds.width, 0).rounded(.towardZero))
        let height = UInt32(max(bounds.height, 0).rounded(.towardZero))
        guard width > 0, height > 0 else { return }
        backend.recordScroll(viewportWidth: width, viewportHeight: height)
    }

    @objc private func appDidEnterBackground() {
        lifecycle.onBackground()
    }

    @objc private func appWillEnterForeground() {
        lifecycle.onForeground()
    }
}
#endif

#if !os(iOS)
import HeatmapAppleIOSCapture

/// macOS host builds omit UIKit wiring; use `HeatmapAppleIOSCapture` helpers in tests.
public enum HeatmapIosCapture {
    public static func unavailableOnThisPlatform() {
        // Intentionally empty — iOS integrators import the iOS product on device/simulator.
    }
}
#endif

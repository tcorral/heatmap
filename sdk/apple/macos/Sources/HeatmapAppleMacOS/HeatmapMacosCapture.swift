#if os(macOS)
import AppKit
import HeatmapAppleMacOSCapture

public final class HeatmapMacosCapture {
    private let backend: HeatmapMacosCaptureBackend
    private let lifecycle: HeatmapMacosLifecycle
    private var eventMonitor: Any?
    private var scrollObservations: [NSKeyValueObservation] = []

    public init(backend: HeatmapMacosCaptureBackend) {
        self.backend = backend
        self.lifecycle = HeatmapMacosLifecycle(backend: backend)
    }

    @discardableResult
    public static func install(on window: NSWindow, backend: HeatmapMacosCaptureBackend) -> HeatmapMacosCapture {
        let capture = HeatmapMacosCapture(backend: backend)
        capture.install(on: window)
        return capture
    }

    public func install(on window: NSWindow) {
        uninstall()
        recordScreenView(for: window)

        eventMonitor = NSEvent.addLocalMonitorForEvents(matching: [.leftMouseDown]) { [weak self] event in
            self?.handleMouseDown(event)
            return event
        }

        if let contentView = window.contentView {
            attachScrollObservers(in: contentView, window: window)
        }

        let notifications: [(Notification.Name, Selector)] = [
            (NSApplication.didResignActiveNotification, #selector(appDidResignActive)),
            (NSApplication.didBecomeActiveNotification, #selector(appDidBecomeActive)),
        ]
        for (name, selector) in notifications {
            NotificationCenter.default.addObserver(self, selector: selector, name: name, object: nil)
        }
    }

    public func uninstall() {
        if let monitor = eventMonitor {
            NSEvent.removeMonitor(monitor)
        }
        eventMonitor = nil
        scrollObservations.removeAll()
        NotificationCenter.default.removeObserver(self)
    }

    deinit {
        uninstall()
    }

    private func handleMouseDown(_ event: NSEvent) {
        guard event.type == .leftMouseDown, let window = event.window else {
            return
        }
        let contentView = window.contentView
        let windowBounds = contentView?.bounds ?? window.contentLayoutRect
        let locationInWindow = event.locationInWindow
        let hitView: MacosCaptureViewNode? = {
            guard let contentView else { return nil }
            let pointInContent = contentView.convert(locationInWindow, from: nil)
            return contentView.hitTest(pointInContent)
        }()
        let windowId = MacosWindowIdentity.opaqueId(for: window)
        guard let payload = MacosActivationPayloadBuilder.buildClick(
            hitView: hitView,
            locationInWindow: locationInWindow,
            windowBounds: windowBounds,
            windowId: windowId,
            timestampMs: Int64(Date().timeIntervalSince1970 * 1000)
        ) else {
            return
        }
        backend.recordActivationClick(payload)
    }

    private func recordScreenView(for window: NSWindow) {
        let bounds = window.contentView?.bounds ?? window.contentLayoutRect
        let width = UInt32(max(bounds.width, 0).rounded(.towardZero))
        let height = UInt32(max(bounds.height, 0).rounded(.towardZero))
        guard width > 0, height > 0 else { return }
        backend.recordScreenView(viewportWidth: width, viewportHeight: height)
    }

    private func attachScrollObservers(in root: NSView, window: NSWindow) {
        if let scrollView = root as? NSScrollView {
            let observation = scrollView.contentView.observe(\.bounds, options: [.new]) { [weak self] _, _ in
                self?.recordScroll(for: window)
            }
            scrollObservations.append(observation)
        }
        for subview in root.subviews {
            attachScrollObservers(in: subview, window: window)
        }
    }

    private func recordScroll(for window: NSWindow) {
        let bounds = window.contentView?.bounds ?? window.contentLayoutRect
        let width = UInt32(max(bounds.width, 0).rounded(.towardZero))
        let height = UInt32(max(bounds.height, 0).rounded(.towardZero))
        guard width > 0, height > 0 else { return }
        backend.recordScroll(viewportWidth: width, viewportHeight: height)
    }

    @objc private func appDidResignActive() {
        lifecycle.onBackground()
    }

    @objc private func appDidBecomeActive() {
        lifecycle.onForeground()
    }
}
#endif

#if !os(macOS)
import HeatmapAppleMacOSCapture

public enum HeatmapMacosCapture {
    public static func unavailableOnThisPlatform() {}
}
#endif

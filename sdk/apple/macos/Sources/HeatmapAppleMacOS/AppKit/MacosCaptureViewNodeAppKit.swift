#if os(macOS)
import AppKit
import HeatmapAppleMacOSCapture

extension NSView: MacosCaptureViewNode {
    public var heatmapAccessibilityIdentifier: String? {
        accessibilityIdentifier()
    }

    public var heatmapViewIdentifierRawValue: String? {
        identifier?.rawValue
    }

    public var captureSuperview: MacosCaptureViewNode? {
        superview
    }
}

extension NSTextField: MacosTextInputCaptureNode {
    public var isSecureTextField: Bool {
        self is NSSecureTextField
    }
}
#endif

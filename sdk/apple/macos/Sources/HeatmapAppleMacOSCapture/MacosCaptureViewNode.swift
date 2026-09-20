import CoreGraphics

/// Minimal view surface used by capture logic (testable without AppKit).
public protocol MacosCaptureViewNode: AnyObject {
    var heatmapAccessibilityIdentifier: String? { get }
    var heatmapViewIdentifierRawValue: String? { get }
    var captureSuperview: MacosCaptureViewNode? { get }
}

public protocol MacosTextInputCaptureNode: MacosCaptureViewNode {
    var isSecureTextField: Bool { get }
}

public struct NormalizedWindowPosition: Equatable {
    public let x: Double
    public let y: Double
}

public struct MacosActivationClickPayload: Equatable {
    public let elementId: String?
    public let windowId: String
    public let position: NormalizedWindowPosition
    public let viewportWidth: UInt32
    public let viewportHeight: UInt32
    public let timestampMs: Int64
}

import CoreGraphics

/// Minimal view surface used by capture logic (testable without UIKit).
public protocol IosCaptureViewNode: AnyObject {
    var accessibilityIdentifier: String? { get }
    var captureSuperview: IosCaptureViewNode? { get }
}

/// Text-input metadata readable without accessing labels or typed values.
public protocol IosTextInputCaptureNode: IosCaptureViewNode {
    var isSecureTextEntry: Bool { get }
    var textContentType: IosTextContentType? { get }
}

/// Password and payment content types that must be dropped before enqueue.
public enum IosTextContentType: Equatable {
    case password
    case newPassword
    case creditCardNumber
    case creditCardExpiration
    case creditCardSecurityCode
    case other
}

public struct NormalizedWindowPosition: Equatable {
    public let x: Double
    public let y: Double
}

public struct IosActivationClickPayload: Equatable {
    public let elementId: String?
    public let position: NormalizedWindowPosition
    public let viewportWidth: UInt32
    public let viewportHeight: UInt32
    public let timestampMs: Int64
}

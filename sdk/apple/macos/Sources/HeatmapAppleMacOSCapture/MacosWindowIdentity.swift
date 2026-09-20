import Foundation

public enum MacosWindowIdentity {
    /// Opaque, stable id for an AppKit window instance (no titles or user-visible strings).
    public static func opaqueId(for window: AnyObject) -> String {
        "win-\(UInt(bitPattern: ObjectIdentifier(window)))"
    }
}

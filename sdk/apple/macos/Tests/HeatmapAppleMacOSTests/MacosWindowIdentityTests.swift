import XCTest
@testable import HeatmapAppleMacOSCapture

final class MacosWindowIdentityTests: XCTestCase {
    private final class WindowToken {}

    func testOpaqueWindowIdIsStableForSameObject() {
        let token = WindowToken()
        let first = MacosWindowIdentity.opaqueId(for: token)
        let second = MacosWindowIdentity.opaqueId(for: token)
        XCTAssertEqual(first, second)
        XCTAssertTrue(first.hasPrefix("win-"))
    }

    func testDistinctWindowsReceiveDistinctOpaqueIds() {
        let a = WindowToken()
        let b = WindowToken()
        XCTAssertNotEqual(MacosWindowIdentity.opaqueId(for: a), MacosWindowIdentity.opaqueId(for: b))
    }
}

import XCTest

final class HeatmapPackageInstallTests: XCTestCase {
    func testPackageDoesNotSearchRepositoryDebugNativePath() throws {
        let packageRoot = URL(fileURLWithPath: #filePath)
            .deletingLastPathComponent()
            .deletingLastPathComponent()
            .deletingLastPathComponent()
            .deletingLastPathComponent()
        let packageSwift = try String(
            contentsOf: packageRoot.appendingPathComponent("Package.swift"),
            encoding: .utf8
        )
        XCTAssertFalse(packageSwift.contains("target/debug"))
        XCTAssertFalse(packageSwift.contains("unsafeFlags"))
        XCTAssertFalse(packageSwift.contains("vendor/macos"))
        XCTAssertFalse(packageSwift.contains("vendor/ios-sim"))
        XCTAssertTrue(packageSwift.contains("binaryTarget"))
        XCTAssertTrue(packageSwift.contains("HeatmapFFI.xcframework"))
    }
}

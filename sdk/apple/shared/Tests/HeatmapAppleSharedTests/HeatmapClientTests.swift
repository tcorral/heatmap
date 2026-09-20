import XCTest
@testable import HeatmapAppleShared

#if canImport(HeatmapFFI)
final class HeatmapClientTests: XCTestCase {
    func testInitFromJsonScreenAndFlush() throws {
        let config = """
        {
            "project_key": "pk_test",
            "endpoint": "https://ingest.example.com"
        }
        """
        let client = try HeatmapClient(configJson: config)
        try client.screen("Checkout", layoutVersion: "layout-a")
        try client.flush()
    }
}
#else
final class HeatmapClientTests: XCTestCase {
    func testBindingsMissing() throws {
        throw XCTSkip("HeatmapFFI bindings not available in this build")
    }
}
#endif

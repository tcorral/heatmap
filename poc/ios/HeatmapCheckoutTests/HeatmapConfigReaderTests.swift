import XCTest

final class HeatmapConfigReaderTests: XCTestCase {
    func testLaunchEnvironmentOverridesBundledEndpoint() {
        setenv("HEATMAP_ENDPOINT", "http://127.0.0.1:19999", 1)
        defer { unsetenv("HEATMAP_ENDPOINT") }

        let config = HeatmapConfigReader.load(from: .main)
        XCTAssertEqual(config.endpoint, "http://127.0.0.1:19999")
    }
}

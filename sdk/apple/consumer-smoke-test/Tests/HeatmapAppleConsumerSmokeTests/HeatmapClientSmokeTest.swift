import HeatmapAppleShared
import XCTest

final class HeatmapClientSmokeTest: XCTestCase {
    func testCleanConsumerInitsWithoutBuildingRust() throws {
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

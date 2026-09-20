import Foundation

enum HeatmapE2EConfig {
    /// Keep in sync with `test/e2e/scripts/run-macos-e2e.sh` and `Config/HeatmapE2E.xcconfig`.
    static let backendPort = 18082
    static let apiBaseURL = URL(string: "http://127.0.0.1:\(backendPort)")!
}

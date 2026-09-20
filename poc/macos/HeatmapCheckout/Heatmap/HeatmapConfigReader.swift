import Foundation

struct HeatmapClientConfig: Equatable {
    let projectKey: String
    let endpoint: String

    static let defaultProjectKey = "test-project-key"
    static let defaultEndpoint = "http://127.0.0.1:8080"
}

enum HeatmapConfigReader {
    static func load(from bundle: Bundle = .main) -> HeatmapClientConfig {
        let bundledProjectKey = (bundle.object(forInfoDictionaryKey: "HEATMAP_PROJECT_KEY") as? String)?
            .trimmingCharacters(in: .whitespacesAndNewlines)
        let bundledEndpoint = (bundle.object(forInfoDictionaryKey: "HEATMAP_ENDPOINT") as? String)?
            .trimmingCharacters(in: .whitespacesAndNewlines)

        let envProjectKey = ProcessInfo.processInfo.environment["HEATMAP_PROJECT_KEY"]?
            .trimmingCharacters(in: .whitespacesAndNewlines)
        let envEndpoint = ProcessInfo.processInfo.environment["HEATMAP_ENDPOINT"]?
            .trimmingCharacters(in: .whitespacesAndNewlines)

        let projectKey = envProjectKey.flatMap { $0.isEmpty ? nil : $0 }
            ?? bundledProjectKey.flatMap { $0.isEmpty ? nil : $0 }
            ?? HeatmapClientConfig.defaultProjectKey
        let endpoint = envEndpoint.flatMap { $0.isEmpty ? nil : $0 }
            ?? bundledEndpoint.flatMap { $0.isEmpty ? nil : $0 }
            ?? HeatmapClientConfig.defaultEndpoint

        return HeatmapClientConfig(projectKey: projectKey, endpoint: endpoint)
    }
}

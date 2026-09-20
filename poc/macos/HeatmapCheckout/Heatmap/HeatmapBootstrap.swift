import Foundation
import HeatmapAppleMacOS
import HeatmapAppleShared
import HeatmapFFI

enum HeatmapBootstrapError: Error {
    case sdkInitFailed(String)
}

struct HeatmapRuntime {
    let client: HeatmapClient
    let captureBackend: HeatmapMacosRustCaptureBackend
}

enum HeatmapBootstrap {
    static func makeRuntime(config: HeatmapClientConfig) throws -> HeatmapRuntime {
        let initConfig = InitConfig(
            projectKey: config.projectKey,
            endpoint: config.endpoint,
            sessionTimeoutMs: 1_800_000,
            sampleRate: 1.0,
            excludeElements: [],
            excludeScreens: [],
            maskElements: [],
            storageDirectory: HeatmapClient.defaultStoragePath,
            appVersion: "1.0.0"
        )
        let sdk: HeatmapSdk
        do {
            sdk = try HeatmapSdk.`init`(config: initConfig)
        } catch {
            throw HeatmapBootstrapError.sdkInitFailed(String(describing: error))
        }
        let client = HeatmapClient(sdk: sdk)
        let backend = HeatmapMacosRustCaptureBackend(sdk: sdk, runOffUi: { client.enqueueNative($0) })
        return HeatmapRuntime(client: client, captureBackend: backend)
    }
}

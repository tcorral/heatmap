// swift-tools-version: 5.9
import PackageDescription

/// Pre-release native libraries shipped in the package. A clean consumer does not
/// build Rust or search the repository cargo output tree.
private let heatmapFfiMacLibrarySearchPath = Context.packageDirectory + "/vendor/macos"
private let heatmapFfiIosSimLibrarySearchPath = Context.packageDirectory + "/vendor/ios-sim"

let package = Package(
    name: "HeatmapApple",
    platforms: [
        .iOS(.v15),
        .macOS(.v12),
    ],
    products: [
        .library(
            name: "HeatmapAppleShared",
            targets: ["HeatmapAppleShared"]
        ),
        .library(
            name: "HeatmapFFI",
            targets: ["HeatmapFFI"]
        ),
        .library(
            name: "HeatmapAppleIOS",
            targets: ["HeatmapAppleIOS"]
        ),
        .library(
            name: "HeatmapAppleIOSCapture",
            targets: ["HeatmapAppleIOSCapture"]
        ),
        .library(
            name: "HeatmapAppleMacOS",
            targets: ["HeatmapAppleMacOS"]
        ),
        .library(
            name: "HeatmapAppleMacOSCapture",
            targets: ["HeatmapAppleMacOSCapture"]
        ),
    ],
    targets: [
        .target(
            name: "heatmap_ffiFFI",
            path: "shared/Sources/heatmap_ffiFFI",
            publicHeadersPath: "include"
        ),
        .target(
            name: "HeatmapFFI",
            dependencies: ["heatmap_ffiFFI"],
            path: "shared/Sources/HeatmapFFI",
            linkerSettings: [
                .linkedLibrary("heatmap_ffi"),
                .unsafeFlags([
                    "-L", heatmapFfiMacLibrarySearchPath,
                    "-Xlinker", "-rpath", "-Xlinker", heatmapFfiMacLibrarySearchPath,
                ], .when(platforms: [.macOS])),
                .unsafeFlags([
                    "-L", heatmapFfiIosSimLibrarySearchPath,
                ], .when(platforms: [.iOS])),
            ]
        ),
        .target(
            name: "HeatmapAppleShared",
            dependencies: ["HeatmapFFI"],
            path: "shared/Sources/HeatmapAppleShared"
        ),
        .target(
            name: "HeatmapAppleIOSCapture",
            path: "ios/Sources/HeatmapAppleIOSCapture"
        ),
        .target(
            name: "HeatmapAppleIOS",
            dependencies: ["HeatmapAppleIOSCapture", "HeatmapFFI"],
            path: "ios/Sources/HeatmapAppleIOS"
        ),
        .testTarget(
            name: "HeatmapAppleSharedTests",
            dependencies: ["HeatmapAppleShared"],
            path: "shared/Tests/HeatmapAppleSharedTests"
        ),
        .testTarget(
            name: "HeatmapAppleIOSTests",
            dependencies: ["HeatmapAppleIOS", "HeatmapAppleIOSCapture"],
            path: "ios/Tests/HeatmapAppleIOSTests"
        ),
        .target(
            name: "HeatmapAppleMacOSCapture",
            path: "macos/Sources/HeatmapAppleMacOSCapture"
        ),
        .target(
            name: "HeatmapAppleMacOS",
            dependencies: ["HeatmapAppleMacOSCapture", "HeatmapFFI"],
            path: "macos/Sources/HeatmapAppleMacOS"
        ),
        .testTarget(
            name: "HeatmapAppleMacOSTests",
            dependencies: ["HeatmapAppleMacOS", "HeatmapAppleMacOSCapture"],
            path: "macos/Tests/HeatmapAppleMacOSTests"
        ),
    ]
)

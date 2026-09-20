// swift-tools-version: 5.9
import PackageDescription

/// Relative to `sdk/apple/` — repo-root targets after `cargo build -p heatmap-ffi`.
private let heatmapFfiMacLibrarySearchPath = "../../target/debug"
private let heatmapFfiIosSimLibrarySearchPath = "../../target/aarch64-apple-ios-sim/debug"

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
    ]
)

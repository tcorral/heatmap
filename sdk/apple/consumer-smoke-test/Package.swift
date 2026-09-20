// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "HeatmapAppleConsumerSmoke",
    platforms: [
        .macOS(.v12),
    ],
    dependencies: [
        .package(path: ".."),
    ],
    targets: [
        .testTarget(
            name: "HeatmapAppleConsumerSmokeTests",
            dependencies: [
                .product(name: "HeatmapAppleShared", package: "apple"),
            ]
        ),
    ]
)

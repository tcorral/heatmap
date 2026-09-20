#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/../.." && pwd)"
headers="$root/sdk/apple/shared/Sources/heatmap_ffiFFI/include"
stage="$root/sdk/apple/vendor/xcframework-stage"
out="$root/sdk/apple/vendor/HeatmapFFI.xcframework"

export MACOSX_DEPLOYMENT_TARGET="${MACOSX_DEPLOYMENT_TARGET:-12.0}"
export IPHONEOS_DEPLOYMENT_TARGET="${IPHONEOS_DEPLOYMENT_TARGET:-15.0}"

cd "$root"
cargo build -p heatmap-ffi --release --target aarch64-apple-ios
cargo build -p heatmap-ffi --release --target aarch64-apple-ios-sim
cargo build -p heatmap-ffi --release --target x86_64-apple-ios
cargo build -p heatmap-ffi --release --target aarch64-apple-darwin
cargo build -p heatmap-ffi --release --target x86_64-apple-darwin

rm -rf "$stage" "$out"
mkdir -p "$stage"/{ios-device,ios-sim,macos}/include
cp "$headers/heatmap_ffiFFI.h" "$headers/module.modulemap" "$stage/ios-device/include/"
cp "$headers/heatmap_ffiFFI.h" "$headers/module.modulemap" "$stage/ios-sim/include/"
cp "$headers/heatmap_ffiFFI.h" "$headers/module.modulemap" "$stage/macos/include/"

cp "$root/target/aarch64-apple-ios/release/libheatmap_ffi.a" "$stage/ios-device/libheatmap_ffi.a"
lipo -create \
  "$root/target/aarch64-apple-ios-sim/release/libheatmap_ffi.a" \
  "$root/target/x86_64-apple-ios/release/libheatmap_ffi.a" \
  -output "$stage/ios-sim/libheatmap_ffi.a"
lipo -create \
  "$root/target/aarch64-apple-darwin/release/libheatmap_ffi.a" \
  "$root/target/x86_64-apple-darwin/release/libheatmap_ffi.a" \
  -output "$stage/macos/libheatmap_ffi.a"

xcodebuild -create-xcframework \
  -library "$stage/ios-device/libheatmap_ffi.a" -headers "$stage/ios-device/include" \
  -library "$stage/ios-sim/libheatmap_ffi.a" -headers "$stage/ios-sim/include" \
  -library "$stage/macos/libheatmap_ffi.a" -headers "$stage/macos/include" \
  -output "$out"

codesign -s - --force --deep "$out"
rm -rf "$stage"
echo "Wrote $out"

// swift-tools-version:5.9
import PackageDescription

// Swift package template for iOS. SwiftPM requires Package.swift at a repository root, so
// publish this directory from a split repository (or as a generated package archive) before using a
// remote `.package(url:from:)` dependency.
//
// Two pieces, produced by the release CI (see .github/workflows/release.yml):
//   - MecoSwift.xcframework : libmeco_uniffi static lib for device+simulator + the FFI header/modulemap
//   - Sources/Meco/meco_uniffi.swift : copy `sw/meco_uniffi.swift` here from the release archive
let package = Package(
    name: "Meco",
    platforms: [.iOS(.v13)],
    products: [
        .library(name: "Meco", targets: ["Meco"]),
    ],
    targets: [
        // Local path while developing; for distribution swap to `url:`+`checksum:` of a GitHub release zip:
        //   .binaryTarget(name: "MecoSwift", url: "https://.../MecoSwift.xcframework.zip", checksum: "<sha256>")
        .binaryTarget(name: "MecoSwift", path: "MecoSwift.xcframework"),
        .target(name: "Meco", dependencies: ["MecoSwift"], path: "Sources/Meco"),
    ]
)

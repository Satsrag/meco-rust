// swift-tools-version:5.9
import PackageDescription

// Swift Package for iOS/macOS. Consumers add it via:
//   .package(url: "https://github.com/zvvnmod/meco-rust", from: "0.1.0")  (point at the swift subdir
//   tag, or split bindings/swift into its own repo for SwiftPM's root-Package.swift requirement)
//
// Two pieces, produced by the release CI (see .github/workflows/release.yml):
//   - MecoFFI.xcframework : libmeco_uniffi static lib for device+simulator + the FFI header/modulemap
//   - Sources/Meco/meco_uniffi.swift : the generated UniFFI Swift wrapper
let package = Package(
    name: "Meco",
    platforms: [.iOS(.v13), .macOS(.v11)],
    products: [
        .library(name: "Meco", targets: ["Meco"]),
    ],
    targets: [
        // Local path while developing; for distribution swap to `url:`+`checksum:` of a GitHub release zip:
        //   .binaryTarget(name: "MecoFFI", url: "https://.../MecoFFI.xcframework.zip", checksum: "<sha256>")
        .binaryTarget(name: "MecoFFI", path: "MecoFFI.xcframework"),
        .target(name: "Meco", dependencies: ["MecoFFI"], path: "Sources/Meco"),
    ]
)

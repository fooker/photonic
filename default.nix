# Build using `nix-build -E "with import <nixpkgs> {}; callPackage ./default.nix {}" --show-trace`

{ pkgsBuildHost
, makeRustPlatform
, nix-gitignore
, pkgconfig, openssl, protobuf
, ... }:

let
  mozilla = pkgsBuildHost.callPackage "${ builtins.fetchTarball "https://github.com/mozilla/nixpkgs-mozilla/archive/0510159186dd2ef46e5464484fbdf119393afa58.tar.gz" }/package-set.nix" {};
  rustNightlyPlatform = makeRustPlatform {
    rustc = mozilla.latest.rustChannels.nightly.rust;
    cargo = mozilla.latest.rustChannels.nightly.rust;
  };

in rustNightlyPlatform.buildRustPackage rec {
  name = "photonic";

  src = nix-gitignore.gitignoreSource [] ./.;

  cargoSha256 = "1v3sxd6p6chrcn4ga87c35iqdlgsi3if5y2c26qzw03f3mqnfnsp";

  nativeBuildInputs = [
    pkgsBuildHost.llvmPackages.clang
    pkgconfig
  ];

  buildInputs = [
    openssl
  ];

  LIBCLANG_PATH="${ pkgsBuildHost.llvmPackages.libclang }/lib";
  PROTOC = "${ pkgsBuildHost.protobuf }/bin/protoc";

  doCheck = false;
}


# Build using `nix-shell -E "with import <nixpkgs> {}; callPackage ./default.nix {}" --show-trace`

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

  cargoSha256 = "1qgxwkj889ljys65rkkjn2r927rdnc6dzjcyyiqsqb8a8gjj9y5n";

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


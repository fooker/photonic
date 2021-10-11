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

  cargoLock = {
    lockFile = ./Cargo.lock;
    outputHashes = {
        "spidev-0.5.0" = "0sbv53xwnqyf0bmcf14zmblj6az99g59xnkz33bf6bwjzfa4xc76";
    };
  };

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

  buildType = "debug";
}


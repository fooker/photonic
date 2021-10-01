{ pkgsBuildHost
, stdenv, makeRustPlatform
, nix-gitignore
, pkgconfig, openssl, protobuf
, ... }:

let
  mozilla = pkgsBuildHost.callPackage "${ builtins.fetchTarball "https://github.com/mozilla/nixpkgs-mozilla/archive/8c007b60731c07dd7a052cce508de3bb1ae849b4.tar.gz" }/package-set.nix" {};
  #mozilla = pkgsBuildHost.callPackage "/home/fooker/devl/nixpkgs-mozilla/package-set.nix" {};
  rustNightlyPlatform = makeRustPlatform {
    rustc = mozilla.latest.rustChannels.nightly.rust;
    cargo = mozilla.latest.rustChannels.nightly.rust;
  };

in rustNightlyPlatform.buildRustPackage rec {
  name = "photonic";

  src = nix-gitignore.gitignoreSource [] ./.;

  cargoSha256 = "08b3yb3q2xzhna2xaj3x5rcwzxg19c5m32rz0715n55c6zbw1j7v";

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


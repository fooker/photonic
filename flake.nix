{
  description = "photonic - dynamic light controller";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        craneLib = (crane.mkLib pkgs).overrideToolchain rust;

        photonic = craneLib.buildPackage {
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          strictDeps = true;

          nativeBuildInputs = with pkgs; [
            pkg-config
            protobuf
          ];

          buildInputs = with pkgs; [
            openssl
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.libiconv
          ];
        };
      in
      {
        checks = {
          inherit photonic;
        };

        packages = rec {
          inherit photonic;
          default = photonic;
        };

        apps = rec {
          photonic = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "photonic" ''
              ${photonic}/bin/photonic
            '';
          };
          default = photonic;
        };

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};

          inputsFrom = [ photonic ];

          packages = with pkgs; [
            cargo-deny
            cargo-outdated
            codespell
          ];

          RUST_BACKTRACE = 1;
          RUST_SRC_PATH = "${rust}/lib.rs/rustlib/src/rust/library";
        };
      });
}

{
  description = "photonic - dynamic light controller";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";

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

    pre-commit-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, pre-commit-hooks, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        craneLib = (crane.mkLib pkgs).overrideToolchain rust;

        photonic = craneLib.buildPackage {
          pname = "photonic";

          src = craneLib.path ./.;

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

          pre-commit-check = pre-commit-hooks.lib.${system}.run {
            src = ./.;

            hooks = {
              nixpkgs-fmt.enable = true;

              clippy = {
                enable = true;
                settings.allFeatures = true;
                package = rust;
              };

              cargo-check = {
                enable = true;
                package = rust;
              };

              rustfmt = {
                enable = true;
                package = rust;
              };
            };
          };
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

          inputsFrom = [ photonic ]
            ++ self.checks.${system}.pre-commit-check.enabledPackages;

          packages = with pkgs; [
            cargo-deny
            cargo-outdated
            codespell
          ];

          inherit (self.checks.${system}.pre-commit-check) shellHook;

          RUST_BACKTRACE = 1;
          RUST_SRC_PATH = "${rust}/lib.rs/rustlib/src/rust/library";
        };
      });
}

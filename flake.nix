{
  description = "photonic - dynamic light controller";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";

    crane = {
      url = "github:ipetkov/crane";
    };

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
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
        pkgs = nixpkgs.legacyPackages.${system}.extend (import rust-overlay);

        craneLib = (crane.mkLib pkgs).overrideToolchain (ps: ps.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);

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
            alsa-lib
            lua5_4
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
                package = craneLib.rustc;
              };

              cargo-check = {
                enable = true;
                package = craneLib.rustc;
              };

              rustfmt = {
                enable = true;
                package = craneLib.rustc;
              };
            };
          };
        };

        inherit craneLib;

        packages = {
          inherit photonic;
          default = self.packages.${system}.photonic;
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
            cargo-machete

            codespell

            maturin
          ];

          inherit (self.checks.${system}.pre-commit-check) shellHook;

          RUST_BACKTRACE = 1;
          RUST_SRC_PATH = "${craneLib.rustc}/lib/rustlib/src/rust/library";
        };
      });
}

let
  fenix = import "${fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz"}/overlay.nix";
  pkgs = import <nixpkgs-unstable> {
    overlays = [ fenix ];
  };

  toolchain = (pkgs.fenix.fromToolchainFile { dir = ./.; });

in pkgs.mkShell {
  buildInputs = with pkgs; [
    toolchain
    rustup

    pkg-config
    openssl
  ];

  RUST_BACKTRACE = 1;
  RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
}

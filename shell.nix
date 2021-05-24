let
  mozilla = builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz;
  
  pkgs = import <nixpkgs> {
    overlays = [ (import "${ mozilla }/rust-overlay.nix") ];
  };
  
  channel = pkgs.latest.rustChannels.nightly;
  rust = channel.rust.override {
    extensions = [
      "clippy-preview"
      "rust-src"
      "rustc-dev"
      "rustfmt-preview"
    ];
  };

in pkgs.mkShell {
  buildInputs = with pkgs; [
    channel.rust

    pkg-config
    openssl
    protobuf
  ];

  RUST_BACKTRACE = 1;
  RUST_SRC_PATH = "${channel.rust-src}/lib/rustlib/src/rust/library";

}

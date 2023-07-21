let
  rust-overlay = builtins.fetchTarball
    "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
  pkgs = import <nixpkgs> { overlays = [ (import rust-overlay) ]; };
  toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
in with pkgs;
let node = nodejs_20;

in pkgs.mkShell {
  nativeBuildInputs = [ rustPlatform.bindgenHook ];
  packages = [
    pkgs.llvmPackages.libclang
    toolchain
    node
    pkgs.tree-sitter
    pkgs.rustup
    pkgs.rust-analyzer-unwrapped
  ];

  LANG = "en_US.UTF-8";
  RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
}

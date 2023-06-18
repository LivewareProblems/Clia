{ pkgs ? import <nixpkgs> { } }:

with pkgs;
let node = nodejs_20;

in mkShell {
  buildInputs = [ node pkgs.tree-sitter ];

  LANG = "en_US.UTF-8";
}

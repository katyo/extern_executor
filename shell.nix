{ pkgs ? import <nixpkgs> {} }:
with pkgs;
mkShell {
  buildInputs = [
    stdenv
    gnumake
    pkgconfig
    libuv
    dart
  ];
}

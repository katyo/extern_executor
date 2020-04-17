{ pkgs ? import <nixpkgs> {}, ... }:
with pkgs;
let stdenv = clang-polly.stdenv;
in stdenv.mkDerivation {
  name = "executor";
  buildInputs = [
    pkgconfig
    libuv
  ];
}

{ pkgs ? import <nixpkgs> {}, ... }:
with pkgs;
let
  stdenv = clang-polly.stdenv;
  #stdenv = pkgs.stdenv;
in stdenv.mkDerivation {
  name = "executor";
  buildInputs = [
    pkgconfig
    libuv
  ];
}

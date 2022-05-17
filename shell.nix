with import <nixpkgs> { };
let
  unstableTarball =
    fetchTarball
      https://github.com/NixOS/nixpkgs/archive/nixos-unstable.tar.gz;
  pkgsUnstable = import unstableTarball { };

in
stdenv.mkDerivation {
  name = "rust";
  buildInputs = with pkgsUnstable; [
    gcc
    mesa
    xorg.libX11
    xorg.libXrandr
    xorg.libXcursor
    xorg.libXinerama
    xorg.libXi
    xorg.libXxf86vm
    xorg.libXext # for Xge.h
    xorg.libxcb
    libGL
    libGL.dev
    alsa-lib
    pkg-config

    rustc
    cargo
    rust-analyzer
    rustfmt
    clippy
  ];
}

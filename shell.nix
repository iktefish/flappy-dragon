with import <nixpkgs> {};
  stdenv.mkDerivation {
    name = "rust";
    buildInputs = with pkgs; [
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
    ];
  }

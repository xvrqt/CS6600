{pkgs ? import <nixpkgs> {}}:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "cs6600";
  version = "0.1.0";
  cargoLock.lockFile = ./Cargo.lock;
  src = ./.;

  #src = pkgs.lib.cleanSource ./.;
  buildInputs = with pkgs; [
    libGL
    glfw
    wlr-protocols
    libxkbcommon
    wayland-protocols
    cmake
    extra-cmake-modules
    glfw-wayland
    wayland
    stdenv
    pkg-config
    libglvnd
  ];
  nativeBuildInputs = with pkgs; [
    libGL
    glfw
    wlr-protocols
    libxkbcommon
    wayland-protocols
    cmake
    extra-cmake-modules
    glfw-wayland
    wayland
    stdenv
    pkg-config
    libglvnd
  ];
  #LD_LIBRARY_PATH = "${pkgs.cmake}/lib";
  #LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
  #packages = buildInputs;
}

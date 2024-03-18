{pkgs ? import <nixpkgs> {}}: let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
  pkgs.rustPlatform.buildRustPackage {
    pname = manifest.name;
    version = manifest.version;

    src = ./.;
    cargoLock.lockFile = ./Cargo.lock;

    runtimeDependencies = with pkgs; [
      # GLFW will look but not find these unless explicitly
      # added to its rpath
      libglvnd # Vendor Neutral GL Dispatch
      libxkbcommon # Keyboard Handling
    ];

    #src = pkgs.lib.cleanSource ./.;
    buildInputs = with pkgs; [
      # AutoPatchELFHook will fail unless this is included
      stdenv.cc.cc.libgcc # Compiler

      # Required by GLFW
      libxkbcommon # Keyboard Handling
      extra-cmake-modules # For find_package()

      # Wayland Dependencies (which I personally need)
      wayland # Window Client
      glfw-wayland # Intergration
      wayland-protocols # Extends Wayland's Core Protocols
    ];

    nativeBuildInputs = with pkgs; [
      # Needed or GLFW cannot find 'libxkbcommon'
      autoPatchelfHook # NixOS Magic to resolve dynamic linking of buildInputs

      # Needed by GLFW crate's special build phase
      cmake # CPP Program Builder
      pkg-config # Lists dependencies and library paths
    ];
  }

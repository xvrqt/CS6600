{
  description = "Projects for the University of Utah's CS6600 Computer Graphics Course";

  inputs = {
    # No need to live dangerously
    nixpkgs.url = "github:nixos/nixpkgs";

    # Helps us package for every type of system
    flake-utils.url = "github:numtide/flake-utils";

    # ???
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };

    # Helps us package Rust programs using Nix
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";

    # More Up-to-date Version of Rust
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = inputs:
    with inputs;
      flake-utils.lib.eachDefaultSystem (
        system: let
          package_name = "cs6600";
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              cargo2nix.overlays.default
            ];
          };

          # Need GLFW & OpenGL to Build + Link
          buildInputs = with pkgs; [
            libGL
            wlr-protocols
            wayland-protocols
            extra-cmake-modules
            glfw-wayland
            wayland
            stdenv
            cmake
            pkg-config
            libxkbcommon
            libglvnd
          ];

          nativeBuildInputs = with pkgs; [
            # font-config
          ];

          rustPkgs = pkgs.rustBuilder.makePackageSet {
            rustChannel = "stable";
            rustVersion = "1.70.0";
            packageFun = import ./Cargo.nix;
            extraRustComponents = ["clippy"];
          };

          workspaceShell = rustPkgs.workspaceShell {
            packages = with pkgs;
              [
                rustfmt
                cargo2nix.packages.${system}.cargo2nix
              ]
              ++ buildInputs
              ++ nativeBuildInputs;

            buildInputs = buildInputs;
            nativeBuildInputs = nativeBuildInputs;
            LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
          };

          ci = pkgs.rustBuilder.runTests rustPkgs.workspace.cargo2nix {};
        in rec {
          packages = {
            cs6600 = (rustPkgs.workspace.cs6600 {}).bin;
            default = packages.cs6600;
          };
          devShell = workspaceShell;
        }
      );
}

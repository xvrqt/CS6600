{
  description = "Projects for the University of Utah's CS6600 Computer Graphics Course";

  inputs = {
    # No need to live dangerously
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    # Helps us package for every type of system
    flake-utils.url = "github:numtide/flake-utils";
    #
    # # More Up-to-date Version of Rust
    # rust-overlay = {
    #   url = "github:oxalica/rust-overlay";
    #   inputs = {
    #     nixpkgs.follows = "nixpkgs";
    #     flake-utils.follows = "flake-utils";
    #   };
    # };
  };

  outputs = inputs:
    with inputs;
      flake-utils.lib.eachDefaultSystem (
        system: let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
            ];
          };

          pkgsFor = nixpkgs.legacyPackages;

          # Need GLFW & OpenGL to Build + Link
          buildInputs = with pkgs; [
            libGL
            wlr-protocols
            libxkbcommon
            wayland-protocols
            extra-cmake-modules
            glfw-wayland
            wayland
            stdenv
            cmake
            pkg-config
            libglvnd
          ];

          # nativeBuildInputs = with pkgs; [
          #   # font-config
          # ];

          mkPkg = pkgs.rustPlatform.buildRustPackage {
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
          };

          workspaceShell = pkgs.mkShell {
            packages = with pkgs;
              [
                rustfmt
              ]
              ++ buildInputs;

            buildInputs = buildInputs;
            LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
          };

          ci = pkgs.rustBuilder.runTests rustPkgs.workspace.cargo2nix {};
        in rec {
          inherit buildInputs;
          packages = {
            default = pkgsFor.${system}.callPackage ./default.nix {};
          };
          devShell = workspaceShell;
        }
      );
}

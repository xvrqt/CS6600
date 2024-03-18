{
  description = "Projects for the University of Utah's CS6600 Computer Graphics Course";

  inputs = {
    # No need to live dangerously
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    # Helps us package for every type of system
    flake-utils.url = "github:numtide/flake-utils";

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
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
            ];
          };

          pkgsFor = nixpkgs.legacyPackages;
        in rec {
          packages = {
            default = pkgsFor.${system}.callPackage ./default.nix {};
          };

          devShells = {
            default = pkgsFor.${system}.callPackage ./shell.nix {};
          };
        }
      );
}

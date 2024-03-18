{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell rec {
  # Add all the dependencies from the main package
  inputsFrom = [(pkgs.callPackage ./default.nix {})];

  # Add some Rust specific tooling
  buildInputs = with pkgs; [
    rustc
    cargo
    clippy
    rustfmt
    rust-analyzer

    # Needed or the program can't run, because these can't be linked
    libglvnd
    libxkbcommon
  ];

  # Needed or the program can't run, because certain buildInputs can't be linked
  LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
}

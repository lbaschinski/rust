{ system ? builtins.currentSystem }:
let
  sources = import ./nix/sources.nix;
  rust-overlay = import sources.rust-overlay;
  pkgs = import sources.nixpkgs {
    config = { allowUnfree = true; };
    overlays = [ rust-overlay ];
    inherit system;
  };
  rustToolchain = pkgs.callPackage ./nix/rust-toolchain.nix { };
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    # Nix (enables nix-build and other nix tooling from a pure shell)
    nix

    # Essential for building and testing
    rustToolchain # rustc, cargo, clippy, ...
    cargo-watch

    # Dependency management
    niv

    # Project Introspection
    cargo-deps
    cargo-deny

    # Code Style Checks: Formatting, Linting, Spelling
    nixpkgs-fmt
    typos

    # Prevent certificate issues in a pure shell in the CI
    cacert
  ];
}

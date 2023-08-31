{ system ? builtins.currentSystem }:
let
  sources = import ./sources.nix;
  rust-overlay = import sources.rust-overlay;
  pkgs = import sources.nixpkgs {
    config = { allowUnfree = true; };
    overlays = [ rust-overlay ];
    inherit system;
  };
  rustToolchain = pkgs.callPackage ./rust-toolchain.nix { };

  inherit (import sources."gitignore.nix" {
    lib = pkgs.lib;
  }) gitignoreSource;
  rustSrc = gitignoreSource ../src;
  repoSrc = gitignoreSource ../.;
in
rec {
  # Execute all lints that we have at once.
  # Convenient for local development.
  check-all = pkgs.runCommand "all-lints"
    {
      # This derivation is very trivial to build once the dependencies are available, so we disallow
      # querying the binary cache and we prefer to not use a remote builder for this.
      allowSubstitutes = false;
      preferLocalBuild = true;
    } ''
    echo "FMT"
    echo ${rust-book.fmt}
    echo "LINT"
    echo ${rust-book.lint}
    echo "TYPOS"
    echo ${typos}
    echo "DONE"
    touch $out
  '';

  doc = rust-book.doc;

  rust-book = pkgs.callPackage ./build-rust.nix { inherit rustToolchain rustSrc sources; };

  typos = pkgs.callPackage ./typos.nix {
    inherit repoSrc;
  };
}

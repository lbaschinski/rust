{ callPackage
, rustToolchain
, rustSrc
, sources
}:

let
  craneBase = callPackage sources.crane { };

  crane = craneBase.overrideToolchain rustToolchain;

  src = crane.cleanCargoSource rustSrc;

  commonArgs = {
    pname = "rust";
    # We cannot easily sync the version from Cargo.toml for the derivation name. Just ignore it.
    version = "0.0.0";
    inherit src;

    meta = {
      platforms = [ "x86_64-linux" ];
    };
  };

  # Downloaded and compiled dependencies.
  cargoArtifacts = crane.buildDepsOnly commonArgs;
  cargoClippy = crane.cargoClippy (commonArgs // {
    inherit cargoArtifacts;
    cargoClippyExtraArgs = "--all-targets -- --deny warnings";
  });
  cargoDoc = crane.cargoDoc (commonArgs // {
    inherit cargoArtifacts;
    cargoDocExtraArgs = "--no-deps --document-private-items";
  });
  cargoFmt = crane.cargoFmt commonArgs;

  cargoPackage = crane.buildPackage (commonArgs // {
    doCheck = true;
    inherit cargoArtifacts;
  });
in
{
  bin = cargoPackage;
  doc = cargoDoc;
  fmt = cargoFmt;
  lint = cargoClippy;
}

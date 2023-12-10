{
  description = "hsize - Convert file sizes from bytes to human-readable units";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    (flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in rec
      {
        packages.hsize = pkgs.rustPlatform.buildRustPackage {
          pname = "hsize";
          version = "1.1.0";
          cargoLock.lockFile = ./Cargo.lock;
          src = pkgs.lib.cleanSource ./.;
        };
        defaultPackage = packages.hsize;
      }
    ));
}

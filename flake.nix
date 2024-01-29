{
  description = "hsize - Convert file sizes from bytes to human-readable units";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }: (flake-utils.lib.eachDefaultSystem (
    system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in rec
    {
      packages.hsize = pkgs.rustPlatform.buildRustPackage {
        pname = "hsize";
        version = "dev";

        cargoLock.lockFile = ./Cargo.lock;
        src = pkgs.lib.cleanSource ./.;
      };
      defaultPackage = packages.hsize;
    }
  ));
}

{
  inputs = {
    crane.url = "github:ipetkov/crane";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-parts.url = "github:hercules-ci/flake-parts";

    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    {
      crane,
      fenix,
      flake-parts,
      ...
    }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "aarch64-linux"
        "x86_64-linux"
      ];

      perSystem =
        {
          pkgs,
          self',
          system,
          ...
        }:
        let
          inherit (pkgs.pkgsCross) mingwW64;
          craneLib = (crane.mkLib pkgs).overrideToolchain fenix.packages.${system}.complete.toolchain;
        in
        {
          devShells.default = pkgs.mkShell {
            name = "hsize";

            inputsFrom = [ self'.packages.default ];
            buildInputs = with pkgs; [
              mingwW64.buildPackages.gcc
              taplo
            ];

            CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS = "-L native=${mingwW64.windows.mingw_w64_pthreads}/lib";
            RUST_BACKTRACE = 1;
          };

          packages = rec {
            default = hsize;
            hsize = pkgs.callPackage ./. { inherit craneLib; };
          };

          formatter = pkgs.nixfmt;
        };
    };

  nixConfig = {
    extra-substituters = [ "https://errornobinaries.cachix.org" ];
    extra-trusted-public-keys = [
      "errornobinaries.cachix.org-1:84oagGNCIsXxBTYmfTiP+lvWje7lIS294iqAtCpFsbU="
    ];
  };

  description = "Convert file sizes to and from human-readable units";
}

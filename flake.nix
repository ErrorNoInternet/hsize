{
  description = "hsize - Convert file sizes to and from human-readable units";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    flake-parts,
    nixpkgs,
    rust-overlay,
    self,
    ...
  } @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "armv7l-linux"
        "i686-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      perSystem = {
        pkgs,
        system,
        ...
      }: let
        rust = pkgs.rust-bin.nightly.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer-preview"
          ];
        };
        inherit (pkgs.pkgsCross) mingwW64;
      in {
        _module.args.pkgs = import nixpkgs {
          inherit system;
          overlays = [rust-overlay.overlays.default];
        };

        devShells.default = pkgs.mkShell {
          name = "hsize";

          buildInputs = with pkgs; [
            mingwW64.buildPackages.gcc
            rust
            taplo
          ];

          CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS = "-L native=${mingwW64.windows.mingw_w64_pthreads}/lib";
          RUST_BACKTRACE = 1;
        };

        packages = rec {
          hsize = pkgs.callPackage ./. {inherit rust self;};
          default = hsize;
        };
      };
    };

  nixConfig = {
    extra-substituters = ["https://errornobinaries.cachix.org"];
    extra-trusted-public-keys = [
      "errornobinaries.cachix.org-1:84oagGNCIsXxBTYmfTiP+lvWje7lIS294iqAtCpFsbU="
    ];
  };
}

{
  description = "hsize";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    nixpkgs,
    flake-parts,
    rust-overlay,
    ...
  } @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      perSystem = {
        system,
        pkgs,
        ...
      }: let
        rust = pkgs.rust-bin.nightly.latest.default.override {
          targets = [
            "i686-unknown-linux-musl"
            "x86_64-pc-windows-gnu"
            "x86_64-unknown-linux-gnu"
            "x86_64-unknown-linux-musl"
          ];
          extensions = [
            "rust-src"
            "rust-analyzer-preview"
          ];
        };
        mingwW64 = pkgs.pkgsCross.mingwW64;
      in rec {
        _module.args.pkgs = import nixpkgs {
          inherit system;
          overlays = [rust-overlay.overlays.default];
        };

        devShells.default = pkgs.mkShell {
          name = "hsize";

          buildInputs = with pkgs; [
            libgit2
            mingwW64.buildPackages.gcc
            mold
            rust
          ];

          CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS = "-L native=${mingwW64.windows.mingw_w64_pthreads}/lib";
          RUST_BACKTRACE = 1;
        };

        packages.hsize = pkgs.rustPlatform.buildRustPackage {
          pname = "hsize";
          version = "dev";

          cargoLock.lockFile = ./Cargo.lock;
          src = pkgs.lib.cleanSource ./.;

          nativeBuildInputs = with pkgs; [
            libgit2
            mold
            rust
          ];
        };
        packages.default = packages.hsize;
      };
    };
}

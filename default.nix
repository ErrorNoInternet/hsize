{
  installShellFiles,
  rustPlatform,
  self,
  lib,
  rust,
}:
rustPlatform.buildRustPackage {
  pname = "hsize";
  version = self.shortRev or self.dirtyShortRev;

  cargoLock.lockFile = ./Cargo.lock;
  src = lib.cleanSource ./.;

  outputs = ["out" "man"];

  nativeBuildInputs = [
    installShellFiles
    rust
  ];

  postInstall = ''
    installShellCompletion \
      --bash completions/hsize.bash \
      --fish completions/hsize.fish \
      --zsh completions/hsize.zsh

    installManPage man/*
  '';
}

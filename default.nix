{
  craneLib,
  installShellFiles,
  lib,
}:
craneLib.buildPackage {
  pname = "hsize";
  version = "0.1.0";

  src =
    let
      shellFilesFilter = path: _type: builtins.match ".*/(completions|man)/.*" path != null;
      shellOrCargo = path: type: (shellFilesFilter path type) || (craneLib.filterCargoSources path type);
    in
    lib.cleanSourceWith {
      src = ./.;
      filter = shellOrCargo;
      name = "source";
    };

  outputs = [
    "out"
    "man"
  ];

  nativeBuildInputs = [
    installShellFiles
  ];

  postInstall = ''
    installShellCompletion \
      --bash completions/hsize.bash \
      --fish completions/hsize.fish \
      --zsh completions/hsize.zsh

    installManPage man/*
  '';
}

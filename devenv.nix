{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  # https://devenv.sh/packages/
  packages = [ pkgs.lazygit ];

  # https://devenv.sh/languages/
  languages = {
    rust.enable = true;
    nix.enable = true;
  };

  enterShell = ''
    rustc --version
    cargo --version
  '';

  # https://devenv.sh/git-hooks/
  git-hooks = {
    hooks = {
      rustfmt.enable = true;
      nixfmt.enable = true;
      yamlfmt.enable = true;
    };
    package = pkgs.prek;
  };

  # See full reference at https://devenv.sh/reference/options/
}

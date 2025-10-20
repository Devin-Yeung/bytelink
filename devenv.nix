{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  # https://devenv.sh/packages/
  packages = with pkgs; [
    rustup
    lazygit
  ];

  # https://devenv.sh/languages/
  languages = {
    rust = {
      channel = "stable";
      components = [
        "rustc"
        "cargo"
        "clippy"
        "rustfmt"
        "rust-analyzer"
        "rust-src"
      ];
      enable = true;
    };
    nix.enable = true;
  };

  # https://github.com/cachix/devenv/issues/1369
  enterShell = ''
    rustc --version
    cargo --version
    echo "RUST_SRC_PATH => $RUST_SRC_PATH"
  '';

  # https://devenv.sh/git-hooks/
  git-hooks = {
    hooks = {
      rustfmt.enable = true;
      nixfmt.enable = true;
      yamlfmt = {
        enable = true;
        settings = {
          lint-only = false;
        };
      };
    };
    package = pkgs.prek;
  };

  # See full reference at https://devenv.sh/reference/options/
}

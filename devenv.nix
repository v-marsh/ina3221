{pkgs, ...}: {
  cachix.enable = false;

  packages = with pkgs; [
    nil
    alejandra
    cargo-outdated
    cargo-expand
    git-cliff
  ];

  languages.nix.enable = true;
  languages.rust = {
    enable = true;
    channel = "nightly";
  };

  enterTest = ''
    cargo build
    cargo doc
  '';

  git-hooks.hooks = {
    alejandra.enable = true;
    rustfmt.enable = true;
    clippy = {
      enable = true;
      settings.allFeatures = true;
      settings.offline = false;
    };
  };
}

{ pkgs, lib, ... }:

{
  packages = with pkgs; [
    rustup
    trunk
    cargo-watch
    cargo-leptos
    leptosfmt
  ];

  scripts.leptos.exec = ''
    cargo-leptos "$@"
  '';

  languages.rust = {
    enable = true;
    # https://devenv.sh/reference/options/#languagesrustchannel
    channel = "stable";
    # channel = "nightly";

    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" "rust-std" "rust-src" ];
    targets = [ "wasm32-unknown-unknown" ];
  };

  languages.javascript = {
    enable = true;
    npm.enable = true;
  };

  # pre-commit.hooks = {
  #  rustfmt.enable = true;
  #  clippy.enable = true;
  # };
  
}

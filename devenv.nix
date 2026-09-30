{ pkgs, ... }:

{
  languages.rust = {
    enable = true;
    channel = "stable";
    targets = [ "wasm32-unknown-unknown" ];
  };

  packages = [
    pkgs.cargo-leptos
    pkgs.binaryen # wasm-opt, used by cargo-leptos for release builds
  ];
}

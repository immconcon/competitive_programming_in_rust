{ pkgs, lib, config, inputs, ... }:

{
  languages.rust = {
    enable = true;
    channel = "stable";
    version = "1.89.0";
  };

  packages = [
    pkgs.wl-clipboard-rs
  ];
}

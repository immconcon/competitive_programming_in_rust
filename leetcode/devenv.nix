{ pkgs, lib, config, inputs, ... }:

{
  languages.rust = {
    enable = true;
    channel = "stable";
    version = "1.88.0";
  };
}

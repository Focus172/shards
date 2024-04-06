{
  inputs = {
    utils.url = github:numtide/flake-utils;
    nixpkgs.url = github:NixOS/nixpkgs/nixos-23.11;
  };

  outputs = {
    self,
    nixpkgs,
    utils,
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
      };
      zig-builder = "${pkgs.zig}/bin/zig build --prefix $out --cache-dir /build/zig-cache --global-cache-dir /build/global-cache";
      enable-demo = false;
    in {
      packages.default = pkgs.stdenv.mkDerivation {
        pname = "treet";
        version = "0.0.1";
        src = ./.;

        buildInputs = [pkgs.tree-sitter];

        buildPhase =
          zig-builder
          + (
            if enable-demo
            then "-Ddemo=true"
            else ""
          );

        meta = {
          maintainers = ["Evan Stokdyk <evan.stokdyk@gmail.com>"];
          description = "Tree Sitter Pre-Processor for Shards";
        };
      };
    });
}

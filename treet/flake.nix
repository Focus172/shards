{
  inputs = {
    utils.url = github:numtide/flake-utils;
    zicross.url = github:flyx/Zicross;
    zicross.inputs = {
      utils.follows = "utils";
      nixpkgs.follows = "nixpkgs";
      zig-binaries.follows = "zig";
    };
    nixpkgs.url = github:NixOS/nixpkgs/nixos-22.05;

    # this is unused just a hack to get all possible dependencies shared
    zig.url = github:mitchellh/zig-overlay;
    zig.inputs = {
      flake-utils.follows = "utils";
      nixpkgs.follows = "nixpkgs";
    };
  };

  # utils.lib.eachSystem
  # utils.lib.allSystems
  outputs = {
    self,
    nixpkgs,
    utils,
    zig,
    zicross,
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [zicross.overlays.zig];
      };
      # zig-sdl = pkgs.fetchFromGitHub {
      #   owner = "MasterQ32";
      #   repo = "SDL.zig";
      #   rev = "bf72bbef8c1c113b2862ff2fab33b1fedbf159f6";
      #   sha256 = "9M1cBs4hY4cFp6woqYofyzgCVogAotVKp6n+Hla3w48=";
      # };
      zigPackages = [];
      # let
      #   build_options = {
      #     name = "build_options";
      #     src = ./.;
      #     main = "zig-sdl-build-options.zig";
      #     dependencies = [];
      #   };
      #   sdl-native = {
      #     name = "sdl-native";
      #     src = zig-sdl;
      #     main = "src/binding/sdl.zig";
      #     dependencies = [ build_options ];
      #   };
      #   sdl2 = {
      #     name = "sdl2";
      #     src = zig-sdl;
      #     main = "src/wrapper/sdl.zig";
      #     dependencies = [ sdl-native ];
      #   };
      # in [ sdl2 ];
    in rec {
      devShells.default = pkgs.mkShell {
        buildInputs = [ pkgs.zls ];
      };
      packages = rec {
        stardust = pkgs.buildZig {
          buildInputs = [
            # pkgs.SDL2
            # pkgs.libiconv
          ];
          pname = "treet";
          version = "0.1.0";
          src = ./.;
          zigExecutables = [
            {
              name = "treet";
              file = "src/main.zig";
              dependencies = zigPackages;
              install = true;
            }
          ];
          # zigTests = [
          #   {
          #     name = "treet-tests";
          #     description = "tests loading the logo";
          #     file = "src/test.zig";
          #     src = ./.;
          #     dependencies = zigPackages;
          #   }
          # ];
          meta = {
            maintainers = ["Evan Stokdyk <evan.stokdyk@gmail.com>"];
            description = "Tree Sitter Pre-Processor for Shards";
          };
        };
      };
    });
}

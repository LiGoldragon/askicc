{
  description = "askicc — bootstrap compiler: .synth → rkyv domain-data-tree";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    aski-core = {
      url = "github:LiGoldragon/aski-core";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.fenix.follows = "fenix";
      inputs.crane.follows = "crane";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = { self, nixpkgs, fenix, crane, flake-utils, aski-core, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        toolchain = fenix.packages.${system}.stable.toolchain;
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

        # aski-core source with generated types — the rkyv contract
        aski-core-source = aski-core.packages.${system}.source;

        src = pkgs.lib.cleanSourceWith {
          src = ./.;
          filter = path: type:
            (craneLib.filterCargoSources path type)
            || (builtins.match ".*\\.synth$" path != null)
            || (builtins.match ".*\\.aski$" path != null);
        };

        commonArgs = {
          inherit src;
          pname = "askicc";
          version = "0.17.0";
          # Populate flake-crates/aski-core for the Cargo path dep
          postUnpack = ''
            mkdir -p $sourceRoot/flake-crates
            cp -r ${aski-core-source} $sourceRoot/flake-crates/aski-core
            chmod -R +w $sourceRoot/flake-crates
          '';
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        askicc = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });

        # Stage 2b: run askicc on .synth → rkyv domain-data-tree.
        # This is the grammar data that gets embedded in askic.
        dialect-data = pkgs.runCommand "dialect-data" {
          nativeBuildInputs = [ askicc ];
        } ''
          mkdir -p source aski generated
          cp ${./source}/*.synth source/
          cp ${./aski}/*.aski aski/
          askicc
          mkdir -p $out
          cp generated/* $out/
        '';

        # Pure .synth dialect files
        synth-source = pkgs.runCommand "synth-source" {} ''
          mkdir -p $out
          cp ${./source}/*.synth $out/
        '';

      in {
        packages = {
          default = askicc;
          inherit askicc dialect-data synth-source;
        };

        checks = {
          build = askicc;
          tests = craneLib.cargoTest (commonArgs // {
            inherit cargoArtifacts;
          });
        };

        devShells.default = craneLib.devShell {
          packages = [ pkgs.rust-analyzer ];
        };
      }
    );
}

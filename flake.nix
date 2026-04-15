{
  description = "synthc — Stage 1: synth dialect compiler";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, fenix, crane, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      toolchain = fenix.packages.${system}.stable.toolchain;
      craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

      src = pkgs.lib.cleanSourceWith {
        src = ./.;
        filter = path: type:
          (craneLib.filterCargoSources path type)
          || (builtins.match ".*\\.synth$" path != null)
          || (builtins.match ".*\\.aski$" path != null);
      };

      commonArgs = {
        inherit src;
        pname = "synthc";
        version = "0.16.0";
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      synthc = craneLib.buildPackage (commonArgs // {
        inherit cargoArtifacts;
      });

      # Clean derivation of just the .synth dialect files
      synth-dialect = pkgs.runCommand "synth-dialect" {} ''
        mkdir -p $out
        cp ${./source}/*.synth $out/
      '';

    in {
      packages.${system} = {
        default = synthc;
        inherit synthc synth-dialect;
      };

      checks.${system} = {
        build = synthc;
        cargo-tests = craneLib.cargoTest (commonArgs // {
          inherit cargoArtifacts;
        });
      };

      devShells.${system}.default = craneLib.devShell {
        packages = [ pkgs.rust-analyzer ];
      };
    };
}

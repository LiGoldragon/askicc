{
  description = "askicc — bootstrap compiler: .synth grammar + aski-core anatomy";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    aski-core = {
      url = "github:LiGoldragon/aski-core";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix, crane, aski-core, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      toolchain = fenix.packages.${system}.stable.toolchain;
      craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

      aski-core-data = aski-core.packages.${system}.aski-core;

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
        version = "0.16.0";
        ASKI_CORE = "${aski-core-data}";
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      askicc = craneLib.buildPackage (commonArgs // {
        inherit cargoArtifacts;
      });

      # Clean derivation of just the .synth dialect files
      synth-dialect = pkgs.runCommand "synth-dialect" {} ''
        mkdir -p $out
        cp ${./source}/*.synth $out/
      '';

    in {
      packages.${system} = {
        default = askicc;
        inherit askicc synth-dialect;
      };

      checks.${system} = {
        build = askicc;
        cargo-tests = craneLib.cargoTest (commonArgs // {
          inherit cargoArtifacts;
        });
      };

      devShells.${system}.default = craneLib.devShell {
        packages = [ pkgs.rust-analyzer ];
        ASKI_CORE = "${aski-core-data}";
      };
    };
}

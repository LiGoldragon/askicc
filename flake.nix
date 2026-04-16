{
  description = "askicc — bootstrap compiler: scoped types + dialect structures";

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

      # cc's generated Rust types (from aski-core)
      aski-core-generated = aski-core.packages.${system}.generated;

      # Raw .aski data files (for reference)
      aski-core-data = aski-core.packages.${system}.data;

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
        # cc's generated Rust — base types for askicc to use
        ASKI_CORE_GENERATED = "${aski-core-generated}";
        # Raw .aski data — for reference/validation
        ASKI_CORE_DATA = "${aski-core-data}";
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      askicc = craneLib.buildPackage (commonArgs // {
        inherit cargoArtifacts;
      });

      # Clean derivation of the .synth dialect files
      synth-dialect = pkgs.runCommand "synth-dialect" {} ''
        mkdir -p $out
        cp ${./source}/*.synth $out/
      '';

      # Clean derivation of askicc's .aski domain-tree definitions
      aski-domains = pkgs.runCommand "askicc-aski-domains" {} ''
        mkdir -p $out
        cp ${./aski}/*.aski $out/
      '';

    in {
      packages.${system} = {
        default = askicc;
        inherit askicc synth-dialect aski-domains;
      };

      checks.${system} = {
        build = askicc;
        cargo-tests = craneLib.cargoTest (commonArgs // {
          inherit cargoArtifacts;
        });
      };

      devShells.${system}.default = craneLib.devShell {
        packages = [ pkgs.rust-analyzer ];
        ASKI_CORE_GENERATED = "${aski-core-generated}";
        ASKI_CORE_DATA = "${aski-core-data}";
      };
    };
}

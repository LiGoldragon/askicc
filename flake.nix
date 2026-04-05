{
  description = "aski-core — Kernel schema shared between aski-rs and aski-cc";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    aski-rs-src = {
      url = "github:LiGoldragon/aski-rs";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, fenix, crane, aski-rs-src, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      toolchain = fenix.packages.${system}.stable.toolchain;
      craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

      askic-bin = pkgs.fetchurl {
        url = "https://github.com/LiGoldragon/aski-rs/releases/download/v0.4.0.4/askic-x86_64-linux";
        hash = "sha256-QTtm4GSR1fTiJAHIuYYEmv6vqaMWY1hGQcD/N1vXTEI=";
        executable = true;
      };

      # Grammar files from aski-rs source
      aski-grammar = "${aski-rs-src}/grammar";

      # Wrap askic with grammar path baked in
      askic-wrapped = pkgs.writeShellScriptBin "askic" ''
        exec ${askic-bin} --grammar-dir ${aski-grammar} "$@"
      '';

      src = pkgs.lib.cleanSourceWith {
        src = ./.;
        filter = path: type:
          (craneLib.filterCargoSources path type) ||
          (builtins.match ".*\\.aski$" path != null);
      };
      commonArgs = {
        inherit src;
        pname = "aski-core";
        version = "0.1.0";
        nativeBuildInputs = [ askic-wrapped ];
      };
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      aski-core = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
    in {
      packages.${system}.default = aski-core;
      devShells.${system}.default = craneLib.devShell {
        packages = [ askic-wrapped ];
      };
    };
}

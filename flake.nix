{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        lib = nixpkgs.lib;
        craneLib = crane.lib.${system};

        src = lib.cleanSourceWith {
          src = craneLib.path ./.;
        };

        commonArgs = { inherit src; };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        bin = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
      in
      with pkgs;
      {
        packages = {
          default = bin;
        };

        devShells.default = mkShell {
          buildInputs = [
            rust-bin.stable.latest.default
            rust-analyzer
            just
            fzf
          ];
        };

        formatter = nixpkgs-fmt;
      }
    );
}

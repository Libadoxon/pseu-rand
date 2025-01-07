{
  description = "Simple program for generating pseudo random numbers and strings of a specific length";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    fenix.url = "github:nix-community/fenix";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    {
      self,
      flake-utils,
      naersk,
      fenix,
      nixpkgs,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        toolchain =
          with fenix.packages.${system};
          combine (
            with stable;
            [
              rustc
              cargo
              clippy
            ]
          );

        naersk' = naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        };
      in
      {
        packages = rec {
          default = naersk'.buildPackage {
            src = ./.;
          };
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            toolchain
            fenix.packages.${system}.rust-analyzer
          ];
        };
      }
    );
}

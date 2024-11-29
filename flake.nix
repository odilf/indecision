{
  description = "Application packaged using poetry2nix";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    pyproject-nix = {
      url = "github:nix-community/pyproject.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    uv2nix = {
      url = "github:adisbladis/uv2nix";
      inputs.pyproject-nix.follows = "pyproject-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        python = pkgs.python313;
      in
      {
        devShells = {
          default = {};

          impure = pkgs.mkShell {
          packages = [
            python
            pkgs.uv
          ];
          # shellHook = ''
          #   unset PYTHONPATH
          # '';
        };

          uv = pkgs.mkShell {
            packages = [ pkgs.uv ];
          };
        };
        legacyPackages = pkgs;
      }
    );
}

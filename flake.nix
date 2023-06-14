{
  description = "A way to power-off my pc remotely.";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in rec {
        devShell = with pkgs;
          mkShell {
            buildInputs = [
              cargo
              rustc
              rustfmt
              rust-analyzer
              alejandra
              efibootmgr
            ];
          };

        packages = rec {
          poweroff = pkgs.callPackage ./default.nix {};
          default = poweroff;
        };

        legacyPackages = packages;

        formatter = pkgs.alejandra;
      }
    );
}

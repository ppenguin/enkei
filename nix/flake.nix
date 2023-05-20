{
    description = "nix rust env";

    inputs.flake-utils.url = "github:numtide/flake-utils";

    outputs = { self, nixpkgs, flake-utils }:
        flake-utils.lib.eachDefaultSystem (system:
            let
                pkgs = nixpkgs.legacyPackages.${system};
                inherit (pkgs) lib;

           in {
                devShells = {
                     rust = import ./devshell-rust.nix { inherit pkgs; };
                };

            }
        );
}
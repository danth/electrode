{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self, nixpkgs, crane, utils, ... }:
    {
      nixosModules.electrode = import ./nixos.nix self;
    } //
    utils.lib.eachSystem [ "aarch64-linux" "i686-linux" "x86_64-linux" ] (system:
      let
        pkgs = import nixpkgs { inherit system; };
        craneLib = crane.lib.${system};

        commonArguments = rec {
          src = craneLib.cleanCargoSource ./.;

          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ gtk3 gtk-layer-shell pulseaudio ];

          cargoArtifacts = craneLib.buildDepsOnly commonArguments;
        };

      in {
        packages.default = craneLib.buildPackage commonArguments;

        checks.clippy = craneLib.cargoClippy (commonArguments // {
          cargoClippyExtraArgs = "-- --deny warnings";
        });

        devShells.default = with pkgs; mkShell {
          nativeBuildInputs = [ cargo ];
        };
      }
    );
}

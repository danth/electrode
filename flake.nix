{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    nix-filter.url = "github:numtide/nix-filter";

    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self, nixpkgs, crane, nix-filter, utils, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        craneLib = crane.lib.${system};

        commonArguments = rec {
          src = nix-filter.lib {
            root = ./.;
            include = with nix-filter.lib; [
              "Cargo.toml"
              "Cargo.lock"
              (inDirectory "src")
            ];
          };

          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ gtk3 gtk-layer-shell pulseaudio ];

          cargoArtifacts = craneLib.buildDepsOnly commonArguments;
        };

      in {
        packages.default = craneLib.buildPackage commonArguments;

        lib.makeBase16 = colors: craneLib.buildPackage (commonArguments // {
          postPatch = ''
            sed -i src/style.css \
              -e 's/#e0e0e0/#${colors.base00}/g' \
              -e 's/#d0d0d0/#${colors.base01}/g' \
              -e 's/#363636/#${colors.base05}/g'
          '';
        });

        checks.clippy = craneLib.cargoClippy (commonArguments // {
          cargoClippyExtraArgs = "-- --deny warnings";
        });

        devShells.default = with pkgs; mkShell {
          nativeBuildInputs = [
            cargo
            (writeShellScriptBin "rustfmt" ''
              PATH=${rustfmt.override { asNightly = true; }}/bin
              rustfmt src/*.rs
            '')
          ];
        };
      }
    );
}

{
  # Flake inputs
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-root.url = "github:srid/flake-root";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  # Flake outputs
  outputs = inputs @ {
    flake-parts,
    flake-root,
    treefmt-nix,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        flake-root.flakeModule
        treefmt-nix.flakeModule
      ];

      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      perSystem = {
        system,
        self',
        pkgs,
        config,
        lib,
        ...
      }: let
        # Custom rust toolchain from the `rust-toolchain.toml` file
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

        # Get the name and version of the rust crate
        crateMeta = craneLib.crateNameFromCargoToml {cargoToml = ./Cargo.toml;};

        commonArgs = {
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;

          # Executables build dependencies
          nativeBuildInputs = [];

          # Libraries build dependecies
          buildInputs = [];
        };

        # Build just the cargo dependencies
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # Build the rust crate itself
        day1 = craneLib.buildPackage (commonArgs // {inherit cargoArtifacts;});
      in {
        # Override the `pkgs` attribute
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            # Adds a `rust-bin` attibute to pkgs
            # that allows installing a rust toolchain
            # from a `rust-toolchain.toml` file
            (import inputs.rust-overlay)
          ];
        };

        # Format the whole project with `nix fmt`
        treefmt.config = {
          inherit (config.flake-root) projectRootFile;

          # To format nix files
          programs.alejandra.enable = true;

          # To format rust files
          programs.rustfmt.enable = true;

          # To format toml files
          programs.taplo.enable = true;
        };

        # Checks run with `nix flake check`
        checks = {
          # Check that the project builds
          # and the tests pass
          inherit day1;

          # Check that the doc builds without warnings
          day1-doc = craneLib.cargoDoc (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoDocExtraArgs = "--no-deps --document-private-items";
              RUSTDOCFLAGS = "--deny warnings";
            }
          );
        };

        # Packages built with `nix build`
        packages = {
          # Build the crate itself by default
          default = day1;

          # Build the crate's documentation
          doc = craneLib.cargoDoc (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoDocExtraArgs = "--document-private-items";
            }
          );
        };

        # Development shell with all needed dependencies
        # entered with `nix develop`
        devShells.default = craneLib.devShell {
          # Inherit inputs from checks
          checks = self'.checks;

          name = crateMeta.pname;

          # Export $FLAKE_ROOT in the development shell
          inputsFrom = [config.flake-root.devShell];

          # Provide treefmt's formaters in the development shell
          packages = lib.attrValues config.treefmt.build.programs;

          shellHook = ''
            # Environment variable to tell the system where to find the libraries
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${lib.makeLibraryPath commonArgs.buildInputs}"
          '';
        };
      };
    };
}

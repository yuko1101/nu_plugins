{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        custom-rust-bin = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = custom-rust-bin;
          rustc = custom-rust-bin;
        };
        p =
          name:
          rustPlatform.buildRustPackage rec {
            pname = name;
            version = (builtins.fromTOML (builtins.readFile "${src}/Cargo.toml")).package.version;

            src = "${self}/${pname}";

            cargoLock = {
              lockFile = "${src}/Cargo.lock";
            };
          };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            custom-rust-bin
          ];
        };
        packages = pkgs.lib.foldl' (acc: name: acc // { "${name}" = p name; }) { } [
          "nu_plugin_extra_parsers"
          "nu_plugin_extras"
          "nu_plugin_reverse_engineering"
        ];
      }
    );
}

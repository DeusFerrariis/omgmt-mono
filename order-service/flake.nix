{
  description = "A devShell example";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        name = "omgmt_order_service";
        src = ./.;
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        packages.default = 
          let
            version = "0.0.1a";
            inherit (pkgs) stdenv lib;
          in
          stdenv.mkDerivation {

            buildInputs = with pkgs; [
              openssl
              pkg-config
              eza
              fd
              rustup
              cargo
            ];
            name = "omgmt-order-service-${version}";
            src = ./.;
            dontUnpack = true;
            sourceRoot = ".";

            buildPhase = ''
              export RUSTUP_HOME=./.rustup
              rustup override set nightly
              cargo build
            '';
          };
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            openssl
            pkg-config
            eza
            fd
            rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
              extensions = [ "rust-src" ];
            })
          ];

          shellHook = ''
            alias ls=eza
            alias find=fd
          '';
        };
      }
    );
}

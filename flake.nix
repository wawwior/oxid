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
    inputs@{
      self,
      ...
    }:
    inputs.flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [ inputs.rust-overlay.overlays.default ];
        };
        lib = pkgs.lib;
      in
      {
        devShell = pkgs.mkShell rec {
          buildInputs = with pkgs; [
            (rust-bin.selectLatestNightlyWith (
              toolchain:
              toolchain.default.override {
                targets = [
                  "aarch64-unknown-linux-gnu"
                ];
              }
            ))
            rust-analyzer

            gcc

            jdk21
            jdt-language-server
            kotlin-language-server
            gradle
          ];

          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
        };
      }
    );
}

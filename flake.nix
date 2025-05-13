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
          config.allowUnsupportedSystem = true;
        };
        lib = pkgs.lib;
      in
      {
        devShell = pkgs.mkShell rec {
          buildInputs = with pkgs; [
            (rust-bin.selectLatestNightlyWith (
              toolchain:
              toolchain.default.override {
                extensions = [
                  "rust-src"
                  "rust-analyzer"
                  "llvm-tools-preview"
                ];
                targets = [
                  "x86_64-unknown-linux-gnu"
                  "x86_64-pc-windows-msvc"
                  "x86_64-apple-darwin"
                  "aarch64-unknown-linux-gnu" # currently not used
                  "aarch64-pc-windows-msvc"
                  "aarch64-apple-darwin"
                ];
              }
            ))

            cargo-xwin
            cargo-zigbuild

            zig
            clang

            jdk21

            gradle
            jdt-language-server

            libGL
            glfw-wayland-minecraft

            libpulseaudio
            openal
          ];

          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
        };
      }
    );
}

/**
  Noxwave GPUI development environment
*/
{
  description = "Noxwave GPUI development environment";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { nixpkgs, ... }:
    let
      system = "x86_64-linux";

      pkgs = import nixpkgs {
        inherit system;
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        name = "noxwave-dev";

        packages = with pkgs; [
          # Rust tools
          rustc
          cargo
          rustfmt
          clippy
          rust-analyzer
          rustPlatform.rustLibSrc

          # Build tools
          pkg-config
          cmake
          ninja

          # Font
          fontconfig
          freetype

          # Wayland
          wayland
          wayland-protocols
          wayland-scanner

          # X11
          libxcb
          xcbutil
          xcbutilwm
          xcbutilimage
          xcbutilkeysyms
          xcbutilrenderutil

          # Keyboard/input
          libxkbcommon

          # GPU
          vulkan-loader
          vulkan-headers
          vulkan-tools

          # OpenGL
          libGL
          mesa
        ];

        shellHook = ''
          export LD_LIBRARY_PATH=${
            pkgs.lib.makeLibraryPath [
              pkgs.wayland
              pkgs.libxkbcommon
              pkgs.libxcb
              pkgs.fontconfig
              pkgs.freetype
              pkgs.vulkan-loader
              pkgs.libGL
              pkgs.mesa
            ]
          }:$LD_LIBRARY_PATH

          export WINIT_UNIX_BACKEND=wayland
        '';
      };
    };
}

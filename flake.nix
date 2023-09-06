{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.05";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };

      system = "x86_64-linux";

      rust = pkgs.rust-bin.stable.latest.default.override { extensions = [ "rust-src" ]; };

      appRuntimeInputs = with pkgs; [
        udev
        vulkan-loader
        alsa-lib
        xorg.libX11
        xorg.libXtst
      ];
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        NIX_CFLAGS_LINK = "-fuse-ld=mold";

        nativeBuildInputs = with pkgs; [
          pkg-config
          clang
          udev
        ];

        buildInputs = with pkgs; [
          rust
          clang
          mold
          systemdMinimal

          libxkbcommon
          udev
          alsa-lib
          
          vulkan-tools
          vulkan-headers
          vulkan-validation-layers

          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          xorg.libX11
          xorg.libXtst
        ];

        shellHook = ''
          export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath appRuntimeInputs}"
          ln -fsT ${rust} ./.direnv/rust
        '';
      };
    };
}

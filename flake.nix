{
  inputs = { nixpkgs.url = "github:nixos/nixpkgs"; };

  outputs = { self, nixpkgs }:
    let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
    rust-toolchain = pkgs.symlinkJoin {
      name = "rust-toolchain";
      paths = [pkgs.rustc pkgs.cargo pkgs.rustPlatform.rustcSrc pkgs.rustfmt pkgs.clippy];
    };
    in {
      devShell.x86_64-linux =
        pkgs.mkShell {
          buildInputs = with pkgs; [
            rust-toolchain
            dbus
            pkg-config
            glibc
            cmake
            xorg.libxcb.dev
            alsa-lib.dev
            pipewire.dev
            clang12Stdenv
            rustPlatform.bindgenHook
            dbus.lib

            # Video/Audio data composition framework tools like "gst-inspect", "gst-launch" ...
            gst_all_1.gstreamer
            # Common plugins like "filesrc" to combine within e.g. gst-launch
            gst_all_1.gst-plugins-base
            # Specialized plugins separated by quality
            gst_all_1.gst-plugins-good
            gst_all_1.gst-plugins-bad
            gst_all_1.gst-plugins-ugly
            # Plugins to reuse ffmpeg to play almost every video format
            gst_all_1.gst-libav
            # Support the Video Audio (Hardware) Acceleration API
            gst_all_1.gst-vaapi
          ];
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          LIBCLANG_PATH = "${pkgs.llvmPackages_12.libclang.lib}/lib";
        };
    };
}

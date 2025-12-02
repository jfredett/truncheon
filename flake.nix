{
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
        fenix.url = "github:nix-community/fenix";
        devshell.url = "github:numtide/devshell";
        flake-parts.url = "github:hercules-ci/flake-parts";

        rust-manifest = {
            url = "https://static.rust-lang.org/dist/2025-11-10/channel-rust-nightly.toml";
            flake = false;
        };
    };

    outputs = { self, nixpkgs, fenix, rust-manifest, devshell, flake-parts, ... } @ inputs:
        flake-parts.lib.mkFlake { inherit inputs; } {
            imports = [
                devshell.flakeModule
            ];

            systems = [
                "x86_64-linux"
            ];

            perSystem = { pkgs, system, ... }: let
                rustpkg = (fenix.packages.${system}.fromManifestFile rust-manifest).completeToolchain;
                deps = with pkgs; rec {
                    bevy_dev = [
                            vulkan-loader
                            xorg.libX11
                            xorg.libXi
                            xorg.libXcursor
                            libxkbcommon
                            alsa-lib-with-plugins.dev
                            openssl.dev
                            wayland.dev
                            udev.dev
                            libudev-zero
                    ];
                    bevy = [
                        alsa-lib-with-plugins
                        libudev-zero
                        libxkbcommon
                        pkg-config
                        vulkan-loader
                        vulkan-tools
                        wayland
                        xorg.libX11
                        xorg.libXcursor
                        xorg.libXi
                        xorg.libXrandr
                        systemd.dev
                    ];
                    dev = [
                        bacon
                        cloc
                        gnuplot
                        imhex
                        perf
                        perf-tools
                        plantuml
                        timg
                    ] ++ ci;
                    ci = [
                        cargo-insta
                        cargo-llvm-cov
                        cargo-mutants
                        cargo-nextest
                        cargo-udeps
                        clang
                        just
                        libcxx
                        mold
                        rustpkg
                    ] ++ bevy;
                };
            in {
                packages = {
                    ci = pkgs.writeShellApplication {
                        name = "ci";
                        runtimeInputs = deps.ci;
                        text = /* bash */ ''
                            just ci
                            '';
                    };
                };
                devshells.default = with builtins; {
                    motd = "Hexy and we know it.";
                    packages = deps.dev;
                    env = [
                        { name = "LD_LIBRARY_PATH"; value = pkgs.lib.makeLibraryPath deps.bevy_dev; }
                        { name = "RUST_SRC_PATH"; value = head (filter (x: null != (match ".*rust-src.*" x)) rustpkg.paths); }
                        { name = "PKG_CONFIG_PATH"; value = pkgs.lib.makeSearchPath "lib/pkgconfig" deps.bevy_dev; }
                        { name = "RUSTFLAGS"; value = "-C link-arg=-fuse-ld=${pkgs.mold}/bin/mold"; }
                    ];
                };
            };
        };
}

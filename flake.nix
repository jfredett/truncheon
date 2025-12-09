{
    description =
        "A flake using nix-community's fenix wrapped with bevy-flake.";

    inputs = {
        nixpkgs.url = "nixpkgs/nixos-unstable";
        bevy-flake = {
            url = "github:swagtop/bevy-flake?ref=dev";
            inputs.nixpkgs.follows = "nixpkgs";
        };
        fenix = {
            url = "github:nix-community/fenix";
            inputs.nixpkgs.follows = "nixpkgs";
        };
    };

    outputs = { nixpkgs, bevy-flake, fenix, ... }: 
        let
            bf = bevy-flake.configure (
                { pkgs, default, ... }:
                {
                    src = ./.;
                    targetEnvironments = default.targetEnvironments // {
                        "wasm32-unknown-unknown" = {};
                    };
                    rustToolchainFor =
                        targets:
                        let
                            fx = fenix.packages.${pkgs.stdenv.hostPlatform.system};
                            channel = "latest"; # For nightly, use "latest".
                        in
                            fx.combine (
                                [ fx.${channel}.toolchain ] ++ map (target: fx.targets.${target}.${channel}.rust-std) targets
                            );
                }
            );

        in {
            inherit (bf) packages formatter;

            devShells = bf.eachSystem (system:
                let
                    pkgs = import nixpkgs { inherit system; };
                    deps = with pkgs; rec {
                        dev = [
                            # General stuff
                            bacon
                            cloc
                            gnuplot
                            imhex
                            perf
                            perf-tools
                            plantuml
                            graphviz
                            timg
                            # Bevy stuff
                            alsa-lib-with-plugins
                            alsa-lib-with-plugins.dev
                            libudev-zero
                            libxkbcommon
                            openssl
                            openssl.dev
                            pkg-config
                            systemd.dev
                            udev
                            udev.dev
                            vulkan-loader
                            vulkan-tools
                            wayland
                            wayland.dev
                            xorg.libX11
                            xorg.libXcursor
                            xorg.libXi
                            xorg.libXrandr
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
                        ];
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
                    default = pkgs.mkShell {
                        name = "truncheon";
                        packages = [
                            bf.packages.${system}.rust-toolchain
                            bf.packages.${system}.dioxus-cli
                            bf.packages.${system}.bevy-cli
                        ];
                        buildInputs = deps.dev;
                        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath deps.dev;
                        PKG_CONFIG_PATH = pkgs.lib.makeSearchPath "lib/pkgconfig" deps.dev;
                        RUSTFLAGS = "-C link-arg=-fuse-ld=${pkgs.mold}/bin/mold";
                    };
                });
        };
}

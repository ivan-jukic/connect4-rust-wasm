{
    description = "Connect4 Nix Shell";
    inputs = {
        nixpkgs.url = github:nixos/nixpkgs;
        flake-utils.url = "github:numtide/flake-utils";
        rust-overlay.url = "github:oxalica/rust-overlay";
    };
    outputs = inputs: with inputs;
        flake-utils.lib.eachDefaultSystem (system:
            let
                overlays = [ (import rust-overlay) ];
                pkgs = import nixpkgs {
                    inherit system overlays;
                };

                rustStableWithExtras = pkgs.rust-bin.stable.latest.default.override {
                    extensions = [ "rust-src" ];
                    targets = [ "wasm32-unknown-unknown" ];
                };

                nativeBuildInputs = with pkgs; [
                    rustStableWithExtras
                    cargo-make
                    cargo-sort
                    wasm-pack

                    # C compiler which includes a linker, also used by
                    # some common Rust packages that depend on C code 
                    gcc11
                    libiconv

                    # UI specific packages
                    yarn
                    nodejs
                    gnumake
                    elmPackages.elm
                    elmPackages.elm-format
                    elmPackages.elm-analyse
                    elmPackages.elm-test
                ];

                buildInputs = with pkgs; [
                    figlet
                    lolcat
                    which
                ];

                shellHook = ''
                    figlet "Connect4" | lolcat --freq 0.5
                '';
            in
            {
                devShell = pkgs.mkShell {
                    inherit nativeBuildInputs;
                    inherit buildInputs;
                    inherit shellHook;
                };
            }
        );
}

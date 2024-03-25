{
  description = "annotations";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay.url = "github:oxalica/rust-overlay";

    flake-utils.url = "github:numtide/flake-utils";

    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

    noto-emoji-metadata = {
      url = "github:googlefonts/emoji-metadata";
      flake = false;
    };

    # dictionaries.url = "ssh://git@gitlab.com:l4010/game-group/dictionaries.git";

  };

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, pre-commit-hooks
    , noto-emoji-metadata }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustPkg = pkgs.rust-bin.stable.latest;
        rustMinimalToolchain = rustPkg.minimal;
        rustDefaultToolchain = rustPkg.default;

        craneLib = crane.lib.${system}.overrideToolchain rustMinimalToolchain;

        my-crate = craneLib.buildPackage {
          src = ./.;

          doCheck = false;
        };

        noto-emoji-metadata-latest =
          "${noto-emoji-metadata}/emoji_15_1_ordering.json";

        pre-commit = pre-commit-hooks.lib."${system}".run;
      in {
        checks = {
          inherit my-crate;

          pre-commit = pre-commit {
            src = ./.;
            hooks = {
              nixfmt.enable = true;
              rustfmt.enable = true;
              # typos.enable = true;
            };
          };
        };

        # `nix build`
        packages.default = my-crate;

        # `nix run`
        apps.default = flake-utils.lib.mkApp { drv = my-crate; };

        # `nix develop`
        devShells.default = pkgs.mkShell {
          inherit (self.checks.${system}.pre-commit) shellHook;
          inputsFrom = builtins.attrValues self.checks;

          # OPENSSL_DIR = "${pkgs.openssl.dev}";
          # OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";

          noto_emoji_metadata_path = noto-emoji-metadata-latest;

          nativeBuildInputs = with pkgs; [
            # Development
            cargo-watch
            cargo-insta
            rustDefaultToolchain

            openssl.dev
            pkg-config

            # Debugging
            jless
          ];
        };

      });
}

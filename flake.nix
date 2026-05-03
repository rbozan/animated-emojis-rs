{
  description = "annotations";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    crane.url = "github:ipetkov/crane";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";

    noto-emoji-metadata = {
      url = "github:googlefonts/emoji-metadata";
      flake = false;
    };

    # dictionaries.url = "ssh://git@gitlab.com:l4010/game-group/dictionaries.git";

  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      rust-overlay,
      pre-commit-hooks,
      noto-emoji-metadata,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default;

        craneLib = crane.lib.${system}.overrideToolchain rustToolchain;

        my-crate = craneLib.buildPackage {
          src = ./.;

          doCheck = false;
        };

        noto-emoji-metadata-latest = "${noto-emoji-metadata}/emoji_17_0_ordering.json";

        pre-commit = pre-commit-hooks.lib."${system}".run;
      in
      {
        checks = {
          inherit my-crate;

          pre-commit = pre-commit {
            src = ./.;
            hooks = {
              nixfmt.enable = true;
              rustfmt.enable = true;
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

          noto_emoji_metadata_path = noto-emoji-metadata-latest;

          nativeBuildInputs = with pkgs; [
            # Development
            cargo-watch
            cargo-insta
            rustToolchain

            openssl.dev
            pkg-config

            # Debugging
            jless
          ];
        };

      }
    );
}

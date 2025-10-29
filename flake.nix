{
  description = "Basic Rust Cargo Template";

  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixos-unstable;
    crane.url = github:ipetkov/crane;
    rust-overlay = {
      url = github:oxalica/rust-overlay;
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, crane, rust-overlay }: 
    let
      supportedSystems = ["x86_64-linux"];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      nixpkgsFor = forAllSystems(system: import nixpkgs {
        inherit system;
        overlays = [self.overlays (import rust-overlay)];
      });
    in {
      overlays = final: prev: {

      };

      packages = forAllSystems(system: 
        let
          pkgs = nixpkgsFor.${system};
          craneLib = (crane.mkLib pkgs).overrideToolchain(
            p: p.rust-bin.stable.latest.default.override {
              targets = ["x86_64-unknown-linux-musl"];
            }
          );
        in { 
          default = craneLib.buildPackage {
            src = craneLib.cleanCargoSource ./. ;
            strictDeps = true;
            CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";
            CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
          };
        });

      devShells = forAllSystems(system: 
        let
          pkgs = nixpkgsFor.${system};
        in {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              rustc
              cargo
              rust-analyzer
              rustfmt
              treefmt
            ];
            shellHook = ''
              export PS1='[$PWD]\n❄ '
            '';
          };
        });
    };
}

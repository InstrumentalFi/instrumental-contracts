{
  description = "";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    cosmos = {
      url = "github:informalsystems/cosmos.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
      perSystem = { config, self', inputs', pkgs, system, ... }:
        let
          rust-toolchain =
            pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          rust =
            (inputs.crane.mkLib pkgs).overrideToolchain
              rust-toolchain;
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              inputs.rust-overlay.overlays.default
            ];
          };
          packages.default =
            pkgs.writeShellApplication {
              name = "build";
              runtimeInputs = [
                rust.rustc
                rust.cargo
                pkgs.binaryen
                pkgs.gnumake
                inputs.cosmos.packages.${system}.cosmwasm-check
              ];
              text = ''
                make build
                contracts=("distributor" "staking" "collector" "liquidator")
                mkdir --parents artifacts
                for contract in "''${contracts[@]}"; do
                  wasm-opt target/wasm32-unknown-unknown/release/"$contract".wasm -o artifacts/"$contracst".wasm -Os --signext-lowering
                  cosmwasm-check artifacts/"$contract".wasm
                done
              '';
            };
        };
    };
}

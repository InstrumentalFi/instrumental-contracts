{
  description = "Contracts";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
    cosmos = {
      url =
        "github:informalsystems/cosmos.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

  };

  outputs = inputs@{ flake-parts, self, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-darwin" ];
      perSystem = { config, self', inputs', pkgs, system, ... }: {
        _module.args.pkgs = import self.inputs.nixpkgs {
          inherit system;
          overlays = with self.inputs; [
            rust-overlay.overlays.default
          ];
        };

        packages =
          let
            rust-toolchain =
              pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
            rust = (self.inputs.crane.mkLib pkgs).overrideToolchain rust-toolchain;

            makeCosmwasmContract = name: rust: std-config:
              let binaryName = "${builtins.replaceStrings [ "-" ] [ "_" ] name}.wasm";
              in rust.buildPackage {
                src = ./.;
                version = "0.1";
                # LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath
                #     (with pkgs; [ stdenv.cc.cc.lib llvmPackages.libclang.lib ]);
                LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
                nativeBuildInputs = [
                  pkgs.binaryen
                  self.inputs.cosmos.packages.${system}.cosmwasm-check
                ];
                pname = name;
                DOCS_RS = 1;
                RUST_BACKTRACE = 1;
                cargoBuildCommand =
                  "cargo build --lib --release --target wasm32-unknown-unknown --locked --workspace --exclude instrumental-testing --package ${name} ${std-config}";
                RUSTFLAGS = "-C link-arg=-s";
                installPhaseCommand = ''
                  mkdir --parents $out/lib
                  # from CosmWasm/rust-optimizer
                  # --signext-lowering is needed to support blockchains runnning CosmWasm < 1.3. It can be removed eventually
                  wasm-opt target/wasm32-unknown-unknown/cosmwasm-contracts/${binaryName} -o $out/lib/${binaryName} -Os --signext-lowering
                  cosmwasm-check $out/lib/${binaryName}
                '';
              };
          in
          {
            staking = makeCosmwasmContract "staking" rust "";
          };
      };
    };
}

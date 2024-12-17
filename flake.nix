{
  description = "A Nix-flake development environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    ...
  }: let
    system = "x86_64-linux";
  in {
    devShells."${system}".default = let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
    in
      pkgs.mkShell {
        packages = with pkgs; [
          (rust-bin.nightly.latest.default.override
            {
              extensions = ["rust-src"];
            })
          cargo
          lldb_18
          nasm
          gdb
        ];
  
        shellHook = ''
          alias test='cargo run ./test.ir; nasm -felf64 ./build/out.asm; ld -o ./build/test.out ./build/out.o; ./build/test.out'
        '';

        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
  };
}

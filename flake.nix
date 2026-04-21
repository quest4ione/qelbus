{
  description = "A simulator for the subleq instruction set.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
  };

  outputs = { self, nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.${system}.default = pkgs.mkShellNoCC {
        packages = [
          pkgs.cargo
          pkgs.rust-analyzer
          pkgs.rustc
          pkgs.clippy
          pkgs.rustfmt
        ];
      };
    };
}

{
  description = "Dev shell for Faircamp build with vips support";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        foundation =
          if pkgs.stdenv.isDarwin
          then pkgs.darwin.apple_sdk.frameworks.Foundation
          else null;

        buildScript = pkgs.writeShellScriptBin "build" ''
          cargo install --features libvips --locked --path .
        '';
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.cargo
            pkgs.libiconv
            pkgs.cmake
            pkgs.vips
            buildScript
          ] ++ pkgs.lib.optional (foundation != null) foundation;
        };
      }
    );
}

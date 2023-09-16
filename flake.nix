{
  description = "Development Environment for Rusty Mount";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    shell-utils.url = "github:waltermoreira/shell-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };
  outputs =
    { self
    , nixpkgs
    , flake-utils
    , crane
    , shell-utils
    , rust-overlay
    }:
      with flake-utils.lib; eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        shell = shell-utils.myShell.${system};
        craneLib = crane.mkLib pkgs;
        rwam = craneLib.buildPackage {
          src = craneLib.cleanCargoSource ./.;
          buildInputs = with pkgs; [
            fuse3
          ];
        };
      in
      {
        packages.default = rwam;
        devShells.default = shell {
          name = "rust";
          buildInputs = [ pkgs.fuse3 ];
          packages = [
            (pkgs.rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
            })
            # Rusty Web requires fuse3 and the header files installed
            pkgs.fuse3
            pkgs.stdenv.cc.cc.lib

          ];
        };
      });
}

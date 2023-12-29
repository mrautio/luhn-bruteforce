{
  sources ? import ./nix/sources.nix,
  pkgs ? import sources.nixpkgs {},
}:

let
  paths = [
    pkgs.git
    pkgs.rustc
    pkgs.cargo
  ];
in
pkgs.mkShell {
  buildInputs = paths;
}

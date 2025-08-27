{pkgs}:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "gtk-template";
  version = "0.1.0";
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;
}

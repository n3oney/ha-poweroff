{
  rustPlatform,
  pkgs,
  makeWrapper,
}:
rustPlatform.buildRustPackage rec {
  pname = "ha-poweroff";
  version = "0.1.0";

  buildInputs = [pkgs.efibootmgr];

  nativeBuildInputs = [
    makeWrapper
  ];

  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;

  postInstall = ''
    wrapProgram $out/bin/${pname} \
      --prefix PATH : ${pkgs.efibootmgr}/bin
  '';

  meta.mainProgram = "ha-poweroff";
}

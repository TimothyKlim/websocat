let
  mozillaOverlay = import (builtins.fetchTarball "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ mozillaOverlay ]; };
  rust = pkgs.latest.rustChannels.stable.rust;
in
  pkgs.mkShell {
    buildInputs = [
      rust
      pkgs.pkgconfig
      pkgs.openssl
    ];

    shellHook = ''
      export PATH=$PWD/target/release:$PATH
      '';
  }

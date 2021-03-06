let
  # Mozilla Overlay
  moz_overlay = import (
    builtins.fetchTarball
      "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz"
  );

  nixpkgs = import (builtins.fetchTarball https://github.com/NixOS/nixpkgs/archive/21.05.tar.gz) {
    overlays = [ moz_overlay ];
    config = {};
  };

  rust = nixpkgs.latest.rustChannels.nightly.rust;

  frameworks = nixpkgs.darwin.apple_sdk.frameworks;
in
  with nixpkgs;

  stdenv.mkDerivation {
    name = "wilspi-rust-env";
    buildInputs = [ rust ];

    nativeBuildInputs = [
      clang
      llvm
      zsh
      vim
      postgresql
      libiconv
      diesel-cli
      openssl
      pkg-config
    ] ++ (
      lib.optionals stdenv.isDarwin [
        frameworks.Security
        frameworks.CoreServices
        frameworks.CoreFoundation
        frameworks.Foundation
      ]
    );

    # ENV Variables
    RUST_BACKTRACE = 1;
    SOURCE_DATE_EPOCH = 315532800;
    LIBCLANG_PATH = "${llvmPackages.libclang}/lib";

    # Post Shell Hook
    shellHook = ''
      echo "Using ${rust.name}"

    '' + (
      if !pkgs.stdenv.isDarwin then
        ""
      else ''
        # Cargo wasn't able to find CF during a `cargo test` run on Darwin.
        export NIX_LDFLAGS="-F${frameworks.CoreFoundation}/Library/Frameworks -framework CoreFoundation $NIX_LDFLAGS";
      ''
    );
  }

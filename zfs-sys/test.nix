with import <nixpkgs> {} ;
let
    testing = import "${toString <nixpkgs>}/nixos/lib/testing.nix" {
        inherit system;
    };

    zfs_sys_tests = stdenv.mkDerivation {
        name = "zfs_sys_tests";
        src = ./../.;
        phases = "unpackPhase buildCargo";
        buildCargo = ''
            CARGO_HOME=`pwd`/.cargo
            HOME=`pwd`
            export SSL_CERT_FILE=${cacert}/etc/ssl/certs/ca-bundle.crt
            cd zfs-sys
            cargo clean
            cargo test --no-run
            mkdir -p $out/bin
            cp target/debug/zfs_sys-* $out/bin/zfs_sys_test
        '';
        buildInputs = [ rustc cargo zfs pkgconfig];
    };

    machine = { config, pkgs, ... }: {
        config.networking.hostId="aaaaaaaa";
        config.boot.initrd.supportedFilesystems = [ "zfs" ];
        config.environment.systemPackages = [ zfs_sys_tests ];
    };

    testScript =
      ''
        $machine->waitForUnit("multi-user.target");
        $machine->succeed("${zfs_sys_tests}/bin/zfs_sys_test");
        $machine->shutdown;
      '';

in
    testing.makeTest { testScript = testScript;  machine = machine; }

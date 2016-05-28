#!/usr/bin/env bash
. /home/travis/.nix-profile/etc/profile.d/nix.sh
NIX_PATH=https://nixos.org/channels/nixos-unstable/nixexprs.tar.xz nix-build test.nix

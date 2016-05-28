#!/usr/bin/env bash
cargo test --no-run
if [[ $? != 0 ]]; then
  echo "Cargo build failed!"
  exit 1
fi

echo "Building VM tests"
(NIX_PATH=https://nixos.org/channels/nixos-unstable/nixexprs.tar.xz nix-build test.nix 2>&1)  > test.log

if [[ $? != 0 ]]; then
  echo "VM tests failed!"
  cat test.log
  exit 1
else
  echo "All VM tests passed"
  exit 0
fi

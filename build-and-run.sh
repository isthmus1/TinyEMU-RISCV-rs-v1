#!/usr/bin/env bash
# Build temu with the Rust in argo_rewrite/ linked in place of the C it replaces, then boot
# Fabrice Bellard's RISC-V 64 Linux image on it (Linux host). C-a x quits the emulator.
set -euo pipefail
cd "$(dirname "$0")"
cargo build --release --manifest-path argo_rewrite/Cargo.toml
make -j"$(nproc)" temu CFLAGS=-DCURL_DISABLE_TYPECHECK \
    LDFLAGS="-Wl,--gc-sections $PWD/argo_rewrite/target/release/libargo_rewrite.a -lpthread -ldl -lm"
IMG=diskimage-linux-riscv-2018-09-23
if [ ! -d "$IMG" ]; then
    curl -fL -o "$IMG.tar.gz" "https://bellard.org/tinyemu/$IMG.tar.gz"
    echo "808ecc1b32efdd76103172129b77b46002a616dff2270664207c291e4fde9e14  $IMG.tar.gz" | sha256sum -c -
    tar xzf "$IMG.tar.gz"
fi
cd "$IMG"
exec ../temu root-riscv64.cfg

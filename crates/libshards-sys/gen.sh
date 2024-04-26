#!/bin/sh

# nix-shell -p rust-bindgen
bindgen shards.h -o src/bindings.rs --allowlist-type '^Shards.*'

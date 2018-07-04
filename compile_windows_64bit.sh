#!/usr/bin/env bash

export PKG_CONFIG_ALLOW_CROSS=1
export PKG_CONFIG_PATH=/usr/i686-w64-mingw32/lib/pkgconfig
cargo build --target=x86_64-pc-windows-gnu --release
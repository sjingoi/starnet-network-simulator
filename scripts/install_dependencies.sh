#!/usr/bin/env bash
set -euo pipefail
NS3_VERSION="ns-3.45"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SRC="$ROOT/dependencies/ns-3"
PREFIX="$ROOT/dependencies/ns3-install"

git clone --depth=1 --branch "$NS3_VERSION" https://gitlab.com/nsnam/ns-3-dev.git "$SRC"

cmake -S "$SRC" -B "$SRC/cmake-build" -G Ninja \
  -DCMAKE_BUILD_TYPE=Release \
  -DCMAKE_INSTALL_PREFIX="$PREFIX" \
  -DNS3_EXAMPLES=OFF \
  -DNS3_TESTS=OFF \
  -DNS3_WARNINGS_AS_ERRORS=OFF
cmake --build "$SRC/cmake-build"
cmake --install "$SRC/cmake-build"
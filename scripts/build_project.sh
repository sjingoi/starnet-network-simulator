#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BUILD_DIR="$ROOT/build"
NS3_PREFIX="$ROOT/dependencies/ns3-install"

cmake -S "$ROOT" -B "$BUILD_DIR" \
  -DCMAKE_PREFIX_PATH="$NS3_PREFIX"
cmake --build "$BUILD_DIR"
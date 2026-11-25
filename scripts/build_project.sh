set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BUILD_DIR="$ROOT/build"

ns3_DIR="$ROOT/dependencies/ns3-install/lib/cmake"
cmake -S "$ROOT" -B "$BUILD_DIR"
cmake --build "$BUILD_DIR"
# Satellite Network Simulator

Satellite network simulation built on **ns-3** (C++) with a **Rust** orbital visualizer.

## Dependencies

Install yourself: C++20 compiler, CMake ≥ 3.22, Ninja, Git, Python 3, Rust ≥ 1.85, OpenGL/X11 dev libraries.

```bash
sudo apt install build-essential cmake ninja-build git python3 \
  libgl1-mesa-dev libx11-dev libxcursor-dev libxrandr-dev libxi-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Fetched automatically: ns-3 3.45, Corrosion 0.6.1, `glam`, `kiss3d`.

## Scripts

Run all scripts from the repository root.

### 1. Install dependencies

Clones and builds ns-3 into `dependencies/`. Only needed once.

```bash
bash scripts/install_dependencies.sh
```

### 2. Build

Compiles the C++ simulator and Rust library into `build/`.

```bash
bash scripts/build_project.sh
```

### 3. Run

Launches the simulator.

```bash
bash scripts/run_project.sh
```

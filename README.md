# Satellite Network Simulator

Satellite network simulation built on **ns-3** (C++) with a **Rust** orbital visualizer.

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

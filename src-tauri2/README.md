# CubeShuffle Tauri 2 app

Tauri 2 desktop and Android build for CubeShuffle. Uses the same WASM frontend as the legacy Tauri 1 app (`cube_shuffle-wasm`); builds for Windows, Linux, macOS, and Android from one crate.

## Prerequisites

- Rust (stable) with `wasm32-unknown-unknown`: `rustup target add wasm32-unknown-unknown`
- [Trunk](https://trunkrs.dev): `cargo install trunk`
- Tauri CLI v2: `cargo install tauri-cli --version 2`
- For Android: Android SDK/NDK; run `cargo tauri android init` once in this directory.

## Build

From this directory (`src-tauri2`):

- **Desktop (Windows/Linux/macOS):** `cargo tauri build`
- **Android APK:** after `cargo tauri android init`, run `cargo tauri android build --apk`

The frontend is built automatically by `beforeBuildCommand` (Trunk outputs into `websrc/`).

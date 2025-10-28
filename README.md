# Ferari
![CI Status](https://github.com/suvorovrain/Ferari/actions/workflows/ci.yml/badge.svg)

Fast Engine for Rendering Axonometric Rust-based Isometry.
## Description
An isometric engine that allows you to create simple games with static objects and mobs.
## Authors
* Rodion Suvorov. [GitHub](https://github.com/suvorovrain), [Contact](https://t.me/suvorovrain).
* Ilhom Kombaev. [GitHub](https://github.com/homka122), [Contact](https://t.me/homka122).
* Vyacheslav Kochergin. [GitHub](https://github.com/VyacheslavIurevich), [Contact](https://t.me/se4life).
* Dmitri Kuznetsov. [GitHub](https://github.com/f1i3g3), [Contact](https://t.me/f1i3g3).
## Dependencies
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install cargo
rustup default stable
```
## Usage
* Put your custom map information into `input.json` file in the root of repository, according format from example
* Compile & run via `cargo run`
## Development dependencies
```shell
rustup component add rustfmt
rustup component add clippy
cargo install cargo-tarpaulin
```
## Development
* View docs via `cargo doc` (use  --document-private-items if you want)
* Format your code via `cargo fmt`
* Everything else - in CI
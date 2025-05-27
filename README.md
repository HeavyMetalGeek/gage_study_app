# Gage Study

This is a tool for generating gage R&R results.

## Build from Source

Make sure you have the Rust toolchain installed:
[Install Rust](https://www.rust-lang.org/tools/install)

```shell
git clone https://github.com/HeavyMetalGeek/gage_study_app
```
```shell
cargo run -r --bin frontend
```

## Installation with Cargo

This will build and install the application as an executable.  Also requires
having the Rust toolchain installed.

```shell
cargo install --git https://github.com/HeavyMetalGeek/gage_study_app
```

## Download a Pre-Compiled Binary

***Coming Soon***

[Releases](https://github.com/HeavyMetalGeek/gage_study_app/releases)

## Use the Webapp

***Coming Soon***

## Reference

* [Base Library Documentation](https://heavymetalgeek.github.io/gage_study/)

## TODO

- [ ] Remove `FromData` in favor of `std::convert::From`
- [ ] Add UI element to notify user of deserialization errors

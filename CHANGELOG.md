# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1](https://github.com/fooker/photonic/compare/photonic-v0.1.0...photonic-v0.1.1) - 2025-09-21

### Added

- *(interface-grpc/client)* Use pyo3 async feature
- *(interface-grpc)* rework GRPC client and CLI and add python bindings
- *(interface-grpc)* separate client crate
- *(build)* Export cranLib instance
- *(examples)* Add audio reactive wled example
- *(build)* Enable release-plz workflow
- *(docs)* Add badges to readme

### Fixed

- *(deps)* update rust crate serde to v1.0.226 ([#170](https://github.com/fooker/photonic/pull/170))
- *(deps)* update rust crate clap to v4.5.48 ([#169](https://github.com/fooker/photonic/pull/169))
- *(deps)* update rust crate toml to v0.9.7 ([#167](https://github.com/fooker/photonic/pull/167))
- *(deps)* update rust crate toml to v0.9.6 ([#166](https://github.com/fooker/photonic/pull/166))
- *(deps)* update rust crate serde to v1.0.225 ([#165](https://github.com/fooker/photonic/pull/165))
- *(deps)* update rust crate serde to v1.0.223 ([#164](https://github.com/fooker/photonic/pull/164))
- *(deps)* update rust crate serde_json to v1.0.145 ([#163](https://github.com/fooker/photonic/pull/163))
- *(deps)* update rust crate serde_json to v1.0.144 ([#162](https://github.com/fooker/photonic/pull/162))
- *(deps)* update rust crate serde to v1.0.221 ([#161](https://github.com/fooker/photonic/pull/161))
- *(deps)* update rust crate serde_dhall to 0.13 ([#160](https://github.com/fooker/photonic/pull/160))
- *(deps)* update rust crate rumqttc to 0.25.0 ([#159](https://github.com/fooker/photonic/pull/159))
- *(deps)* update tonic monorepo to v0.14.2 ([#158](https://github.com/fooker/photonic/pull/158))
- *(deps)* update rust crate clap to v4.5.47 ([#157](https://github.com/fooker/photonic/pull/157))
- *(deps)* update rust crate pyo3 to 0.26 ([#156](https://github.com/fooker/photonic/pull/156))
- *(deps)* update rust crate mlua to v0.11.3 ([#155](https://github.com/fooker/photonic/pull/155))
- *(deps)* update rust crate ron to 0.11 ([#154](https://github.com/fooker/photonic/pull/154))
- *(deps)* update rust crate clap to v4.5.46 ([#153](https://github.com/fooker/photonic/pull/153))
- *(deps)* update rust crate serde_json to v1.0.143 ([#152](https://github.com/fooker/photonic/pull/152))
- *(deps)* update rust crate clap to v4.5.45 ([#150](https://github.com/fooker/photonic/pull/150))
- *(deps)* update rust crate clap to v4.5.44 ([#148](https://github.com/fooker/photonic/pull/148))
- *(deps)* update rust crate mlua to v0.11.2 ([#146](https://github.com/fooker/photonic/pull/146))
- *(deps)* update tonic monorepo to v0.14.1 ([#145](https://github.com/fooker/photonic/pull/145))
- *(deps)* update rust crate clap to v4.5.43 ([#144](https://github.com/fooker/photonic/pull/144))
- *(deps)* update rust crate serde_json to v1.0.142 ([#142](https://github.com/fooker/photonic/pull/142))
- *(deps)* update rust crate toml to v0.9.5 ([#143](https://github.com/fooker/photonic/pull/143))
- *(deps)* update rust crate clap to v4.5.42 ([#141](https://github.com/fooker/photonic/pull/141))
- *(deps)* update rust crate toml to v0.9.3 ([#139](https://github.com/fooker/photonic/pull/139))
- *(deps)* update tonic monorepo to 0.14.0 ([#138](https://github.com/fooker/photonic/pull/138))
- *(deps)* update rust crate serde_json to v1.0.141 ([#135](https://github.com/fooker/photonic/pull/135))
- *(deps)* update rust crate mlua to v0.11.1 ([#134](https://github.com/fooker/photonic/pull/134))
- *(deps)* update rust crate mlua to 0.11.0 ([#133](https://github.com/fooker/photonic/pull/133))
- *(deps)* update rust crate toml to v0.9.2 ([#132](https://github.com/fooker/photonic/pull/132))
- *(deps)* update rust crate toml to v0.9.1 ([#131](https://github.com/fooker/photonic/pull/131))
- *(deps)* update rust crate clap to v4.5.41 ([#130](https://github.com/fooker/photonic/pull/130))
- *(deps)* update rust crate toml to 0.9 ([#129](https://github.com/fooker/photonic/pull/129))
- *(deps)* update rust crate pyo3 to v0.25.1 ([#123](https://github.com/fooker/photonic/pull/123))
- *(deps)* update rust crate clap to v4.5.40 ([#122](https://github.com/fooker/photonic/pull/122))
- *(deps)* update rust crate pyo3 to 0.25
- *(deps)* update rust crate clap to v4.5.39 ([#117](https://github.com/fooker/photonic/pull/117))
- *(deps)* update rust crate toml to v0.8.23 ([#119](https://github.com/fooker/photonic/pull/119))
- *(deps)* update rust crate cpal to 0.16 ([#121](https://github.com/fooker/photonic/pull/121))
- *(deps)* update rust crate parking_lot to v0.12.4 ([#118](https://github.com/fooker/photonic/pull/118))
- *(deps)* update rust crate clap to v4.5.39 ([#117](https://github.com/fooker/photonic/pull/117))
- *(deps)* update rust crate mlua to v0.10.5 ([#114](https://github.com/fooker/photonic/pull/114))
- *(deps)* update rust crate clap to v4.5.38 ([#112](https://github.com/fooker/photonic/pull/112))
- *(deps)* update rust crate mlua to v0.10.4 ([#110](https://github.com/fooker/photonic/pull/110))
- *(deps)* update tonic monorepo to v0.13.1 ([#109](https://github.com/fooker/photonic/pull/109))
- *(deps)* update rust crate nix to v0.30.1 ([#108](https://github.com/fooker/photonic/pull/108))
- *(deps)* update rust crate nix to 0.30
- *(deps)* update rust crate ron to 0.10
- *(deps)* update tonic monorepo to 0.13.0
- *(deps)* update rust crate bytes to v1.10.1 ([#93](https://github.com/fooker/photonic/pull/93))
- *(deps)* update rust crate serde_json to v1.0.140 ([#92](https://github.com/fooker/photonic/pull/92))
- *(deps)* update rust crate pyo3 to v0.23.5 ([#89](https://github.com/fooker/photonic/pull/89))
- *(deps)* update rust crate clap to v4.5.31 ([#88](https://github.com/fooker/photonic/pull/88))
- *(deps)* update rust crate serde_json to v1.0.139 ([#86](https://github.com/fooker/photonic/pull/86))
- *(deps)* update rust crate serde to v1.0.218 ([#87](https://github.com/fooker/photonic/pull/87))
- *(deps)* update rust crate clap to v4.5.30 ([#84](https://github.com/fooker/photonic/pull/84))
- *(deps)* update rust crate prost to v0.13.5 ([#82](https://github.com/fooker/photonic/pull/82))
- *(deps)* update rust crate clap to v4.5.29 ([#81](https://github.com/fooker/photonic/pull/81))
- *(deps)* update rust crate toml to v0.8.20 ([#77](https://github.com/fooker/photonic/pull/77))
- *(deps)* update rust crate mlua to v0.10.3 ([#72](https://github.com/fooker/photonic/pull/72))
- *(deps)* update rust crate clap to v4.5.28 ([#76](https://github.com/fooker/photonic/pull/76))
- *(deps)* update rust crate bytes to v1.10.0 ([#75](https://github.com/fooker/photonic/pull/75))
- *(deps)* update rust crate serde_json to v1.0.138 ([#73](https://github.com/fooker/photonic/pull/73))
- *(deps)* update rust crate clap to v4.5.27 ([#70](https://github.com/fooker/photonic/pull/70))
- *(deps)* update rust crate serde_json to v1.0.137 ([#69](https://github.com/fooker/photonic/pull/69))
- *(deps)* update rust crate serde_json to v1.0.136 ([#68](https://github.com/fooker/photonic/pull/68))
- *(core)* Add error context
- *(deps)* update rust crate clap to v4.5.26
- *(docs)* Update flake dependencies
- *(deps)* update rust crate serde_json to v1.0.135 ([#61](https://github.com/fooker/photonic/pull/61))
- *(deps)* update rust crate serde to v1.0.217 ([#57](https://github.com/fooker/photonic/pull/57))
- *(deps)* update rust crate serde_json to v1.0.134 ([#55](https://github.com/fooker/photonic/pull/55))
- *(deps)* update rust crate serde to v1.0.216 ([#53](https://github.com/fooker/photonic/pull/53))
- *(deps)* update rust crate clap to v4.5.22 ([#48](https://github.com/fooker/photonic/pull/48))
- *(deps)* update rust crate bytes to v1.9.0 ([#44](https://github.com/fooker/photonic/pull/44))
- *(deps)* update rust crate serde_json to v1.0.133
- *(deps)* update rust crate serde to v1.0.215 ([#40](https://github.com/fooker/photonic/pull/40))
- *(deps)* update rust crate clap to v4.5.21 ([#38](https://github.com/fooker/photonic/pull/38))
- *(deps)* update rust crate mlua to v0.10.1
- *(deps)* update rust crate serde to v1.0.214
- *(output-net/wled)* fix sending
- *(deps)* update rust crate mlua to 0.10.0
- *(deps)* Remove unused dependencies
- *(deps)* add CI check for unused dependencies
- *(build)* Let renovate-bot use semantic commits
- *(docs)* Add related crates list
- *(build)* Add crate metadata and missing monorepo versioning

### Other

- *(deps)* update rust crate anyhow to v1.0.100 ([#168](https://github.com/fooker/photonic/pull/168))
- *(deps)* update rust crate async-trait to v0.1.89 ([#151](https://github.com/fooker/photonic/pull/151))
- *(deps)* update rust crate anyhow to v1.0.99 ([#149](https://github.com/fooker/photonic/pull/149))
- *(deps)* update rust crate tokio to v1.47.1 ([#140](https://github.com/fooker/photonic/pull/140))
- *(deps)* update rust crate tokio to v1.47.0 ([#137](https://github.com/fooker/photonic/pull/137))
- *(deps)* update rust crate rand to v0.9.2 ([#136](https://github.com/fooker/photonic/pull/136))
- *(deps)* update rust crate tokio to v1.46.1 ([#128](https://github.com/fooker/photonic/pull/128))
- *(deps)* update rust crate tokio to v1.46.0 ([#127](https://github.com/fooker/photonic/pull/127))
- *(deps)* Update nix dependencies
- *(deps)* update rust crate tokio to v1.45.1 ([#116](https://github.com/fooker/photonic/pull/116))
- *(deps)* update rust crate tokio to v1.45.0 ([#111](https://github.com/fooker/photonic/pull/111))
- *(deps)* update rust crate rand to 0.9
- *(deps)* Update cargo deps
- *(deps)* Update nix and cargo deps
- *(deps)* update rust crate async-trait to v0.1.87 ([#91](https://github.com/fooker/photonic/pull/91))
- *(deps)* update rust crate anyhow to v1.0.97 ([#90](https://github.com/fooker/photonic/pull/90))
- *(deps)* update rust crate anyhow to v1.0.96 ([#85](https://github.com/fooker/photonic/pull/85))
- *(ci)* Remove EOL magic nix cache
- *(deps)* update rust crate async-trait to v0.1.86 ([#74](https://github.com/fooker/photonic/pull/74))
- *(deps)* enable auto-merge for renovatebot updates
- *(deps)* Update nix dependencies
- *(deps)* update rust crate tokio to v1.43.0
- *(deps)* update rust crate async-trait to v0.1.85
- *(deps)* update rust crate async-trait to v0.1.84
- *(deps)* update rust crate anyhow to v1.0.95 ([#56](https://github.com/fooker/photonic/pull/56))
- *(deps)* migrate deny.toml
- *(deps)* dependency updates
- *(deps)* update nixpkgs
- *(deps)* update flakes
- *(deps)* update rust crate anyhow to v1.0.94 ([#47](https://github.com/fooker/photonic/pull/47))
- *(deps)* update rust crate tokio to v1.42.0 ([#46](https://github.com/fooker/photonic/pull/46))
- *(deps)* update rust crate anyhow to v1.0.93 ([#36](https://github.com/fooker/photonic/pull/36))
- *(deps)* update rust crate tokio to v1.41.1
- *(deps)* update flakes
- *(deps)* Update Rust crate anyhow to v1.0.91
- *(deps)* Update Rust crate serde to v1.0.213

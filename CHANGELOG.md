# Changelog

All notable changes to this project will be documented in this file.

## [0.3.0] - 2025-02-09

### 🚀 Features

- First step to run as Docker container
- Add help text on bottom to show keys
- Add tracing logging to logfile
- Logging with tracing crate
- Swap out zmq/async_zmq for zeromq

### 🐛 Bug Fixes

- Selection with multiple tabs
- Restore terminal at quit
- Probes per tab if there is only one tab
- Replace deprecated functions
- Deprecated functions in rand crate

### 💼 Other

- Use zigbuild
- Try parallel multi-arch builds
- Make visible to cargo dist
- Don't build for musl
- Start building for Windows
- *(deps)* Bump tokio from 1.35.0 to 1.35.1 (#12)
- *(deps)* Bump serde from 1.0.193 to 1.0.194 (#15)
- *(deps)* Bump serde from 1.0.194 to 1.0.195 (#16)
- *(deps)* Bump regex from 1.10.2 to 1.10.3 (#17)
- *(deps)* Bump serde from 1.0.195 to 1.0.196 (#18)
- *(deps)* Bump itertools from 0.12.0 to 0.12.1 (#19)
- *(deps)* Bump toml from 0.8.8 to 0.8.9 (#20)
- *(deps)* Bump ratatui from 0.25.0 to 0.26.0 (#21)
- *(deps)* Bump tokio from 1.35.1 to 1.36.0 (#22)
- *(deps)* Bump toml from 0.8.9 to 0.8.10 (#23)
- *(deps)* Bump ratatui from 0.26.0 to 0.26.1 (#24)
- *(deps)* Bump serde from 1.0.196 to 1.0.197 (#25)
- *(deps)* Bump toml from 0.8.10 to 0.8.11 (#26)
- *(deps)* Bump toml from 0.8.11 to 0.8.12 (#27)
- Update cargo-dist to 0.12.0
- Remove musl build again, forgot that doesn't work
- *(deps)* Bump regex from 1.10.3 to 1.10.4 (#28)
- *(deps)* Bump tokio from 1.36.0 to 1.37.0 (#29)
- *(deps)* Bump ratatui from 0.26.1 to 0.26.2 (#30)
- *(deps)* Bump serde from 1.0.197 to 1.0.198 (#31)
- *(deps)* Bump serde from 1.0.198 to 1.0.199 (#32)
- *(deps)* Bump serde from 1.0.199 to 1.0.200 (#33)
- *(deps)* Bump serde from 1.0.200 to 1.0.201 (#34)
- *(deps)* Bump toml from 0.8.12 to 0.8.13 (#35)
- *(deps)* Bump serde from 1.0.201 to 1.0.202 (#36)
- *(deps)* Bump itertools from 0.12.1 to 0.13.0 (#37)
- *(deps)* Bump serde from 1.0.202 to 1.0.203 (#39)
- *(deps)* Bump tokio from 1.37.0 to 1.38.0 (#40)
- *(deps)* Bump toml from 0.8.13 to 0.8.14 (#41)
- *(deps)* Bump regex from 1.10.4 to 1.10.5 (#42)
- *(deps)* Bump ratatui from 0.26.3 to 0.27.0 (#43)
- *(deps)* Bump serde from 1.0.203 to 1.0.204 (#44)
- *(deps)* Bump tokio from 1.38.0 to 1.38.1 (#45)
- *(deps)* Bump toml from 0.8.14 to 0.8.15 (#46)
- *(deps)* Bump tokio from 1.38.1 to 1.39.1 (#47)
- *(deps)* Bump toml from 0.8.15 to 0.8.16 (#48)
- *(deps)* Bump tokio from 1.39.1 to 1.39.2 (#49)
- *(deps)* Bump toml from 0.8.16 to 0.8.17 (#50)
- *(deps)* Bump crossterm from 0.27.0 to 0.28.0 (#51)
- *(deps)* Bump toml from 0.8.17 to 0.8.19 (#52)
- *(deps)* Bump regex from 1.10.5 to 1.10.6 (#54)
- *(deps)* Bump ratatui from 0.27.0 to 0.28.0 (#55)
- *(deps)* Bump serde from 1.0.204 to 1.0.205 (#56)
- *(deps)* Bump serde from 1.0.205 to 1.0.207 (#58)
- *(deps)* Bump serde from 1.0.207 to 1.0.208 (#59)
- *(deps)* Bump tokio from 1.39.2 to 1.39.3 (#60)
- *(deps)* Bump serde from 1.0.208 to 1.0.209 (#61)
- *(deps)* Bump ratatui from 0.28.0 to 0.28.1 (#62)
- *(deps)* Bump tokio from 1.39.3 to 1.40.0 (#63)
- *(deps)* Bump serde from 1.0.209 to 1.0.210 (#64)
- *(deps)* Bump regex from 1.10.6 to 1.11.0 (#65)
- *(deps)* Bump ratatui from 0.28.1 to 0.29.0 (#66)
- *(deps)* Bump serde from 1.0.210 to 1.0.213 (#67)
- *(deps)* Bump tokio from 1.40.0 to 1.41.0 (#68)
- *(deps)* Bump regex from 1.11.0 to 1.11.1 (#69)
- *(deps)* Bump serde from 1.0.213 to 1.0.214 (#70)
- *(deps)* Bump tokio from 1.41.0 to 1.41.1 (#71)
- *(deps)* Bump serde from 1.0.214 to 1.0.215 (#72)
- *(deps)* Bump tokio from 1.41.1 to 1.42.0 (#73)
- *(deps)* Bump serde from 1.0.215 to 1.0.216 (#74)
- *(deps)* Bump argh from 0.1.12 to 0.1.13 (#75)
- *(deps)* Bump serde from 1.0.216 to 1.0.217 (#76)
- *(deps)* Bump itertools from 0.13.0 to 0.14.0 (#77)
- *(deps)* Bump tokio from 1.42.0 to 1.43.0 (#78)
- *(deps)* Bump rand from 0.8.5 to 0.9.0 (#79)
- *(deps)* Bump toml from 0.8.19 to 0.8.20 (#81)

### 🚜 Refactor

- Simplify code
- Switch to crossterm from termion for Windows support
- Name stuff better
- Use async instead of threads

### 📚 Documentation

- Auto-generated Changelog
- Update readme description
- Add configuration section
- Fix table
- Add more badges to readme

### ⚙️ Miscellaneous Tasks

- Add MIT license
- Update deps
- Upgrade to ratatui
- Clippy fixes
- Update edition to 2021
- Add clippy and fmt jobs
- Remove unused import
- Fix CI
- Fix CI
- Fix CI
- Fix CI
- Fix CI
- Add more targets
- Keep artifacts
- Set up release stuff
- Don't publish on crates.io
- Refresh release config
- Update deps daily
- Allow dirty cargo dist config
- Add CI workflow
- Fix runs-on
- Add cargo-release config
- Release probe version 0.2.0
- Update cargo-dist to 0.6.0
- Don't update actions; use compatible actions only
- Update cargo dist to 0.6.2
- Update cargo dist
- Add unit test job
- Release probe version 0.2.1
- Fix clippy lint
- Release probe version 0.2.2
- Release probe version 0.2.2
- Remove dead code
- Update deps to fix build
- Tweak actions with cache etc
- Fix pedantic clippy
- Replace unwraps with expect for slightly better error handling
- All the release management stuff
- Rename crate to publish it
- Release 0.3.0

<!-- generated by git-cliff -->

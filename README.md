# gdua - Graphical Disk Usage Analyzer

Graphical disk usage analyzer, inspired by [GNOME/baobab]

gdua is an experimental project that demonstrates building GUI application built with [web-view] and [yew].

**Goal**: Add a pie chart of disk usage, like baobab.

## Usage

Requirements: [cargo-web]

```bash
cargo web deploy -p gdua-ui
cargo build --bin gdua
cargo run --bin gdua -- /path/to/analyze
```

---

_gdua_ is distributed under the terms of both the [MIT license](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE). See [COPYRIGHT](COPYRIGHT) for details.

[gnome/baobab]: https://en.wikipedia.org/wiki/Disk_Usage_Analyzer
[web-view]: https://github.com/Boscop/web-view
[yew]: https://github.com/DenisKolodin/yew
[cargo-web]: https://github.com/koute/cargo-web

# <img src="assets/icon.png" alt="" width="48" height="48" align="top"> Minesweeper

[![CI](https://github.com/mmsaffari/minesweeper-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/mmsaffari/minesweeper-rust/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/mmsaffari/minesweeper-rust)](https://github.com/mmsaffari/minesweeper-rust/releases/latest)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](#license)

The classic Windows Minesweeper, rebuilt in Rust with [macroquad](https://macroquad.rs/).
Written as a project for learning Rust.

## Download

Get the latest Windows build from [Releases](https://github.com/mmsaffari/minesweeper-rust/releases/latest):
unzip it and run `minesweeper.exe`.

The executable isn't code-signed, so Windows SmartScreen will warn the first time
("Windows protected your PC" → **More info** → **Run anyway**). On a PC with
Smart App Control turned on, Windows may refuse to run it at all; build it from source instead.

## How to play

Open every cell that isn't a mine. A number tells you how many of the 8 surrounding cells are mines.
The first click is never a mine.

| Input | Action |
|---|---|
| Left click | Open a cell (an empty area opens up by itself) |
| Right click | Cycle a mark: flag → question mark → none |
| Both buttons, or middle click, on a number | **Chord**: if that number's mines are all flagged, open its other neighbours |
| Click the smiley, or `F2` | New game |
| `1` / `2` / `3` | Beginner (9×9, 10 mines) / Intermediate (16×16, 40) / Expert (30×16, 99) |
| `H` | **Hint**: outlines a cell that is certainly safe (green) or certainly a mine (red), worked out only from what you can see |
| `Ctrl+C` | Copy the board as text (`#` hidden, `F` flag, `?` mark, `.` empty, numbers), e.g. to ask someone for help |
| `F1` | About |
| **Game** / **Help** menus | All of the above, plus your best time for each difficulty |

The counter on the left shows mines minus flags; the timer on the right stops at 999.
When you win, every mine is flagged and the time is saved if it's a new record.

## Build from source

You need [Rust](https://rustup.rs/) (stable).

```bash
cargo run --release
```

On **Linux** (including WSL), macroquad also needs some system libraries first:

```bash
sudo apt install libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev
```

Under **WSL**, the window opens through WSLg. If the game uses a lot of CPU, Mesa is
probably drawing in software; `export GALLIUM_DRIVER=d3d12` makes it use the GPU.

## Development

```bash
cargo test                              # all tests (game rules, layout, input, best times)
cargo lint                              # clippy on everything, warnings included (alias in .cargo/config.toml)
cargo fmt                               # format (settings in rustfmt.toml)
```

Every push and pull request runs formatting, clippy (`-D warnings`) and the tests on Linux and Windows.
Pushing a `v*` tag builds the Windows release and publishes it.

### Layout

```text
src/
├── main.rs      window setup and the frame loop
├── app.rs       the running program: game, layout, menu, input routing
├── logic/       the game rules, with no graphics at all: board, cells, mines, winning/losing
├── ui/          everything on screen: layout, drawing, mouse/keyboard, header, menu
├── scores/      best times, saved as JSON in the user's data folder
└── solver/      the board as the player sees it: the text export and the hint
```

One type (or one function) per file. Logic and layout code is unit-tested; only the
drawing itself (which needs a window) isn't.

Best times are stored in `~/.local/share/minesweeper-rust/best_times.json` on Linux and
`%APPDATA%\minesweeper-rust\data\best_times.json` on Windows.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

The embedded fonts have their own license, the SIL Open Font License 1.1:
[Roboto](assets/fonts/Roboto-LICENSE.txt) and [DSEG7](assets/fonts/DSEG-LICENSE.txt).

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
in this work by you, as defined in the Apache-2.0 license, shall be dual licensed as above,
without any additional terms or conditions.

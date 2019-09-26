# Minesweeper in Rust

This is a Minesweeper game I made in Rust as a learning exercise.

Uses [original MS Windows Minesweeper graphics ripped by Black Squirrel](https://www.spriters-resource.com/pc_computer/minesweeper/).

## Build instructions

1. [Install Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. Clone this repository: `git clone https://github.com/HactarCE/Minesweeper.git && cd Minesweeper`
3. Run `cargo run`

## Usage instructions

```
Usage: ./minesweeper [options]

Options:
    -h, --help          print this help text
    -1, --beginner      play at BEGINNER difficulty (9x9 with 10 mines)
    -2, --intermediate  play at INTERMEDIATE difficulty (16x16 with 40 mines)
    -3, --expert        play at EXPERT difficulty (16x30 with 99 mines)
    -x, --width WIDTH   play with a custom board width
    -y, --height HEIGHT play with a custom board height
    -m, --mines MINE_COUNT
                        play with a custom number of mines
    -d, --density MINE_DENSITY
                        play with a custom mine density

Board size and mine count/density must be specified. Any of the
three preset difficulties specifies both; these can be overridden
manually or specified outright using the other arguments.

The three difficulties (-1, -2, and -3) are mutually exclusive.

Mine count and mine density are mutually exclusive.

If width is specified but not height (or vice versa), the board
is assumed to be square.
```

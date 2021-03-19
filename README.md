# Shazam
> 📖 An automatic way to make homework a cakewalk

## What is Shazam?
Shazam is a rust utility that, given some user input,
pulls answers to questions from Slader automatically.
Unfortunately, not every textbook is available on Slader,
and not every answer is available in a text format.
This will also not give you steps in answering the questions
if they are not provided in "final answer" section in Slader.

## Prerequisites
+ GNU Troff (`groff`)
+ LaTeX (`pdflatex`)
+ Rust (`rustup` or `rustc`/`cargo`)

## Installation
```sh
$ git clone https://github.com/cadenhenrich/shazam.git
$ cd shazam
$ cargo build --release
$ mkdir -p ~/.local/bin
$ cp target/release/shazam ~/.local/bin
```
> NOTE: This relies on having `~/.local/bin` in your `$PATH`

## Usage
< TBD >

## License
This project is licensed under the GNU GPLv3. See `LICENSE` for more info

<img src="./assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2022
![Language](https://badgen.net/badge/Language/Rust/orange)

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).
<!--- advent_readme_stars table --->
## 2022 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2022/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2022/day/2) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2022/day/3) | ⭐ | ⭐ |
| [Day 4](https://adventofcode.com/2022/day/4) | ⭐ | ⭐ |
| [Day 5](https://adventofcode.com/2022/day/5) | ⭐ | ⭐ |
| [Day 6](https://adventofcode.com/2022/day/6) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

**NOTE**

This only can handle 2022 due to a limitation of the aoc-runner

---
## Dependancies

Add Clippy for Cargo.

## Run

```bash
cargo run
```

## Run with logging

```sh
RUST_LOG=info cargo run
```

## Run in test mode

```sh
cargo test
```

## Use the cargo runner

```sh
cargo aoc
```

## Setting up the CLI

You will need to find your session token for the AoC in order for cargo-aoc to work. Thankfully, finding your token is easy since it is stored in your Browser's cookies. Open up the devtools of your browser, and then :

* Firefox: "Storage" tab, Cookies, and copy the "Value" field of the session cookie.
* Google Chrome / Chromium: "Application" tab, Cookies, and copy the "Value" field of the session cookie.
Once you have it, simply run : cargo aoc credentials -s {token}

You're now ready to start coding !

NOTE: If for some reason your token has changed, dont forget to change it back.

cargo aoc credentials will show the currently stored user token

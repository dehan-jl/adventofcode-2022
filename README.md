# Advent of Code 2022
A (mostly) Rust 🦀 set of solutions to the [Advent of Code](https://adventofcode.com/) puzzles for 2022.

## Instructions
Run a day using `cargo run --bin <day>` to run an unoptimized build with example input. Run a day using `cargo run --release --bin <day> -- --real`.

The runner expects that the example input has been provided. It will try to automatically the download the real input. Add the `AOC_SESSION` environment variable using:
```bash
export AOC_SESSION=<your session cookie> #Unix
```


## Folder Structure
```
.
├── alternate
│   └── dayX # Alternate/non-refactored/different language solutions to day X
├── inputs
│   ├── example # example puzzle inputs
│   │   └── dayX.txt
│   └── real # real puzzle inputs
│       └── dayX.txt
└── src
    ├── bin
    │   └── dayX.rs # solution for day X
    ├── lib.rs # helper library
    └── main.rs # main project binary, does nothing right now
```

## Helpful Resources
- A wonderful [series of articles](https://fasterthanli.me/series/advent-of-code-2022) explaing Rust 🦀 using Advent of Code by [@fasterthanlime](https://github.com/fasterthanlime)
- For some genuinely impressive solutions: [@AxlLind](https://github.com/AxlLind/AdventOfCode2022)
- Some more and pretty good solutions: [@jontmy](https://github.com/jontmy/aoc-rust)
- Another good set of solutions: [@Ummon](https://github.com/Ummon/AdventOfCode2022)

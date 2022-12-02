# Advent Of Code 2015

https://adventofcode.com/2015

Retrieve your daily input using your session cookie via:
```sh
cargo run --release -- --day <day> download --session <session>
```
The data is put in data/ and used directly at compile time.

To compute the  execution time, use:
```sh
cargo run --release -- --day <day> execute --part <part>
```

To measure execution time for a particular day, use:
```sh
cargo run --release -- --day <day> benchmark --number <number> --current
```

## [Day 01: Not Quite Lisp](https://adventofcode.com/2015/day/1)

part 2: reply once reached
[Code](./src/solutions/day01.rs)

## [Day 02: I Was Told There Would Be No Math](https://adventofcode.com/2015/day/2)

70µs per part.
[Code](./src/solutions/day02.rs)

## [Day 03: Perfectly Spherical Houses in a Vacuum](https://adventofcode.com/2015/day/3)

maybe a simple 2D bool grid is more efficient than a dump hashset.
120/150µs per part.
[Code](./src/solutions/day03.rs)

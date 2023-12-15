# Aoc Elixir

Small Elixir project for generating sub files for [Advent of Code](https://adventofcode.com/).

---

First run the init task to generate all aoc files:

```
mix aoc_init
```

Update `input/dayXX/input.txt` and `input/dayXX/input.test.txt` with the given
input vales. The `.test.txt` file is used for the problem examples that are
usually given with each test.

Once these input files are updated you can develop your solution in
`lib/aoc/dayXX.ex`. Each part is in a separate function and just takes the input
file path as an argument.

Tests can be executed as follows:

```
# Run all tests
mix test
# Run a specific day
mix test test/aoc/dayXX_test.exs
```

Solutions can be executed with mix tasks:

```
mix dXX.pY

# For example
mix d04.p1
mix d11.p2
```

*Update `XX` with day number and `Y` with part number*

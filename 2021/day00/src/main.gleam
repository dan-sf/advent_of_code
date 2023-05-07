import gleam/io
import gleam/erlang/file
import gleam/int
import gleam/list
import gleam/map
import gleam/option
import gleam/result
import gleam/set
import gleam/string

pub fn part_01(input) {
    io.debug("part_01")
}

pub fn part_02(input) {
    io.debug("part_02")
}

pub fn main() {
    let assert Ok(input) = file.read("input.txt")

    part_01(input)
    part_02(input)
}

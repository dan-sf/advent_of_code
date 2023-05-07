import gleam/io
import gleam/erlang/file
import gleam/result
import gleam/string
import gleam/list
import gleam/int

pub fn part_01(input) {
    let output = input
    |> string.trim()
    |> string.split("\n")
    |> list.map(fn(i) {
        result.unwrap(int.parse(i), 0)
    })
    |> list.window_by_2()
    |> list.fold(0, fn(acc, val) {
        let #(first, second) = val
        case second > first {
            True -> 1 + acc
            False -> acc
        }
    })

    io.debug(output)
}

pub fn part_02(input) {
    let output = input
    |> string.trim()
    |> string.split("\n")
    |> list.map(fn(i) {
        result.unwrap(int.parse(i), 0)
    })
    |> list.window(3)
    |> list.map(fn(w) {
        int.sum(w)
    })
    |> list.window_by_2()
    |> list.fold(0, fn(acc, val) {
        let #(first, second) = val
        case second > first {
            True -> 1 + acc
            False -> acc
        }
    })

    io.debug(output)
}

pub fn main() {
    let assert Ok(input) = file.read("input.txt")

    part_01(input)
    part_02(input)
}

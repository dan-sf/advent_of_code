import gleam/io
import gleam/erlang/file
import gleam/int
import gleam/list
import gleam/string

pub fn part_01(input) {
    let lines = input
    |> string.trim()
    |> string.split("\n")

    let parsed = lines
    |> list.map(fn(line) {
        line
        |> string.split(" ")
        |> fn(sp) {
            let [first, second] = sp
            let assert Ok(val) = int.parse(second)
            #(first, val)
        }
    })

    let output = parsed
    |> list.fold(#(0, 0), fn(acc, command) {
        let #(cmd, val) = command
        let #(x, d) = acc
        case cmd {
            "forward" -> #(x + val, d)
            "down" -> #(x, d + val)
            "up" -> #(x, int.max(0, d - val))
        }
    })
    |> fn(pos) {
        let #(x, d) = pos
        x * d
    }

    io.debug(output)
}

pub fn part_02(input) {
    let lines = input
    |> string.trim()
    |> string.split("\n")

    let parsed = lines
    |> list.map(fn(line) {
        line
        |> string.split(" ")
        |> fn(sp) {
            let [first, second] = sp
            let assert Ok(val) = int.parse(second)
            #(first, val)
        }
    })

    let output = parsed
    |> list.fold(#(0, 0, 0), fn(acc, command) {
        let #(cmd, val) = command
        let #(x, d, a) = acc
        case cmd {
            "forward" -> #(x + val, d + a * val, a)
            "down" -> #(x, d, a + val)
            "up" -> #(x, d, int.max(0, a - val))
        }
    })
    |> fn(pos) {
        let #(x, d, _) = pos
        x * d
    }

    io.debug(output)
}

pub fn main() {
    let assert Ok(input) = file.read("input.txt")

    part_01(input)
    part_02(input)
}

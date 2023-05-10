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
    let lines = input
    |> string.trim()
    |> string.split("\n")

    let values_to_sum = lines
    |> list.map(fn(l) {
        l
        |> string.split("")
        |> list.map(fn(ch) {
            case ch {
                "1" -> 1
                "0" -> -1
            }
        })
    })

    let [first, .._] = values_to_sum
    let width = list.length(first)

    let gamma_bin = values_to_sum
    |> list.fold(list.repeat(0, width), fn(acc, cur) {
        list.zip(acc, cur)
        |> list.map(fn(pair) {
            let #(a, b) = pair
            a + b
        })
    })
    |> list.map(fn(val) {
        case val {
            _ if val > 0 -> "1"
            _ -> "0"
        }
    })

    let assert Ok(gamma) = gamma_bin
    |> string.join("")
    |> int.base_parse(2)

    let epsilon_bin = gamma_bin
    |> list.map(fn(val) {
        case val {
            "1" -> "0"
            "0" -> "1"
        }
    })

    let assert Ok(epsilon) = epsilon_bin
    |> string.join("")
    |> int.base_parse(2)

    io.debug(gamma * epsilon)
}

pub fn part_02(input) {
    io.debug("part_02")
}

pub fn main() {
    let assert Ok(input) = file.read("input.test.txt")

    part_01(input)
    part_02(input)
}

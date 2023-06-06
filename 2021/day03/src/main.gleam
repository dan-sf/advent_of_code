import gleam/io
import gleam/erlang/file
import gleam/int
import gleam/list
import gleam/result
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

pub fn bit_filter(values: List(List(String)), index: Int, keep_one: Bool) {

    let common = calc_common(values)

    let com = case list.at(common, index) {
        Ok(c) -> {
            case c {
                _ if c >= 0 -> "1"
                _ if c < 0 -> "0"
            }
        }
        Error(Nil) -> "0"
    }

    case list.length(values) {
        1 -> values
        _ -> {
            let ret = values
            |> list.filter_map(fn(val) {
                let assert Ok(cur) = list.at(val, index)
                case cur {
                    _ if cur == com && keep_one == True -> Ok(val)
                    _ if cur != com && keep_one == False -> Ok(val)
                    _ -> Error(Nil)
                }
            })

            ret |> bit_filter(index+1, keep_one)
        }
    }
}

pub fn calc_common(values: List(List(String))) {
    let values_to_sum = values
    |> list.map(fn(l) {
        list.map(l, fn(ch) {
            case ch {
                "1" -> 1
                "0" -> -1
            }
        })
    })

    let [first, .._] = values
    let width = list.length(first)

    let common = values_to_sum
    |> list.fold(list.repeat(0, width), fn(acc, cur) {
        list.zip(acc, cur)
        |> list.map(fn(pair) {
            let #(a, b) = pair
            a + b
        })
    })

    common
}

pub fn part_02(input) {
    let lines = input
    |> string.trim()
    |> string.split("\n")

    let bins = lines
    |> list.map(fn(l) { string.split(l, "") })

    let ox_gen = bit_filter(bins, 0, True)
    |> list.map(fn(b) { string.join(b, "") })
    |> string.join("")
    |> int.base_parse(2)

    let co_scr = bit_filter(bins, 0, False)
    |> list.map(fn(b) { string.join(b, "") })
    |> string.join("")
    |> int.base_parse(2)

    let output = result.unwrap(ox_gen, 0) * result.unwrap(co_scr, 0)

    io.debug(output)
}

pub fn main() {
    let assert Ok(input) = file.read("input.txt")

    part_01(input)
    part_02(input)
}

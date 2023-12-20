defmodule Aoc.Day01 do
  def part1(input_path) do
    File.read!(input_path)
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      nums =
        line
        |> String.split("", trim: true)
        |> Enum.filter(&String.match?(&1, ~r/[0-9]/))

      String.to_integer(List.first(nums) <> List.last(nums))
    end)
    |> Enum.sum()
  end

  def maybe_reverse(str, reverse) do
    if reverse do String.reverse(str) else str end
  end

  def get_num(line, is_last) do
    line
    |> maybe_reverse(is_last)
    |> String.replace(
      ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
      |> Enum.map(fn n -> maybe_reverse(n, is_last) end),
      fn val -> 
        one = maybe_reverse("one", is_last)
        two = maybe_reverse("two", is_last)
        three = maybe_reverse("three", is_last)
        four = maybe_reverse("four", is_last)
        five = maybe_reverse("five", is_last)
        six = maybe_reverse("six", is_last)
        seven = maybe_reverse("seven", is_last)
        eight = maybe_reverse("eight", is_last)
        nine = maybe_reverse("nine", is_last)

        case val do
          ^one -> "1"
          ^two -> "2"
          ^three -> "3"
          ^four -> "4"
          ^five -> "5"
          ^six -> "6"
          ^seven -> "7"
          ^eight -> "8"
          ^nine -> "9"
        end
      end)
    |> String.split("", trim: true)
    |> Enum.filter(&String.match?(&1, ~r/[0-9]/))
    |> List.first()
  end

  def part2(input_path) do
    File.read!(input_path)
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      String.to_integer(get_num(line, false) <> get_num(line, true))
    end)
    |> Enum.sum()
  end
end

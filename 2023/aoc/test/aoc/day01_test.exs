defmodule Aoc.Day01Test do
  use ExUnit.Case

  import Aoc.Day01

  test "part1" do
    result =
      "../../input/day01/input.test.1.txt"
      |> Path.expand(__DIR__)
      |> part1()

    assert result == 142
  end

  test "part2" do
    result =
      "../../input/day01/input.test.2.txt"
      |> Path.expand(__DIR__)
      |> part2()

    assert result == 281
  end
end

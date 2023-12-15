defmodule Aoc.Day11Test do
  use ExUnit.Case

  import Aoc.Day11

  @tag :skip
  test "part1" do
    result =
      "../../../input/day11/input.test.txt"
      |> Path.expand(__DIR__)
      |> part1()
    assert result
  end

  @tag :skip
  test "part2" do
    result =
      "../../../input/day11/input.test.txt"
      |> Path.expand(__DIR__)
      |> part2()
    assert result
  end
end

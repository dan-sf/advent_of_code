defmodule Aoc.Day02Test do
  use ExUnit.Case

  import Aoc.Day02

  test "part1" do
    result =
      "../../input/day02/input.test.1.txt"
      |> Path.expand(__DIR__)
      |> part1()

    assert result == 8
  end

  test "part2" do
    result =
      "../../input/day02/input.test.2.txt"
      |> Path.expand(__DIR__)
      |> part2()

    assert result == 2286
  end
end

defmodule Aoc.Day02Test do
  use ExUnit.Case

  import Aoc.Day02

  @tag :skip
  test "part1" do
    result =
      "../../input/day02/input.test.txt"
      |> Path.expand(__DIR__)
      |> part1()

    assert result
  end

  @tag :skip
  test "part2" do
    result =
      "../../input/day02/input.test.txt"
      |> Path.expand(__DIR__)
      |> part2()

    assert result
  end
end

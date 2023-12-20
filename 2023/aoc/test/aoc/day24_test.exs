defmodule Aoc.Day24Test do
  use ExUnit.Case

  import Aoc.Day24

  @tag :skip
  test "part1" do
    result =
      "../../input/day24/input.test.txt"
      |> Path.expand(__DIR__)
      |> part1()

    assert result
  end

  @tag :skip
  test "part2" do
    result =
      "../../input/day24/input.test.txt"
      |> Path.expand(__DIR__)
      |> part2()

    assert result
  end
end

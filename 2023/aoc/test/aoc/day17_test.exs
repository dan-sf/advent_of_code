defmodule Aoc.Day17Test do
  use ExUnit.Case

  import Aoc.Day17

  @tag :skip
  test "part1" do
    result =
      "../../input/day17/input.test.1.txt"
      |> Path.expand(__DIR__)
      |> part1()

    assert result
  end

  @tag :skip
  test "part2" do
    result =
      "../../input/day17/input.test.2.txt"
      |> Path.expand(__DIR__)
      |> part2()

    assert result
  end
end

defmodule Aoc.Day03Test do
  use ExUnit.Case

  import Aoc.Day03

  test "part1" do
    result =
      "../../input/day03/input.test.1.txt"
      |> Path.expand(__DIR__)
      |> part1()

    assert result == 4361
  end

  test "part2" do
    result =
      "../../input/day03/input.test.2.txt"
      |> Path.expand(__DIR__)
      |> part2()

    assert result == 467835
  end
end

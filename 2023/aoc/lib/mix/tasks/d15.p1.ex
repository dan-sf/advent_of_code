defmodule Mix.Tasks.D15.P1 do
  use Mix.Task

  import Aoc.Day15

  @shortdoc "Day 15 Part 1"
  def run(_args) do
    input = Path.expand("../../../input/day15/input.txt", __DIR__)

    input
    |> part1()
    |> IO.inspect(label: "Part 1 Results")

    #if Enum.member?(args, "-b"),
    #  do: Benchee.run(%{part_1: fn -> input |> part1() end}),
    #  else:
    #    input
    #    |> part1()
    #    |> IO.inspect(label: "Part 1 Results")
  end
end

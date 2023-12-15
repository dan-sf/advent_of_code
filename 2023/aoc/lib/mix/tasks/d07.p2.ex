defmodule Mix.Tasks.D07.P2 do
  use Mix.Task

  import Aoc.Day07

  @shortdoc "Day 07 Part 2"
  def run(_args) do
    input = Path.expand("../../../input/day07/input.txt", __DIR__)

    input
    |> part2()
    |> IO.inspect(label: "Part 2 Results")

    #if Enum.member?(args, "-b"),
    #  do: Benchee.run(%{part_1: fn -> input |> part1() end}),
    #  else:
    #    input
    #    |> part1()
    #    |> IO.inspect(label: "Part 1 Results")
  end
end

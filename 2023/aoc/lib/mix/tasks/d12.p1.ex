defmodule Mix.Tasks.D12.P1 do
  use Mix.Task

  import Aoc.Day12

  @shortdoc "Day 12 Part 1"
  def run(_args) do
    input = Path.expand("../../../input/day12/input.txt", __DIR__)

    input
    |> part1()
    |> IO.inspect(label: "Part 1 Results")

    # if Enum.member?(args, "-b"),
    #   do: Benchee.run(%{part_1: fn -> input |> part1() end}),
    #   else:
    #     input
    #     |> part1()
    #     |> IO.inspect(label: "Part 1 Results")
  end
end

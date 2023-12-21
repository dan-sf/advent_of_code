defmodule Mix.Tasks.AocRun do
  @moduledoc """
  This task is used to run the aoc problems for the given the day/part.
  Usage: `mix aoc_run DD.P`
  Where DD.P can be `01.1 | 11.2 | 21.1 | ...`. Only one day/part arg can be given.
  """

  @shortdoc "Runs aoc day/part solution"

  use Mix.Task

  defp get_input(day_part) do
    [day, _part] = String.split(day_part, ".")
    Path.expand("../../../input/day#{day}/input.txt", __DIR__)
  end

  @impl Mix.Task
  def run(args) do
    # TODO: Add error message if multiple day parts are given
    [day_part] = 
      args
      |> Enum.filter(&Regex.match?(~r/^[012][0-9]\.[12]$/, &1))

    # TODO: Can this be generated at compile time?
    case day_part do
      "01.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "01.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      "02.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "02.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      "03.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "03.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      "04.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "04.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      "05.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "05.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      "06.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "06.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      "07.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "07.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      "08.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "08.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      "09.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "09.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      "10.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "10.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      "11.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "11.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      "12.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "12.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      "13.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "13.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      "14.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "14.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      "15.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "15.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      "16.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "16.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      "17.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "17.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      "18.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "18.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      "19.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "19.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      "20.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "20.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      "21.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "21.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      "22.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "22.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      "23.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "23.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      "24.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "24.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      "25.1" -> Aoc.Day01.part1(get_input(day_part)) |> IO.inspect(label: "Part 1 Results")
      "25.2" -> Aoc.Day01.part2(get_input(day_part)) |> IO.inspect(label: "Part 2 Results")
      _ -> IO.puts "Error: Invalid day/part was given, day/part: '#{day_part}'"
    end
  end
end


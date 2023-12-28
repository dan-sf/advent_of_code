defmodule Mix.Tasks.AocRun do
  @moduledoc """
  This task is used to run the aoc problems for the given the day/part.
  Usage: `mix aoc_run DD.P`
  Where DD.P can be `01.1 | 11.2 | 21.1 | ...`. Only one day/part arg can be given.
  """

  @shortdoc "Runs aoc day/part solution"

  use Mix.Task

  defp get_input(day_part, is_test) do
    [day, part] = String.split(day_part, ".")
    if is_test do
      Path.expand("../../../input/day#{day}/input.test.#{part}.txt", __DIR__)
    else
      Path.expand("../../../input/day#{day}/input.txt", __DIR__)
    end
  end

  # The following macro builds out this case statement at compile time:
  #
  # case day_part do
  #   "01.1" -> Aoc.Day01.part1(get_input(day_part, is_test)) |> IO.inspect(label: "Part 1 Results")
  #   "01.2" -> Aoc.Day01.part2(get_input(day_part, is_test)) |> IO.inspect(label: "Part 2 Results")
  #   "02.1" -> Aoc.Day01.part1(get_input(day_part, is_test)) |> IO.inspect(label: "Part 1 Results")
  #   "02.2" -> Aoc.Day01.part2(get_input(day_part, is_test)) |> IO.inspect(label: "Part 2 Results")
  #   ...
  #   _ -> IO.puts "Error: Invalid day/part was given, day/part: '#{day_part}'"
  # end
  #
  # Normally I'd opt for the non-macro solution however it was good learnings
  # working out the macro approach
  defmacro build_case(day_part, is_test) do
    case_body =
      1..25
      |> Enum.flat_map(
        fn n ->
          day = day_number(n)
          module = String.to_existing_atom("Elixir.Aoc.Day#{day}")
          quote do
            unquote("#{day}.1") -> apply(unquote(module), unquote(:part1), [unquote(get_input("#{day}.1", is_test))]) |> IO.inspect(label: "Part 1 Results")
            unquote("#{day}.2") -> apply(unquote(module), unquote(:part2), [unquote(get_input("#{day}.2", is_test))]) |> IO.inspect(label: "Part 2 Results")
          end
        end
      )

    catch_all =
      quote do
        _ -> IO.puts "Error: Invalid day/part was given, day/part: '#{unquote(day_part)}'"
      end

    quote do
      case unquote(day_part) do
        unquote(case_body ++ catch_all)
      end
    end
  end

  defp day_number(day) when is_integer(day) do
      if day < 10, do: "0#{day}", else: "#{day}"
  end

  @impl Mix.Task
  def run(args) do
    # TODO: Add error message if multiple day parts are given
    [day_part] = 
      args
      |> Enum.filter(&Regex.match?(~r/^[012][0-9]\.[12]$/, &1))

    is_test =
      args
      |> Enum.filter(fn arg -> arg == "-t" end)
      |> then(fn args -> if length(args) >= 1 do true else false end end)

    # Since 'is_test' is known at runtime we need an extra if statement so we
    # can give concrete true/false to the macro
    if is_test do
      build_case(day_part, true)
    else
      build_case(day_part, false)
    end
  end
end

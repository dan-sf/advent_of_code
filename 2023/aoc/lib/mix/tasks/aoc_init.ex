defmodule Mix.Tasks.AocInit do
  @moduledoc """
  This task is used to generate aoc template files. It is meant for one time
  use then the generated stub files can be updated for the problems you are
  working on.
  """

  @shortdoc "Generates aoc template files"

  use Mix.Task

  defp init_code(base_path, day) do
    file_path = Path.join(base_path, "day#{day}.ex")

    content = """
    defmodule Aoc.Day#{day} do
      def part1(input_path) do
        File.read!(input_path)
      end

      def part2(input_path) do
        File.read!(input_path)
      end
    end
    """

    if File.exists?(file_path) == false, do: File.write!(file_path, content)
  end

  defp init_tests(base_path, day) do
    file_path = Path.join(base_path, "day#{day}_test.exs")

    content = """
    defmodule Aoc.Day#{day}Test do
      use ExUnit.Case

      import Aoc.Day#{day}

      @tag :skip
      test "part1" do
        result =
          "../../input/day#{day}/input.test.1.txt"
          |> Path.expand(__DIR__)
          |> part1()

        assert result
      end

      @tag :skip
      test "part2" do
        result =
          "../../input/day#{day}/input.test.2.txt"
          |> Path.expand(__DIR__)
          |> part2()

        assert result
      end
    end
    """

    if File.exists?(file_path) == false, do: File.write!(file_path, content)
  end

  defp init_inputs(base_path) do
    Path.join(base_path, "input.txt") |> File.touch!()
    Path.join(base_path, "input.test.1.txt") |> File.touch!()
    Path.join(base_path, "input.test.2.txt") |> File.touch!()
  end

  @impl Mix.Task
  def run(_args) do
    code_base_path = Path.expand("../../aoc", __DIR__)
    code_base_path |> File.mkdir_p!()

    test_base_path = Path.expand("../../../test/aoc", __DIR__)
    test_base_path |> File.mkdir_p!()

    for day <- 1..25 do
      day = if day < 10, do: "0#{day}", else: "#{day}"

      init_code(code_base_path, day)

      input_base_path = Path.expand("../../../input/day#{day}", __DIR__)
      input_base_path |> File.mkdir_p!()
      init_inputs(input_base_path)

      init_tests(test_base_path, day)
    end
  end
end

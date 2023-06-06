
defmodule Solution do
  def part_01(input) do
    output = input
    |> String.trim_trailing
    |> String.split
    |> Enum.map(&String.to_integer/1)
    |> Enum.chunk_every(2, 1, :discard)
    |> Enum.count(fn [a, b] -> b > a end)

    output
  end

  def part_02(input) do
    output = input
    |> String.trim_trailing
    |> String.split
    |> Enum.map(&String.to_integer/1)
    |> Enum.chunk_every(3, 1, :discard)
    |> Enum.map(&Enum.sum/1)
    |> Enum.chunk_every(2, 1, :discard)
    |> Enum.count(fn [a, b] -> b > a end)

    output
  end

  def main() do
    input = File.read!("input.txt")

    IO.puts part_01(input)
    IO.puts part_02(input)
  end
end

Solution.main()

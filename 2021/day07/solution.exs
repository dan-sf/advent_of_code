defmodule Solution do
  def parse(input) do
    input
    |> String.trim_trailing()
    |> String.split(",")
    |> Enum.map(&String.to_integer/1)
  end

  def get_dist(num, vals) do
    vals
    |> Enum.reduce(0, fn x, acc ->
      acc + abs(x - num)
    end)
  end

  def part_01(input) do
    values = parse(input)

    values
    |> Enum.reduce(Float.max_finite(), fn x, out ->
      min(out, get_dist(x, values))
    end)
  end

  def get_fuel(num, vals) do
    vals
    |> Enum.reduce(0, fn x, acc ->
      dist = abs(x - num)
      acc + Enum.sum(1..dist)
    end)
  end

  def part_02(input) do
    values = parse(input)

    Enum.min(values)..Enum.max(values)
    |> Enum.reduce(Float.max_finite(), fn x, out ->
      min(out, get_fuel(x, values))
    end)
  end

  def main() do
    input = File.read!("input.txt")

    IO.puts part_01(input)
    IO.puts part_02(input)
  end
end

Solution.main()

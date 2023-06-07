
defmodule Solution do
  def part_01(input) do
    lines = input
    |> String.trim_trailing
    |> String.split("\n")

    output = lines
    |> Enum.reduce({0, 0}, fn l, acc ->
      {x, depth} = acc
      {cmd, val} = List.to_tuple(String.split(l, " "))
      val = String.to_integer(val)
      case cmd do
        "forward" -> {x+val, depth}
        "down" -> {x, depth+val}
        "up" -> {x, max(0, depth-val)}
      end
    end)
    |> Tuple.product

    output
  end

  def part_02(input) do
    lines = input
    |> String.trim_trailing
    |> String.split("\n")

    calc = lines
    |> Enum.reduce({0, 0, 0}, fn l, acc ->
      {x, depth, aim} = acc
      {cmd, val} = List.to_tuple(String.split(l, " "))
      val = String.to_integer(val)
      case cmd do
        "forward" -> {x+val, depth + aim * val, aim}
        "down" -> {x, depth, aim + val}
        "up" -> {x, depth, max(0, aim - val)}
      end
    end)

    {x, depth, _} = calc

    x * depth
  end

  def main() do
    input = File.read!("input.txt")

    IO.puts part_01(input)
    IO.puts part_02(input)
  end
end

Solution.main()

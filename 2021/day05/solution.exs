defmodule Solution do
  def parse(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(fn x ->
      String.split(x, [",", " -> "])
      |> Enum.map(&String.to_integer/1)
    end)
  end

  def part_01(input) do
    lines = parse(input)

    points = lines
    |> Enum.reduce(%{}, fn
      [x, y1, x, y2], acc ->
        Enum.reduce(y1 .. y2, acc, fn y, acc ->
          Map.update(acc, {x, y}, 1, &(&1 + 1))
        end)
      [x1, y, x2, y], acc ->
        Enum.reduce(x1 .. x2, acc, fn x, acc ->
          Map.update(acc, {x, y}, 1, &(&1 + 1))
        end)
      _, acc ->
        acc
    end)

    Enum.count(points, fn {_, val} -> val > 1 end)
  end

  def part_02(input) do
    lines = parse(input)

    points = lines
    |> Enum.reduce(%{}, fn
      [x, y1, x, y2], acc ->
        Enum.reduce(y1 .. y2, acc, fn y, acc ->
          Map.update(acc, {x, y}, 1, &(&1 + 1))
        end)
      [x1, y, x2, y], acc ->
        Enum.reduce(x1 .. x2, acc, fn x, acc ->
          Map.update(acc, {x, y}, 1, &(&1 + 1))
        end)
      [x1, y1, x2, y2], acc ->
        Enum.reduce(Enum.zip(x1..x2, y1..y2), acc, fn point, acc ->
          Map.update(acc, point, 1, &(&1 + 1))
        end)
    end)

    Enum.count(points, fn {_, val} -> val > 1 end)
  end

  def main() do
    input = File.read!("input.txt")

    IO.puts part_01(input)
    IO.puts part_02(input)
  end
end

Solution.main()

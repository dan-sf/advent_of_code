defmodule Solution do
  def parse(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(fn x ->
      [x1, y1, x2, y2] = String.split(x, [",", " -> "])
      |> Enum.map(&String.to_integer/1)
      [{x1, y1}, {x2, y2}] |> Enum.sort() |> List.to_tuple()
    end)
  end

  def x_static(lines) do
    lines
    |> Enum.filter(fn
      {{x, _}, {x, _}} -> true
      _ -> false
    end)
    |> Enum.sort()
  end

  def y_static(lines) do
    lines
    |> Enum.filter(fn
      {{_, y}, {_, y}} -> true
      _ -> false
    end)
    |> Enum.sort()
  end

  def not_static(lines) do
    lines
    |> Enum.filter(fn x ->
      {{x1, y1}, {x2, y2}} = x
      y1 != y2 and x1 != x2
    end)
    |> Enum.sort()
  end

  def part_01(input) do
    lines = parse(input)

    points = y_static(lines)
    |> Enum.reduce(%{}, fn x, acc ->
      {{x1, y1}, {x2, _}} = x
      x1 .. x2
      |> Enum.reduce(acc, fn x, acc ->
        Map.update(acc, {x, y1}, 1, fn v -> v+1 end)
      end)
    end)

    points = x_static(lines)
    |> Enum.reduce(points, fn x, acc ->
      {{x1, y1}, {_, y2}} = x
      y1 .. y2
      |> Enum.reduce(acc, fn y, acc ->
        Map.update(acc, {x1, y}, 1, fn v -> v+1 end)
      end)
    end)

    Enum.count(points, fn {_, val} -> val > 1 end)
  end

  def part_02(input) do
    lines = parse(input)

    points = y_static(lines)
    |> Enum.reduce(%{}, fn x, acc ->
      {{x1, y1}, {x2, _}} = x
      x1 .. x2
      |> Enum.reduce(acc, fn x, acc ->
        Map.update(acc, {x, y1}, 1, fn v -> v+1 end)
      end)
    end)

    points = x_static(lines)
    |> Enum.reduce(points, fn x, acc ->
      {{x1, y1}, {_, y2}} = x
      y1 .. y2
      |> Enum.reduce(acc, fn y, acc ->
        Map.update(acc, {x1, y}, 1, fn v -> v+1 end)
      end)
    end)

    points = not_static(lines)
    |> Enum.reduce(points, fn x, acc ->
      {{x1, y1}, {x2, y2}} = x
      0 .. (x2-x1)
      |> Enum.reduce(acc, fn inc, acc ->
        if y2 >= y1 do
          Map.update(acc, {x1+inc, y1+inc}, 1, fn v -> v+1 end)
        else
          Map.update(acc, {x1+inc, y1-inc}, 1, fn v -> v+1 end)
        end
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

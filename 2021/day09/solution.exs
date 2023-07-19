defmodule Solution do
  def parse(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(fn values ->
      String.split(values, "", trim: true)
      |> Enum.map(&String.to_integer/1)
      |> List.to_tuple()
    end)
    |> List.to_tuple()
  end

  def smaller_than(value, other) do
    case value < Enum.min(other) do
      true -> value + 1
      false -> 0
    end
  end

  def index(tup, r, c), do: elem(elem(tup, r), c)

  def part_01(input) do
    parsed = parse(input)

    rows = tuple_size(parsed) - 1
    cols = tuple_size(elem(parsed, 0)) - 1

    for row <- 0..rows, col <- 0..cols do
      val = index(parsed, row, col)
      case {row, col} do
        # Corners
        {0, 0} -> smaller_than(val, [index(parsed, row+1, col), index(parsed, row, col+1)])
        {0, ^cols} -> smaller_than(val, [index(parsed, row+1, col), index(parsed, row, col-1)])
        {^rows, 0} -> smaller_than(val, [index(parsed, row-1, col), index(parsed, row, col+1)])
        {^rows, ^cols} -> smaller_than(val, [index(parsed, row-1, col), index(parsed, row, col-1)])
        # Sides
        {0, _} -> smaller_than(val, [index(parsed, row+1, col), index(parsed, row, col+1), index(parsed, row, col-1)])
        {^rows, _} -> smaller_than(val, [index(parsed, row-1, col), index(parsed, row, col+1), index(parsed, row, col-1)])
        {_, 0} -> smaller_than(val, [index(parsed, row+1, col), index(parsed, row-1, col), index(parsed, row, col+1)])
        {_, ^cols} -> smaller_than(val, [index(parsed, row+1, col), index(parsed, row-1, col), index(parsed, row, col-1)])
        # Middle
        {_, _} -> smaller_than(val, [index(parsed, row+1, col), index(parsed, row-1, col), index(parsed, row, col+1), index(parsed, row, col-1)])
      end
    end
    |> Enum.sum()
  end

  def get_basin_size(grid, seen, row, col) do
    val = grid[{row, col}]
    if val == nil or val == 9 or MapSet.member?(seen, {row, col}) do
      {seen, 0}
    else
      new_seen = seen |> MapSet.put({row, col})

      {new_seen, up} = get_basin_size(grid, new_seen, row-1, col)
      {new_seen, down} = get_basin_size(grid, new_seen, row+1, col)
      {new_seen, left} = get_basin_size(grid, new_seen, row, col-1)
      {new_seen, right} = get_basin_size(grid, new_seen, row, col+1)

      {new_seen, up + down + left + right + 1}
    end
  end

  def part_02(input) do
    parsed = parse(input)

    rows = tuple_size(parsed) - 1
    cols = tuple_size(elem(parsed, 0)) - 1

    grid = for row <- 0..rows, col <- 0..cols, into: %{} do
      val = index(parsed, row, col)
      {{row, col}, val}
    end

    indexes = for row <- 0..rows, col <- 0..cols do
      {row, col}
    end

    {values, _} = indexes
    |> Enum.map_reduce(MapSet.new(), fn {row, col}, seen ->
      {seen, val} = get_basin_size(grid, seen, row, col)
      {val, seen}
    end)

    values
    |> Enum.sort(:desc)
    |> Enum.take(3)
    |> Enum.product()
  end

  def main() do
    input = File.read!("input.txt")

    IO.inspect part_01(input)
    IO.inspect part_02(input)
  end
end

Solution.main()

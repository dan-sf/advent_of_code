defmodule Aoc.Day03 do
  def shape(grid), do: [tuple_size(grid), tuple_size(elem(grid, 0))]

  def to_number(number), do: Enum.reverse(number) |> Enum.join() |> String.to_integer()

  def is_valid?(grid, row, col) do
    [rows, cols] = shape(grid)
    if row >= rows || row < 0 || col >= cols || col < 0 do
      false
    else
      true
    end
  end

  def symbol_adjacent?(grid, row, col) do
    dirs = [
      [1, 1],
      [1, 0],
      [0, 1],
      [-1, -1],
      [-1, 0],
      [0, -1],
      [1, -1],
      [-1, 1]
    ]

    for [r, c] <- dirs do
      if is_valid?(grid, row + r, col + c) do
        !Regex.match?(~r/[0-9.]/, elem(elem(grid, row + r), col + c))
      else
        false
      end
    end
    |> Enum.any?()
  end

  def part1(input_path) do
    grid =
      File.read!(input_path)
      |> String.split("\n", trim: true)
      |> Enum.map(fn line ->
        line
        |> String.split("", trim: true)
        |> List.to_tuple()
      end)
      |> List.to_tuple()

    [rows, cols] = shape(grid)
    {solution, _, _} =
      for row <- 0..(rows-1), col <- 0..(cols-1), reduce: {0, [], []} do
        acc ->
          {output, number, adjacent} = acc

          val = elem(elem(grid, row), col)
          if Regex.match?(~r/[0-9]/, val) do
            number = [val | number]
            adjacent = [symbol_adjacent?(grid, row, col) | adjacent]
            {output, number, adjacent}
          else
            output =
              if Enum.any?(adjacent) && length(number) > 0 do
                output + (Enum.reverse(number) |> Enum.join() |> String.to_integer())
              else
                output
              end
            {output, [], []}
          end
      end

    solution
  end

  def adjacent_gear(grid, row, col) do
    dirs = [
      [1, 1],
      [1, 0],
      [0, 1],
      [-1, -1],
      [-1, 0],
      [0, -1],
      [1, -1],
      [-1, 1]
    ]

    for [r, c] <- dirs do
      if is_valid?(grid, row + r, col + c) && elem(elem(grid, row + r), col + c) == "*" do
        {row + r, col + c}
      else
        nil
      end
    end
  end

  def part2(input_path) do
    grid =
      File.read!(input_path)
      |> String.split("\n", trim: true)
      |> Enum.map(fn line ->
        line
        |> String.split("", trim: true)
        |> List.to_tuple()
      end)
      |> List.to_tuple()

    [rows, cols] = shape(grid)
    {gear_map, _, _} =
      for row <- 0..(rows-1), col <- 0..(cols-1), reduce: {%{}, [], []} do
        acc ->
          {gear_map, number, adjacent_gears} = acc

          val = elem(elem(grid, row), col)
          if Regex.match?(~r/[0-9]/, val) do
            number = [val | number]
            adjacent_gears = adjacent_gear(grid, row, col) ++ adjacent_gears
            {gear_map, number, adjacent_gears}
          else
            gear = Enum.find_value(adjacent_gears, fn g -> g end)
            gear_map =
              if gear != nil && length(number) > 0 do
                Map.put_new(gear_map, gear, [])
                |> Map.update!(gear, fn n -> [number | n] end)
              else
                gear_map
              end
            {gear_map, [], []}
          end
      end

    gear_map
    |> Map.reject(fn {_k, v} -> length(v) != 2 end)
    |> Enum.reduce(0, fn {_k, v}, acc ->
      [first, second] = v
      acc + to_number(first) * to_number(second)
    end)
  end
end

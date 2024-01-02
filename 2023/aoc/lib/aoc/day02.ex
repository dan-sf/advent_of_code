defmodule Aoc.Day02 do
  def is_valid(game_set) do
    game_set
    |> String.split([",", ";"])
    |> Enum.map(fn count ->
      [value, color] = count |> String.split(" ", trim: true)
      case color do
        "red" -> String.to_integer(value) <= 12
        "green" -> String.to_integer(value) <= 13
        "blue" -> String.to_integer(value) <= 14
      end
    end)
    |> Enum.all?()
  end

  def part1(input_path) do
    File.read!(input_path)
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      [game, sets] = line |> String.split(": ")
      [_, game_id] = game |> String.split(" ")

      if is_valid(sets) do
        String.to_integer(game_id)
      else
        0
      end
    end)
    |> Enum.sum()
  end

  def get_power(game_set) do
    game_set
    |> String.split([",", ";"])
    |> Enum.reduce([0, 0, 0], fn count, acc ->
      [value, color] = count |> String.split(" ", trim: true)
      [r, g, b] = acc
      case color do
        "red" -> [max(r, String.to_integer(value)), g, b]
        "green" -> [r, max(g, String.to_integer(value)), b]
        "blue" -> [r, g, max(b, String.to_integer(value))]
      end
    end)
    |> Enum.product()
  end

  def part2(input_path) do
    File.read!(input_path)
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      [_, sets] = line |> String.split(": ")
      get_power(sets)
    end)
    |> Enum.sum()
  end
end

defmodule Aoc.Day04 do
  defp parse_numbers(num_string) do
    num_string
    |> String.split(" ", trim: true)
    |> Enum.map(&String.to_integer/1)
    |> Enum.reduce(MapSet.new(), fn n, acc ->
      MapSet.put(acc, n)
    end)
  end

  defp get_points(matches) do
    case matches do
      0 -> 0
      n -> Integer.pow(2, n-1)
    end
  end

  def part1(input_path) do
    File.read!(input_path)
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      line
      |> String.split([":", "|"], trim: true)
      |> then(fn [_card, winning, yours] ->
        yours = yours |> parse_numbers()
        winning = winning |> parse_numbers()

        MapSet.intersection(yours, winning)
        |> MapSet.size()
        |> get_points()
      end)
    end)
    |> Enum.sum()
  end

  def part2(input_path) do
    lines =
      File.read!(input_path)
      |> String.split("\n", trim: true)

    cards = lines |> length()

    card_count = 1..cards |> Enum.into(%{}, fn x -> {x, 1} end)

    #|> Enum.map(fn line ->
    #  line
    #  |> String.split([":", "|"], trim: true)
    #  |> then(fn [_card, winning, yours] ->
    #    yours = yours |> parse_numbers()
    #    winning = winning |> parse_numbers()

    #    MapSet.intersection(yours, winning)
    #    |> MapSet.size()
    #    |> get_points()
    #  end)
    #end)
    #|> Enum.sum()
  end
end

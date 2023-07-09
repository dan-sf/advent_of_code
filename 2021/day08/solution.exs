defmodule Solution do
  def parse(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(fn x ->
      x
      |> String.split(" | ")
      |> Enum.map(&String.split/1)
    end)
  end

  def part_01(input) do
    parse(input)
    |> Enum.reduce(0, fn [_patterns, digits], acc ->
      count = digits
      |> Enum.reduce(0, fn digit, acc ->
        segs = String.split(digit, "", trim: true)
        if length(segs) == MapSet.size(MapSet.new(segs))
          and MapSet.member?(MapSet.new([2,4,3,7]), length(segs))  do
          acc + 1
        else
          acc
        end
      end)

      count + acc
    end)
  end

  def part_02(input) do
    parse(input)
    |> Enum.map(fn [input, output] ->
      len_map = input
      |> Enum.group_by(&byte_size/1, &String.to_charlist/1)

      %{
        2 => [one],
        3 => [seven],
        4 => [four],
        5 => two_three_five,
        6 => zero_six_nine,
        7 => [eight],
      } = len_map

      covers = fn numbers, base ->
        Enum.filter(numbers, &match?([], base -- &1))
      end

      covers_all_but_one = fn numbers, base ->
        Enum.filter(numbers, &match?([_], base -- &1))
      end

      [nine] = covers.(zero_six_nine, four)
      [six] = covers_all_but_one.(zero_six_nine, seven)
      [zero] = zero_six_nine -- [six, nine]

      [three] = covers.(two_three_five, seven)
      [five] = covers_all_but_one.(two_three_five, six)
      [two] = two_three_five -- [three, five]

      numbers = %{
        Enum.sort(zero) => 0,
        Enum.sort(one) => 1,
        Enum.sort(two) => 2,
        Enum.sort(three) => 3,
        Enum.sort(four) => 4,
        Enum.sort(five) => 5,
        Enum.sort(six) => 6,
        Enum.sort(seven) => 7,
        Enum.sort(eight) => 8,
        Enum.sort(nine) => 9
      }

      [d1, d2, d3, d4] = output
      |> Enum.map(fn d ->
        Enum.sort(String.to_charlist(d))
      end)

      Integer.undigits([
        numbers[d1], numbers[d2], numbers[d3], numbers[d4]
      ])
    end)
    |> Enum.sum()
  end

  def main() do
    input = File.read!("input.txt")

    IO.inspect part_01(input)
    IO.inspect part_02(input)
  end
end

Solution.main()

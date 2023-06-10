defmodule Solution do
  def part_01(input) do
    lines = input
    |> String.trim_trailing
    |> String.split("\n")

    gamma_bin = lines
    |> Enum.map(&String.graphemes/1)
    |> Enum.zip_reduce([], fn elements, acc ->
      val = Enum.sum(Enum.map(elements, fn elm ->
        case elm do
          "0" -> -1
          "1" -> 1
        end
      end))
      [ val | acc ]
    end)
    |> Enum.reverse
    |> Enum.map(fn(val) ->
      if val > 0 do "1"
      else "0" end
    end)

    {gamma, _} = gamma_bin
    |> Enum.join
    |> Integer.parse(2)

    epsilon_bin = gamma_bin
    |> Enum.map(fn(val) ->
      case val do
        "1" -> "0"
        "0" -> "1"
      end
    end)

    {epsilon, _} = epsilon_bin
    |> Enum.join
    |> Integer.parse(2)

    gamma * epsilon
  end

  def calc_common(values) do
    values
    |> Enum.zip_reduce([], fn elements, acc ->
      val = Enum.sum(Enum.map(elements, fn elm ->
        case elm do
          "0" -> -1
          "1" -> 1
        end
      end))
      [ val | acc ]
    end)
    |> Enum.reverse
  end

  def bit_filter(values, index, keep_one) do
    common = calc_common(values)
    com = if Enum.at(common, index) >= 0 do "0"
    else "1" end

    case length(values) do
      1 -> values
      _ ->
        ret = values
        |> Enum.flat_map(fn val ->
          cur = Enum.at(val, index)
          cond do
            cur == com && keep_one == True -> [val]
            cur != com && keep_one == False -> [val]
            true -> []
          end
        end)

        ret |> bit_filter(index+1, keep_one)
    end

  end

  def part_02(input) do
    lines = input
    |> String.trim_trailing
    |> String.split("\n")

    bins = lines |> Enum.map(&String.graphemes/1)

    {ox_gen, _} = bit_filter(bins, 0, True)
    |> Enum.map(&Enum.join/1)
    |> List.first
    |> Integer.parse(2)

    {co_scr, _} = bit_filter(bins, 0, False)
    |> Enum.map(&Enum.join/1)
    |> List.first
    |> Integer.parse(2)

    ox_gen * co_scr
  end

  def main() do
    input = File.read!("input.txt")

    IO.puts part_01(input)
    IO.puts part_02(input)
  end
end

Solution.main()

defmodule Solution do
  def parse(input) do
    input
    |> String.split("\n", trim: true)
  end

  def matches(open, close) do
    case open do
      "(" -> close == ")"
      "{" -> close == "}"
      "[" -> close == "]"
      "<" -> close == ">"
    end
  end

  def part_01(input) do
    parsed = parse(input)

    values = %{
      ")" => 3,
      "]" => 57,
      "}" => 1197,
      ">" => 25137
    }

    lines = parsed
    |> Enum.map(fn code ->
      String.graphemes(code)
      |> Enum.reduce_while([], fn x, acc ->
        cond do
          x in ["(", "[", "{", "<"] ->
            {:cont, [x | acc]}
          x in [")", "]", "}", ">"] ->
            [open | new_acc] = acc
            if matches(open, x) do
              {:cont, new_acc}
            else
              {:halt, {x, :found}}
            end
        end
      end)
    end)

    bad_chars = for {value, :found} <- lines, do: values[value]

    bad_chars
    |> Enum.frequencies()
    |> Enum.reduce(0, fn {key, val}, acc ->
      acc + key * val
    end)
  end

  def part_02(input) do
    parsed = parse(input)

    values = %{
      "(" => 1,
      "[" => 2,
      "{" => 3,
      "<" => 4
    }

    scores = parsed
    |> Enum.map(fn code ->
      String.graphemes(code)
      |> Enum.reduce_while([], fn x, acc ->
        cond do
          x in ["(", "[", "{", "<"] ->
            {:cont, [x | acc]}
          x in [")", "]", "}", ">"] ->
            [open | new_acc] = acc
            if matches(open, x) do
              {:cont, new_acc}
            else
              {:halt, {x, :found}}
            end
        end
      end)
    end)
    |> Enum.filter(fn x -> !match?({_, :found}, x) end)
    |> Enum.map(fn row ->
      row
      |> Enum.reduce(0, fn val, acc ->
        5 * acc + values[val]
      end)
    end)
    |> Enum.sort()

    scores |> Enum.at(div(length(scores), 2))
  end

  def main() do
    input = File.read!("input.txt")

    IO.inspect part_01(input)
    IO.inspect part_02(input)
  end
end

Solution.main()

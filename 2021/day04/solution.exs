defmodule Solution do
  def part_01(input) do
    lines = input
    |> String.trim_trailing
    |> String.split("\n\n")

    [draw_numbers | boxes] = lines

    draw_numbers = String.split(draw_numbers, ",")

    boxes = boxes
    |> Enum.map(fn val ->
      val
      |> String.split("\n")
      |> Enum.map(fn it ->
        it
        |> String.split
        |> Enum.map(&{&1, false})
      end)
    end)

    {boxes, index, val} = play(boxes, draw_numbers)

    winner = Enum.at(boxes, index)

    not_called_sum = winner
    |> Enum.reduce(0, fn x, acc ->
      acc + Enum.reduce(x, 0, fn x, acc ->
        acc + case x do
          {_, true} -> 0
          {a, false} -> String.to_integer(a)
        end
      end)
    end)

    not_called_sum * val
  end

  def play(boxes, values) do
    [val | values] = values
    new_boxes = mark_values(boxes, val)

    found_rows = new_boxes
    |> Enum.map(&check_rows/1)

    found_row_index = Enum.find_index(found_rows, fn x -> x == true end)

    found_cols = new_boxes
    |> Enum.map(fn box -> 
      Enum.zip(box)
      |> Enum.map(&Tuple.to_list/1)
    end)
    |> Enum.map(&check_rows/1)

    found_col_index = Enum.find_index(found_cols, fn x -> x == true end)

    found = found_row_index != nil or found_col_index != nil

    if length(values) == 0 or found do
      if found do
        index = Enum.find([found_row_index, found_col_index], fn x -> x != nil end)
        {new_boxes, index, String.to_integer(val)}
      else
        {new_boxes, -1, -1}
      end
    else
      play(new_boxes, values)
    end
  end

  def play_last(boxes, values) do
    [val | values] = values
    new_boxes = mark_values(boxes, val)

    found_rows = new_boxes
    |> Enum.map(&check_rows/1)

    found_cols = new_boxes
    |> Enum.map(fn box -> 
      Enum.zip(box)
      |> Enum.map(&Tuple.to_list/1)
    end)
    |> Enum.map(&check_rows/1)

    found_indexes = Enum.zip_with([found_rows, found_cols], fn [x, y] -> x or y end)

    new_boxes = if length(found_indexes) > 1 do
      new_boxes
      |> Enum.zip(found_indexes)
      |> Enum.filter(fn x -> {_, found} = x; !found end)
      |> Enum.map(fn x -> {box, _} = x; box end)
    else
      new_boxes
    end

    if length(values) == 0 or found_indexes == [true] do
      if found_indexes == [true] do
        {new_boxes, String.to_integer(val)}
      else
        {new_boxes, -1}
      end
    else
      play_last(new_boxes, values)
    end
  end

  def mark_values(boxes, value) do
    for board <- boxes do
      mark_value(board, value)
    end
  end

  def mark_value(board, value) do
    for row <- board do
      for elm <- row do
        {val, _} = elm
        cond do
          val == value -> {val, true}
          true -> elm
        end
      end
    end
  end

  def check_rows([last]) do
    Enum.all?(last, fn x -> elem(x, 1) end)
  end

  def check_rows([row | rest]) do
    found = check_rows([row])
    if found do
      true
    else
      check_rows(rest)
    end
  end

  def part_02(input) do
    lines = input
    |> String.trim_trailing
    |> String.split("\n\n")

    [draw_numbers | boxes] = lines

    draw_numbers = String.split(draw_numbers, ",")

    boxes = boxes
    |> Enum.map(fn val ->
      val
      |> String.split("\n")
      |> Enum.map(fn it ->
        it
        |> String.split
        |> Enum.map(&{&1, false})
      end)
    end)

    {boxes, val} = play_last(boxes, draw_numbers)
    winner = List.first(boxes)

    not_called_sum = winner
    |> Enum.reduce(0, fn x, acc ->
      acc + Enum.reduce(x, 0, fn x, acc ->
        acc + case x do
          {_, true} -> 0
          {a, false} -> String.to_integer(a)
        end
      end)
    end)

    not_called_sum * val
  end

  def main() do
    input = File.read!("input.txt")

    IO.puts part_01(input)
    IO.puts part_02(input)
  end
end

Solution.main()

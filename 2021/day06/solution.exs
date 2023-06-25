defmodule Solution do
  def parse(input) do
    input
    |> String.trim_trailing()
    |> String.split(",")
    |> Enum.map(&String.to_integer/1)
  end

  def part_01(input) do
    start = parse(input)

    fish = 1..80
    |> Enum.reduce(start, fn _day, acc ->
      {val, added} = Enum.map_reduce(acc, 0, fn
        0, aged -> {6, aged + 1}
        fish, aged -> {fish - 1, aged}
      end)

      val ++ List.duplicate(8, added)
    end)

    length(fish)
  end

  def part_02(input) do
    start = parse(input)

    count = Enum.frequencies(start)

    # Use 2 queues [0->6] and [7->8] (sizes 7 and 2) to model the count of the
    # fish at each day
    queue_seven = 0..6
    |> Enum.reduce(:queue.new(), fn x, queue ->
      case Map.get(count, x) do
        nil -> :queue.in(0, queue)
        val -> :queue.in(val, queue)
      end
    end)

    queue_two = :queue.new()
    |> then(fn q -> :queue.in(0, q) end)
    |> then(fn q -> :queue.in(0, q) end)

    {final_seven, final_two} = 1..256
    |> Enum.reduce({queue_seven, queue_two}, fn _day, queues ->
      {q7, q2} = queues
      {{:value, val_q7}, q7} = :queue.out(q7)
      {{:value, val_q2}, q2} = :queue.out(q2)

      added_to_q2 = val_q7
      added_to_q7 = val_q2 + val_q7

      {:queue.in(added_to_q7, q7), :queue.in(added_to_q2, q2)}
    end)

    Enum.sum(:queue.to_list(final_seven)) + Enum.sum(:queue.to_list(final_two))
  end

  def main() do
    input = File.read!("input.txt")

    IO.puts part_01(input)
    IO.puts part_02(input)
  end
end

Solution.main()

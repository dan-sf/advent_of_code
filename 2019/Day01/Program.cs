using System;
using System.IO;
using System.Linq;
using static System.Console;

namespace Day01 {
    class Program {
        static void Main(string[] args) {
            var solution1 = File.ReadAllLines("input.txt")
                .Select(r => Int32.Parse(r) / 3 - 2)
                .Sum();
            WriteLine($"Solution 1: {solution1}");

            int getFuel(int module) {
                var fuel = module / 3 - 2;
                if (fuel <= 0) {
                    return 0;
                }
                return fuel + getFuel(fuel);
            }

            var solution2 = File.ReadAllLines("input.txt")
                .Select(r => getFuel(Int32.Parse(r)))
                .Sum();
            WriteLine($"Solution 2: {solution2}");
        }
    }
}

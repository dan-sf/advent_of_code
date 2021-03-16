using System;
using System.IO;
using System.Linq;
using static System.Console;

namespace Day02 {
    class Program {
        static void Main(string[] args) {
            int[] memory = File.ReadAllText("input.txt")
                .TrimEnd('\n')
                .Split(',')
                .Select(r => Int32.Parse(r))
                .ToArray();

            // We use the .ToArray() function to copy the memory so the array
            // isn't passed as a referenece

            WriteLine($"Solution 1: {run(memory.ToArray(), 12, 2)}");

            for (int i=0; i<100; i++) {
                for (int j=0; j<100; j++) {
                    if (run(memory.ToArray(), i, j) == 19690720) {
                        WriteLine($"Solution 2: {100 * i + j}");
                        return;
                    }
                }
            }
        }

        static int run(int[] input, int noun, int verb) {
            input[1] = noun;
            input[2] = verb;

            var index = 0;
            while (true) {
                if (input[index] == 1) {
                    var a = input[input[index+1]];
                    var b = input[input[index+2]];
                    input[input[index+3]] = a + b;
                } else if (input[index] == 2) {
                    var a = input[input[index+1]];
                    var b = input[input[index+2]];
                    input[input[index+3]] = a * b;
                } else if (input[index] == 99) {
                    break;
                } else {
                    WriteLine("Invalid op-code found");
                    break;
                }
                index += 4;
            }

            return input[0];
        }
    }
}

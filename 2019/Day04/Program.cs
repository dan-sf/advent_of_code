using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;
using static System.Console;

namespace Day04 {
    class Program {
        static void Main(string[] args) {
            var input = File.ReadAllText("input.txt")
                .TrimEnd('\n')
                .Split('-')
                .Select(r => Int32.Parse(r))
                .ToArray();

            var start = input[0];
            var end = input[1];

            var output = 0;
            for (int num = start; num <= end; num++) {
                if (isCandidate(num)) {
                    output += 1;
                }
            }

            WriteLine($"Solution 1: {output}");

            var output2 = 0;
            for (int num = start; num <= end; num++) {
                if (isCandidate2(num)) {
                    output2 += 1;
                }
            }

            WriteLine($"Solution 2: {output2}");
        }

        public static bool isCandidate(int num) {
            var hasEqual = false;
            var numStr = num.ToString();

            for (int i = 1; i < numStr.Length; i++) {
                if (numStr[i] < numStr[i-1]) {
                    return false;
                }
                if (numStr[i] == numStr[i-1]) {
                    hasEqual = true;
                }
            }
            return hasEqual;
        }

        public static bool isCandidate2(int num) {
            var hasEqual = false;
            var numStr = num.ToString();

            for (int i = 1; i < numStr.Length; i++) {
                if (numStr[i] < numStr[i-1]) {
                    return false;
                }
                if (numStr[i] == numStr[i-1]) {
                    if (i == 1 && numStr[i] != numStr[i+1]) {
                        hasEqual = true;
                    } else if (i == numStr.Length-1 && numStr[i] != numStr[i-2]) {
                        hasEqual = true;
                    } else if (i > 1 && i < numStr.Length && numStr[i] != numStr[i-2] && numStr[i] != numStr[i+1]) {
                        hasEqual = true;
                    }
                }
            }
            return hasEqual;
        }
    }
}

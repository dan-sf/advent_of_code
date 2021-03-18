using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;
using static System.Console;

namespace Day06 {
    class Program {
        static void Main(string[] args) {
            var input = File.ReadAllLines("input.txt")
                .Select(r => r.TrimEnd('\n').Split(')'))
                .ToArray();

            Solution1(input);

            var orbitMap = new Dictionary<string, List<string>>();

            foreach (var orbit in input) {
                if (orbitMap.ContainsKey(orbit[0])) {
                    orbitMap[orbit[0]].Add(orbit[1]);
                } else {
                    orbitMap[orbit[0]] = new List<string>() { orbit[1] };
                }

                if (orbitMap.ContainsKey(orbit[1])) {
                    orbitMap[orbit[1]].Add(orbit[0]);
                } else {
                    orbitMap[orbit[1]] = new List<string>() { orbit[0] };
                }
            }

            var queue = new Queue<(string, int)>(new[] { ("SAN", 0) });
            var seen = new HashSet<string>() { "SAN" };

            while (queue.Count > 0) {
                (var current, var count) = queue.Dequeue();
                if (current == "YOU") {
                    WriteLine($"Solution 2: {count-2}");
                }

                seen.Add(current);
                foreach (var p in orbitMap[current]) {
                    if (!seen.Contains(p)) {
                        queue.Enqueue((p, count+1));
                    }
                }
            }
        }

        static void Solution1(string[][] input) {
            var orbitMap = new Dictionary<string, List<string>>();
            foreach (var orbit in input) {
                if (orbitMap.ContainsKey(orbit[0])) {
                    orbitMap[orbit[0]].Add(orbit[1]);
                } else {
                    orbitMap[orbit[0]] = new List<string>() { orbit[1] };
                }
            }

            int recurse(string planet, int level) {
                if (!orbitMap.ContainsKey(planet)) {
                    return 0;
                }

                var output = 0;
                foreach (var p in orbitMap[planet]) {
                    output += level + recurse(p, level + 1);
                }

                return output;
            }

            WriteLine($"Solution 1: {recurse("COM", 1)}");
        }

    }
}

using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;
using static System.Console;

namespace Day03 {
    class Program {
        static void Main(string[] args) {
            var input = File.ReadAllLines("input.txt")
                .Select(r => r.TrimEnd('\n'))
                .ToArray();

            List<Line> generateSegments(string moves) {
                var loc = (0, 0);
                var wirePath = moves.Split(',');

                var wireSegments = new List<Line>();
                foreach (var path in wirePath) {
                    var moved = moveToPoint(loc, path);
                    wireSegments.Add(new Line(loc, moved));
                    loc = moved;
                }
                return wireSegments;
            }

            var wireSegmentsA = generateSegments(input[0]);
            var wireSegmentsB = generateSegments(input[1]);

            var closest = Int32.MaxValue;

            foreach (var wireA in wireSegmentsA) {
                foreach (var wireB in wireSegmentsB) {
                    var intersect = wireA.intersectsWith(wireB);
                    if (intersect.HasValue && intersect.Value != (0, 0)) {
                        var dist = Math.Abs(intersect.Value.Item1) + Math.Abs(intersect.Value.Item2);
                        closest = Math.Min(closest, dist);
                    }
                }
            }

            WriteLine($"Solution 1: {closest}");

            var intersections = new List<(int, int)>();
            foreach (var wireA in wireSegmentsA) {
                foreach (var wireB in wireSegmentsB) {
                    var intersect = wireA.intersectsWith(wireB);
                    if (intersect.HasValue && intersect.Value != (0, 0)) {
                        intersections.Add(intersect.Value);
                    }
                }
            }

            var totals = new int[intersections.Count];

            void addSteps(List<Line> wireSegments) {
                for (int j = 0; j < intersections.Count; j++) {
                    var total = 0;
                    for (int i = 0; i < wireSegments.Count; i++) {
                        if (wireSegments[i].containsPoint(intersections[j])) {
                            if (i == 0) {
                                total += Math.Abs(intersections[j].Item1) + Math.Abs(intersections[j].Item2);
                            } else {
                                if (wireSegments[i].start == wireSegments[i-1].end) {
                                    if (wireSegments[i].isVertical()) {
                                        total += intersections[j].Item2 - wireSegments[i].start.Item2;
                                    } else {
                                        total += intersections[j].Item1 - wireSegments[i].start.Item1;
                                    }
                                } else {
                                    if (wireSegments[i].isVertical()) {
                                        total += wireSegments[i].end.Item2 - intersections[j].Item2;
                                    } else {
                                        total += wireSegments[i].end.Item1 - intersections[j].Item1;
                                    }
                                }
                            }
                            break;
                        }
                        total += wireSegments[i].getLength();
                    }
                    totals[j] += total;
                }
            }

            addSteps(wireSegmentsA);
            addSteps(wireSegmentsB);
            WriteLine($"Solution 2: {totals.Min()}");
        }

        static (int, int) moveToPoint((int, int) point, string move) {
            var dir = move[0];
            var mul = (0, 0);

            switch (dir) {
                case 'U':
                    mul = (1, 0);
                    break;
                case 'D':
                    mul = (-1, 0);
                    break;
                case 'R':
                    mul = (0, 1);
                    break;
                case 'L':
                    mul = (0, -1);
                    break;
                default:
                    break;
            }

            var val = Int32.Parse(move.Substring(1));
            return (point.Item1 + val * mul.Item1, point.Item2 + val * mul.Item2);
        }
    }

    class Line {
        public (int, int) start;
        public (int, int) end;

        public Line((int, int) start, (int, int) end) {
            var startEnd = new (int, int)[] {start, end};
            Array.Sort(startEnd);
            this.start = startEnd[0];
            this.end = startEnd[1];
        }

        public bool isVertical() {
            return this.start.Item1 == this.end.Item1;
        }

        public bool containsPoint((int, int) point) {
            if (point.Item1 == this.start.Item1 && point.Item2 <= this.end.Item2 && point.Item2 >= this.start.Item2) {
                return true;
            }
            if (point.Item2 == this.start.Item2 && point.Item1 <= this.end.Item1 && point.Item1 >= this.start.Item1) {
                return true;
            }
            return false;
        }

        public int getLength() {
            if (isVertical()) {
                return this.end.Item2 - this.start.Item2;
            }
            return this.end.Item1 - this.start.Item1;
        }


        public (int, int)? intersectsWith(Line other) {
            if (this.start == other.start && this.end == other.end) {
                return null;
            }
            if (this.start.Item1 == this.end.Item1 && other.start.Item1 == other.end.Item1) {
                return null;
            }
            if (this.start.Item2 == this.end.Item2 && other.start.Item2 == other.end.Item2) {
                return null;
            }

            if (this.start.Item1 == this.end.Item1) {
                var x = this.start.Item1;
                if (other.start.Item1 <= x && other.end.Item1 >= x) {
                    var y = other.start.Item2;
                    if (this.start.Item2 <= y && this.end.Item2 >= y) {
                        return (x, y);
                    }
                }
            } else {
                var x = other.start.Item1;
                if (this.start.Item1 <= x && this.end.Item1 >= x) {
                    var y = this.start.Item2;
                    if (other.start.Item2 <= y && other.end.Item2 >= y) {
                        return (x, y);
                    }
                }
            }

            return null;
        }
    }
}

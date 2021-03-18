using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;
using static System.Console;

namespace Day07 {
    class Program {
        static void Main(string[] args) {
            int[] memory = File.ReadAllText("input.txt")
                .TrimEnd('\n')
                .Split(',')
                .Select(r => Int32.Parse(r))
                .ToArray();

            Solution1(memory);
            //Solution2(memory);
        }

        static void Solution1(int[] memory) {
            var sequences = generateSequences(0, 5);
            var inputSignal = 0;
            var Solution1 = 0;

            var temp = memory.ToArray();
            foreach (var seq in sequences) {
                var res = 0;
                foreach (var setting in seq) {
                    Instruction inst = new Instruction(memory, 0);

                    if (inst.needsInput) {
                        inst.instInput = setting;
                    }

                    while (true) {
                        var cont = inst.run(memory);
                        if (cont == false) {
                            break;
                        }
                        if (inst.hasResult) {
                            res = inst.result;
                        }

                        inst = new Instruction(memory, inst.opIndex + inst.jump);
                        if (inst.needsInput) {
                            inst.instInput = res;
                        }
                    }
                    inputSignal = res;
                }
                memory = temp.ToArray();
                Solution1 = Math.Max(Solution1, res);
                inputSignal = 0;
            }

            WriteLine($"Solution 1: {Solution1}");
        }

        static void Solution2(int[] memory) {
            //var sequences = generateSequences(5, 10);
            var inputSignal = 0;
            var Solution2 = 0;

            var res = 0;

            var sequences = new int[] {9,8,7,6,5};
            //while (true) {
                //foreach (var seq in sequences) {
                    var seq = sequences;
                    foreach (var setting in seq) {
                        Instruction inst = new Instruction(memory, 0);

                        if (inst.needsInput) {
                            inst.instInput = setting;
                        }

                        while (true) {
                            var cont = inst.run(memory);
                            if (cont == false) {
                                break;
                            }
                            if (inst.hasResult) {
                                res = inst.result;
                            }

                            inst = new Instruction(memory, inst.opIndex + inst.jump);
                            if (inst.needsInput) {
                                inst.instInput = res;
                            }
                        }
                        inputSignal = res;
                    }
                //}
                Solution2 = Math.Max(Solution2, res);
                WriteLine($"Solution 2: {Solution2}");
            //}

            WriteLine($"Solution 2: {Solution2}");
        }


        static List<List<int>> generateSequences(int start, int end) {

            List<List<int>> output = new List<List<int>>();
            void recurse(List<int> chosen, List<int> elements) {
                if (elements.Count == 0) {
                    output.Add(chosen);
                    return;
                }

                for (int i = 0; i < elements.Count; i++) {
                    recurse(
                        chosen.Concat(new List<int> {elements[i]}).ToList(),
                        elements.Take(i).Concat(elements.Skip(i+1)).ToList());
                }
            }

            var elements = new List<int>();
            for (int i = start; i < end; i++) {
                elements.Add(i);
            }
            recurse(new List<int>(), elements);

            return output;
        }
    }

    class ProgramState {
        public int lastOutput = 0;
        public int lastInput = 0;

        public ProgramState() { }
    }

    class Instruction {
        public int jump;
        public int opCode;
        public int opIndex;
        public int instInput;
        public int[] paramModes;
        public int result;

        public bool hasResult;
        public bool needsInput;

        public Instruction(int[] memory, int index) {
            opCode = memory[index] % 100;
            opIndex = index;
            var paramNum = memory[index] / 100;

            needsInput = false;
            hasResult = false;

            if (opCode == 1 || opCode == 2 || opCode == 7 || opCode == 8) {
                paramModes = new int[] {0, 0, 0};
                jump = 4;
            } else if (opCode == 3 || opCode == 4) {
                if (opCode == 3) needsInput = true;
                if (opCode == 4) hasResult = true;
                paramModes = new int[] {0};
                jump = 2;
            } else if (opCode == 5 || opCode == 6) {
                paramModes = new int[] {0, 0};
                jump = 3;
            } else if (opCode == 99) {
                paramModes = new int[] {};
                jump = 0;
            }

            int i = 0;
            while (paramNum > 0) {
                paramModes[i] = paramNum % 10;
                paramNum /= 10;
                i += 1;
            }
        }

        public bool run(int[] memory) {
            if (opCode == 1 || opCode == 2) {
                int a = 0; int b = 0;
                if (paramModes[0] == 1) {
                    a = memory[opIndex+1];
                } else {
                    a = memory[memory[opIndex+1]];
                }
                if (paramModes[1] == 1) {
                    b = memory[opIndex+2];
                } else {
                    b = memory[memory[opIndex+2]];
                }

                if (opCode == 1) {
                    memory[memory[opIndex+3]] = a + b;
                } else {
                    memory[memory[opIndex+3]] = a * b;
                }

                return true;
            } else if (opCode == 3 || opCode == 4) {
                if (opCode == 3) {
                    //string userInput;
                    //if (instInput == "-1") {
                    //    Write("Input value: ");
                    //    userInput = ReadLine();
                    //} else {
                    //    userInput = instInput;
                    //}

                    //memory[memory[opIndex+1]] = Int32.Parse(userInput.TrimEnd('\n'));

                    memory[memory[opIndex+1]] = instInput;

                } else {
                    if (paramModes[0] == 1) {
                        result = memory[opIndex+1];
                    } else {
                        result = memory[memory[opIndex+1]];
                    }
                    hasResult = true;
                    //WriteLine($"Output: {result}");
                }
                return true;
            } else if (opCode >= 5 && opCode <= 8) {
                if (opCode == 5) {
                    var valid = paramModes[0] == 1 ? memory[opIndex+1] != 0 : memory[memory[opIndex+1]] != 0;
                    if (valid) {
                        opIndex = paramModes[1] == 1 ? memory[opIndex+2] : memory[memory[opIndex+2]];
                        jump = 0;
                    }
                } else if (opCode == 6) {
                    var valid = paramModes[0] == 1 ? memory[opIndex+1] == 0 : memory[memory[opIndex+1]] == 0;
                    if (valid) {
                        opIndex = paramModes[1] == 1 ? memory[opIndex+2] : memory[memory[opIndex+2]];
                        jump = 0;
                    }
                } else if (opCode == 7) {
                    int first = paramModes[0] == 1 ? memory[opIndex+1] : memory[memory[opIndex+1]];
                    int second = paramModes[1] == 1 ? memory[opIndex+2] : memory[memory[opIndex+2]];

                    if (paramModes[2] == 1) {
                        memory[opIndex+3] = first < second ? 1 : 0;
                    } else {
                        memory[memory[opIndex+3]] = first < second ? 1 : 0;
                    }
                } else if (opCode == 8) {
                    int first = paramModes[0] == 1 ? memory[opIndex+1] : memory[memory[opIndex+1]];
                    int second = paramModes[1] == 1 ? memory[opIndex+2] : memory[memory[opIndex+2]];

                    if (paramModes[2] == 1) {
                        memory[opIndex+3] = first == second ? 1 : 0;
                    } else {
                        memory[memory[opIndex+3]] = first == second ? 1 : 0;
                    }
                }
                return true;
            }

            return false;
        }
    }

}

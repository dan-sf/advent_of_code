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
            Solution2(memory);
        }

        static void Solution1(int[] memory) {
            var sequences = generateSequences(0, 5);
            var programState = new ProgramState();
            var Solution1 = 0;

            var temp = memory.ToArray();
            foreach (var seq in sequences) {
                foreach (var setting in seq) {
                    Instruction inst = new Instruction(memory, 0, programState);

                    if (inst.needsInput) {
                        inst.instInput = setting;
                    }

                    while (true) {
                        var cont = inst.run(memory);
                        if (cont == false) {
                            break;
                        }

                        inst = new Instruction(memory, inst.opIndex + inst.jump, programState);
                        if (inst.needsInput) {
                            inst.instInput = programState.state;
                        }
                    }
                }
                Solution1 = Math.Max(Solution1, programState.state);
                memory = temp.ToArray();
                programState.reset();
            }

            WriteLine($"Solution 1: {Solution1}");
        }

        static void Solution2(int[] memory) {
            var sequences = generateSequences(5, 10);
            var Solution2 = 0;

            foreach (var inputs in sequences) {
                var initInputs = inputs.ToArray();
                var done = false;
                var firstLoop = true;

                var amps = new int[][] {
                    memory.ToArray(),
                    memory.ToArray(),
                    memory.ToArray(),
                    memory.ToArray(),
                    memory.ToArray()};
                var instructions = new Instruction[] {
                    new Instruction(amps[0], 0, new ProgramState()),
                    new Instruction(amps[1], 0, new ProgramState()),
                    new Instruction(amps[2], 0, new ProgramState()),
                    new Instruction(amps[3], 0, new ProgramState()),
                    new Instruction(amps[4], 0, new ProgramState())};
                var inited = new bool[] {
                    false,
                    false,
                    false,
                    false,
                    false};

                while (!done) {
                    for (int i = 0; i < amps.Length; i++) {
                        var hasOutput = false;
                        while (!done && !hasOutput) {
                            if (instructions[i].needsInput) {
                                if (inited[i]) {
                                    instructions[i].instInput = inputs[i];
                                } else {
                                    instructions[i].instInput = initInputs[i];
                                    inited[i] = true;
                                }

                                if (firstLoop) {
                                    firstLoop = false;
                                    inputs[i] = 0; // Edge case for first amp
                                }
                            }

                            var cont = instructions[i].run(amps[i]);
                            if (cont == false) {
                                Solution2 = Math.Max(Solution2, inputs[0]);
                                done = true;
                                break;
                            }
                            if (instructions[i].hasResult) {
                                inputs[(i+1) % amps.Length] = instructions[i].result;
                                hasOutput = true;
                            }

                            instructions[i] = new Instruction(amps[i],
                                    instructions[i].opIndex + instructions[i].jump,
                                    instructions[i].programState);
                        }
                    }
                }
            }

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
        public int state = 0;

        public ProgramState() { }

        public void reset() {
            state = 0;
        }
    }

    class Instruction {
        public int jump;
        public int opCode;
        public int opIndex;
        public int instInput;
        public int[] paramModes;
        public int result;
        public ProgramState programState;

        public bool hasResult;
        public bool needsInput;

        public Instruction(int[] memory, int index, ProgramState programState) {
            opCode = memory[index] % 100;
            opIndex = index;
            var paramNum = memory[index] / 100;
            this.programState = programState;
            instInput = 0;

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
                    memory[memory[opIndex+1]] = instInput;
                } else {
                    if (paramModes[0] == 1) {
                        result = memory[opIndex+1];
                    } else {
                        result = memory[memory[opIndex+1]];
                    }
                    programState.state = result;
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

using System;
using System.IO;
using System.Linq;
using static System.Console;

// For solution 1, input '1'
// For solution 2, input '5'

namespace Day05 {
    class Program {
        static void Main(string[] args) {
            int[] memory = File.ReadAllText("input.txt")
                .TrimEnd('\n')
                .Split(',')
                .Select(r => Int32.Parse(r))
                .ToArray();

            Instruction inst = new Instruction(memory, 0);

            while (true) {
                var cont = inst.run(memory);
                if (cont == false) {
                    break;
                }
                inst = new Instruction(memory, inst.opIndex + inst.jump);
            }
        }
    }

    class Instruction {
        public int jump;
        public int opCode;
        public int opIndex;
        public int[] paramModes;

        public Instruction(int[] memory, int index) {
            opCode = memory[index] % 100;
            opIndex = index;
            var paramNum = memory[index] / 100;

            if (opCode == 1 || opCode == 2 || opCode == 7 || opCode == 8) {
                paramModes = new int[] {0, 0, 0};
                jump = 4;
            } else if (opCode == 3 || opCode == 4) {
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
                    Write("Input value: ");
                    var userInput = ReadLine();

                    memory[memory[opIndex+1]] = Int32.Parse(userInput.TrimEnd('\n'));
                } else {
                    if (paramModes[0] == 1) {
                        WriteLine($"Output: {memory[opIndex+1]}");
                    } else {
                        WriteLine($"Output: {memory[memory[opIndex+1]]}");
                    }
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

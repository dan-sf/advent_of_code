import java.io.File

fun solution1() {
    val lines = File("input.txt").readLines()

    var x = 1
    var cycles = 0
    var output = 0

    for (value in lines) {
        val splitValue = value.split(' ')

        if (splitValue[0] == "noop") {
            cycles += 1
            if (cycles == 20 || (cycles-20) % 40 == 0) {
                output += x * cycles
            }
        } else {
            cycles += 1
            if (cycles == 20 || (cycles-20) % 40 == 0) {
                output += x * cycles
            }

            cycles += 1
            if (cycles == 20 || (cycles-20) % 40 == 0) {
                output += x * cycles
            }

            x += splitValue[1].toInt()
        }
    }

    println(output)
}

fun solution2() {
    val lines = File("input.txt").readLines()

    var spritePos = 1
    data class Cycle(var num: Int, var pos: Int)
    var cycle = Cycle(0, 1)

    fun processCycle(cycle: Cycle) {
        if (spritePos-1 == cycle.pos || spritePos == cycle.pos || spritePos+1 == cycle.pos) print("#") else print(".")
        cycle.num += 1
        cycle.pos = cycle.num % 40
        if (cycle.num % 40 == 0) println()
    }

    for (value in lines) {
        val splitValue = value.split(' ')

        if (splitValue[0] == "noop") {
            processCycle(cycle)
        } else {
            processCycle(cycle)
            processCycle(cycle)
            spritePos += splitValue[1].toInt()
        }
    }
}

fun main(args: Array<String>) {
    solution1()
    solution2()
}


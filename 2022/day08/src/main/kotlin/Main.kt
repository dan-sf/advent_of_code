import java.io.File
import kotlin.math.max

fun solution1() {
    val lines = File("input.txt").readLines()

    val grid = lines.map({
        it.toList()
    })

    var output = 4 * grid[0].size - 4

    for (row in 1 until grid.size-1) {
        for (col in 1 until grid[0].size-1) {
            val cur = grid[row][col]
            var found = false

            // Originally I wanted to have each of these blocks in their own
            // scope with found initialized in each block. This didn't work due
            // to the "'break' or 'continue' jumps across a function or a class
            // boundary" error. I don't think there is a clean way around this
            // in kotlin, the only way I found to make it work here was putting
            // these into 'if (true)' blocks, which is a bit of a let down

            found = false
            for (up in row-1 downTo 0) {
                if (grid[up][col] >= cur) {
                    found = true
                    break
                }
            }
            if (!found) {
                output += 1
                continue
            }

            found = false
            for (down in row+1 .. grid.size-1) {
                if (grid[down][col] >= cur) {
                    found = true
                    break
                }
            }
            if (!found) {
                output += 1
                continue
            }

            found = false
            for (left in col-1 downTo 0) {
                if (grid[row][left] >= cur) {
                    found = true
                    break
                }
            }
            if (!found) {
                output += 1
                continue
            }

            found = false
            for (right in col+1 .. grid[0].size-1) {
                if (grid[row][right] >= cur) {
                    found = true
                    break
                }
            }
            if (!found) {
                output += 1
                continue
            }
        }
    }

    println(output)
}

fun solution2() {
    val lines = File("input.txt").readLines()

    val grid = lines.map({
        it.toList()
    })

    var output = 0

    for (row in 0 until grid.size) {
        for (col in 0 until grid[0].size) {
            val cur = grid[row][col]
            var score = 1
            var count = 0

            count = 0
            for (up in row-1 downTo 0) {
                count += 1
                if (grid[up][col] >= cur) {
                    break
                }
            }
            score *= count

            count = 0
            for (down in row+1 .. grid.size-1) {
                count += 1
                if (grid[down][col] >= cur) {
                    break
                }
            }
            score *= count

            count = 0
            for (left in col-1 downTo 0) {
                count += 1
                if (grid[row][left] >= cur) {
                    break
                }
            }
            score *= count

            count = 0
            for (right in col+1 .. grid[0].size-1) {
                count += 1
                if (grid[row][right] >= cur) {
                    break
                }
            }
            score *= count

            //if (score > output) {
            //    println(row to col to score)
            //}
            output = max(output, score)
        }
    }

    println(output)
}

fun main(args: Array<String>) {
    solution1()
    solution2()
}


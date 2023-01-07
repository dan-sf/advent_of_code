import java.io.File
import kotlin.math.abs

// Sigh, wanted to use 'MutablePair' but we don't have a mutable pair data type
// so just using a data class here (would actually make more sense to call this
// Pos, with x/y as fields, but leaving this way to highlight MutablePair absence)
data class MutablePair(var first: Int, var second: Int)

fun solution1() {
    val lines = File("input.txt").readLines()

    var headPos = MutablePair(0, 0)
    var tailPos = MutablePair(0, 0)

    val tailLocations: MutableSet<MutablePair> = mutableSetOf()
    tailLocations.add(tailPos.copy())

    for (line in lines) {
        var (direction, steps) = line.split(' ')

        var move = when (direction) {
            "L" -> -1 to 0
            "R" -> 1 to 0
            "U" -> 0 to 1
            "D" -> 0 to -1
            else -> 0 to 0
        }

        for (step in 0 until steps.toInt()) {
            headPos.first += move.first
            headPos.second += move.second

            val xDist = headPos.first - tailPos.first
            val yDist = headPos.second - tailPos.second
            val mDist = abs(xDist) + abs(yDist)

            when (mDist) {
                0, 1 -> continue
                2 -> {
                    if (abs(xDist) == abs(yDist))
                        continue // Diagonal case

                    if (xDist == 0) {
                        tailPos.second += if (yDist > 0) 1 else -1
                    } else {
                        tailPos.first += if (xDist > 0) 1 else -1
                    }
                }
                3 -> {
                    tailPos.first += if (xDist > 0) 1 else -1
                    tailPos.second += if (yDist > 0) 1 else -1
                }
                else -> println("unreachable")
            }
            tailLocations.add(tailPos.copy())
        }
    }

    println(tailLocations.size)
}

fun solution2() {
    val lines = File("input.txt").readLines()

    val rope = listOf(
        MutablePair(0, 0),
        MutablePair(0, 0),
        MutablePair(0, 0),
        MutablePair(0, 0),
        MutablePair(0, 0),
        MutablePair(0, 0),
        MutablePair(0, 0),
        MutablePair(0, 0),
        MutablePair(0, 0),
        MutablePair(0, 0),
    )

    val tailLocations: MutableSet<MutablePair> = mutableSetOf()
    tailLocations.add(rope[9].copy())

    for (line in lines) {
        var (direction, steps) = line.split(' ')

        var move = when (direction) {
            "L" -> -1 to 0
            "R" -> 1 to 0
            "U" -> 0 to 1
            "D" -> 0 to -1
            else -> 0 to 0
        }

        for (step in 0 until steps.toInt()) {
            var headPos = rope[0]
            headPos.first += move.first
            headPos.second += move.second
            for (i in 1 until rope.size) {
                val tailPos = rope[i]

                val xDist = headPos.first - tailPos.first
                val yDist = headPos.second - tailPos.second
                val mDist = abs(xDist) + abs(yDist)

                when (mDist) {
                    0, 1 -> break // If we aren't going to move the tail here, nothing after will move, so we can break
                    2 -> {
                        if (abs(xDist) == abs(yDist)) {
                            break // Diagonal case
                        } else {
                            if (xDist == 0) {
                                tailPos.second += if (yDist > 0) 1 else -1
                            } else {
                                tailPos.first += if (xDist > 0) 1 else -1
                            }
                        }
                    }
                    3, 4 -> { // Extra case needed here for longer rope
                        tailPos.first += if (xDist > 0) 1 else -1
                        tailPos.second += if (yDist > 0) 1 else -1
                    }
                    else -> println("unreachable")
                }

                headPos = rope[i]
            }
            tailLocations.add(rope[9].copy())
        }
    }

    println(tailLocations.size)
}

fun main(args: Array<String>) {
    solution1()
    solution2()
}


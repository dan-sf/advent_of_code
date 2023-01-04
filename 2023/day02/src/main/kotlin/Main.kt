import java.io.File
import kotlin.math.max

fun solution1() {
    val lines = File("input.txt").readLines()
    val moves = lines.map({it.split(" ")})

    val (win, tie, loss) = arrayOf(6, 3, 0)
    val (rock, paper, scissors) = arrayOf(1, 2, 3)
    var score = 0
    for (move in moves) {
        when (move[1]) {
            "X" -> {
                when (move[0]) {
                    "A" -> {score += tie + rock}
                    "B" -> {score += loss + rock}
                    "C" -> {score += win + rock}
                }
            }
            "Y" -> {
                when (move[0]) {
                    "A" -> {score += win + paper}
                    "B" -> {score += tie + paper}
                    "C" -> {score += loss + paper}
                }
            }
            "Z" -> {
                when (move[0]) {
                    "A" -> {score += loss + scissors}
                    "B" -> {score += win + scissors}
                    "C" -> {score += tie + scissors}
                }
            }
        }
    }
    println(score)
}

fun solution2() {
    val lines = File("input.txt").readLines()
    val moves = lines.map({it.split(" ")})

    val (win, tie, loss) = arrayOf(6, 3, 0)
    val (rock, paper, scissors) = arrayOf(1, 2, 3)
    var score = 0
    for (move in moves) {
        when (move[1]) {
            "X" -> { // lose
                when (move[0]) {
                    "A" -> {score += loss + scissors}
                    "B" -> {score += loss + rock}
                    "C" -> {score += loss + paper}
                }
            }
            "Y" -> { // tie
                when (move[0]) {
                    "A" -> {score += tie + rock}
                    "B" -> {score += tie + paper}
                    "C" -> {score += tie + scissors}
                }
            }
            "Z" -> { // win
                when (move[0]) {
                    "A" -> {score += win + paper}
                    "B" -> {score += win + scissors}
                    "C" -> {score += win + rock}
                }
            }
        }
    }
    println(score)
}

fun main(args: Array<String>) {
    solution1()
    solution2()
}


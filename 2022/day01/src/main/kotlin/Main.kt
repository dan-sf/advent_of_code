import java.io.File
import kotlin.math.max

fun solution1() {
    val lines = File("input.txt").readLines()

    var maxCals = 0
    var tmp = 0
    for (line in lines) {
        if (line == "") {
            maxCals = max(maxCals, tmp)
            tmp = 0
        } else {
            tmp += line.toInt()
        }
    }

    println(maxCals)
}

fun solution2() {
    val lines = File("input.txt").readLines()

    // Lets try this with a more functional approach
    var group = 0
    val output = lines.map({
        if (it == "") {
            group += 1
        }
        Pair(group, it)
    }).filter({
        it.second != ""
    }).groupBy({
        it.first
    }).map({
        it.value.map({
            it.second.toInt()
        }).sum()
    }).sortedDescending().take(3).sum()

    println(output)
}

fun main(args: Array<String>) {
    solution1()
    solution2()
}


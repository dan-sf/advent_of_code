import java.io.File

fun solution1() {
    val lines = File("input.txt").readLines()

    val output = lines.map({
        val (elfA, elfB) = it.split(',')

        val splitA = elfA.split('-')
        val rangeA = splitA[0].toInt() .. splitA[1].toInt()
        val setA = rangeA.toSet()

        val splitB = elfB.split('-')
        val rangeB = splitB[0].toInt() .. splitB[1].toInt()
        val setB = rangeB.toSet()

        if (setA.containsAll(setB) or setB.containsAll(setA)) {
            1
        } else {
            0
        }
    }).sum()
    println(output)
}

fun solution2() {
    val lines = File("input.txt").readLines()

    val output = lines.map({
        val (elfA, elfB) = it.split(',')

        val splitA = elfA.split('-')
        val rangeA = splitA[0].toInt() .. splitA[1].toInt()
        val setA = rangeA.toSet()

        val splitB = elfB.split('-')
        val rangeB = splitB[0].toInt() .. splitB[1].toInt()
        val setB = rangeB.toSet()

        if (setA.intersect(setB).size > 0) {
            1
        } else {
            0
        }
    }).sum()
    println(output)

}

fun main(args: Array<String>) {
    // Note: This could be done much more efficiently by using just the start
    // and end ints of the range, here we use sets which isn't really needed
    solution1()
    solution2()
}


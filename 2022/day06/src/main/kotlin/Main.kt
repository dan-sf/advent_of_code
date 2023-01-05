import java.io.File

fun solution1() {
    val lines = File("input.txt").readLines()

    val queue: ArrayDeque<Char> = ArrayDeque()

    for ((index, value) in lines[0].withIndex()) {
        queue.addFirst(value)
        if (queue.size > 4) {
            queue.removeLast()
        }
        if (queue.size == 4 && queue.toSet().size == 4) {
            println(index + 1)
            return
        }
    }
}

fun solution2() {
    val lines = File("input.txt").readLines()

    val queue: ArrayDeque<Char> = ArrayDeque()

    for ((index, value) in lines[0].withIndex()) {
        queue.addFirst(value)
        if (queue.size > 14) {
            queue.removeLast()
        }
        if (queue.size == 14 && queue.toSet().size == 14) {
            println(index + 1)
            return
        }
    }
}

fun main(args: Array<String>) {
    solution1()
    solution2()
}


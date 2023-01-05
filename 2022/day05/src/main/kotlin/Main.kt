import java.io.File

fun solution1() {
    val lines = File("input.txt").readLines()

    val stackLocation: Map<Int, Char> = lines[8].withIndex().associate({
        (index, value) -> Pair(index, value)
    }).filter({
        it.value != ' '
    })

    println(stackLocation)
    val numbers = "123456789"
    val stacks: Map<Char, MutableList<Char>> = numbers.associateWith({
        mutableListOf()
    })

    for (i in 7 downTo 0) {
        println(lines[i])
        for ((index, ch) in lines[i].withIndex()) {
            if (ch in 'A' .. 'Z') {
                stacks[stackLocation[index]]!!.add(ch)
            }
        }
    }
    println(stacks)

    for (i in 10 until lines.size) {
        val splitStr = lines[i].split(' ')
        val number = splitStr[1].toInt()
        val start = splitStr[3].toCharArray()[0]
        val end = splitStr[5].toCharArray()[0]

        for (i in 0 until number) {
            val elm = stacks[start]!!.removeAt(stacks[start]!!.size - 1)
            stacks[end]!!.add(elm)
        }
    }
    println(stacks)
    println(stacks['1'])

    var output = "" // Note: If this was a large string we should use StringBuilder() instead
    for (ch in numbers) {
        output += stacks[ch]!![stacks[ch]!!.size - 1]
    }
    println(output)
}

fun solution2() {
    val lines = File("input.txt").readLines()

    val stackLocation: Map<Int, Char> = lines[8].withIndex().associate({
        (index, value) -> Pair(index, value)
    }).filter({
        it.value != ' '
    })

    println(stackLocation)
    val numbers = "123456789"
    val stacks: Map<Char, MutableList<Char>> = numbers.associateWith({
        mutableListOf()
    })

    for (i in 7 downTo 0) {
        println(lines[i])
        for ((index, ch) in lines[i].withIndex()) {
            if (ch in 'A' .. 'Z') {
                stacks[stackLocation[index]]!!.add(ch)
            }
        }
    }
    println(stacks)

    for (i in 10 until lines.size) {
        val splitStr = lines[i].split(' ')
        val number = splitStr[1].toInt()
        val start = splitStr[3].toCharArray()[0]
        val end = splitStr[5].toCharArray()[0]

        val (stayPart, movePart) = stacks[start]!!.withIndex().partition({
            (index, value) -> index < (stacks[start]!!.size - number)
        })

        val stay = stayPart.map({it.value})
        val move = movePart.map({it.value})

        stacks[start]!!.removeAll({true})
        stacks[start]!!.addAll(stay)
        stacks[end]!!.addAll(move)
    }
    println(stacks)
    println(stacks['1'])

    var output = "" // Note: If this was a large string we should use StringBuilder() instead
    for (ch in numbers) {
        output += stacks[ch]!![stacks[ch]!!.size - 1]
    }
    println(output)
}

fun main(args: Array<String>) {
    solution1()
    solution2()
}


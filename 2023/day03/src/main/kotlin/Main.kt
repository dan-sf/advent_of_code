import java.io.File

fun solution1() {
    val lines = File("input.txt").readLines()

    val half = lines.map({
        val mid = it.length / 2
        it.substring(0, mid).toCharArray() to it.substring(mid).toCharArray()
    })

    val errors = half.map({
        val errorItem = it.first.toSet().intersect(it.second.toSet())
        errorItem.elementAt(0)
    })

    var pri = 1
    val priMap: MutableMap<Char, Int> = mutableMapOf()
    for (ch in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ") {
        priMap[ch] = pri
        pri += 1
    }

    val output = errors.map({
        priMap[it]!!
    }).sum()

    println(output)
}

fun solution2() {
    val lines = File("input.txt").readLines()

    val groups = lines.mapIndexed({
        index, value -> Pair(index / 3, value)
    }).groupBy({
        it.first
    }).map({
        it.value.map({
            it.second.toSet()
        })
    })

    val badges = groups.map({
        it[0].intersect(it[1]).intersect(it[2])
    })

    var pri = 1
    val priMap: MutableMap<Char, Int> = mutableMapOf()
    for (ch in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ") {
        priMap[ch] = pri
        pri += 1
    }

    val output = badges.map({
        priMap[it.elementAt(0)]!!
    }).sum()

    println(output)
}

fun solution2B() {
    val lines = File("input.txt").readLines()

    val priMap = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".withIndex()
        .associate({
            (index, value) -> Pair(value, index + 1)
        })

    val output = lines.mapIndexed({
        index, value -> Pair(index / 3, value.toSet())
    }).groupBy({
        it.first
    }).map({
        val group = it.value
        val key = group[0].second.intersect(group[1].second)
            .intersect(group[2].second).elementAt(0)
        priMap[key]!!
    }).sum()

    println(output)
}

fun solution2Imp() {
    val lines = File("input.txt").readLines()

    val groups: MutableMap<Int, MutableList<Set<Char>>> = mutableMapOf()
    for ((index, element) in lines.withIndex()) {
        if ((index / 3) in groups) {
            groups[index / 3]!!.add(element.toSet())
        } else {
            groups[index / 3] = mutableListOf(element.toSet())
        }
    }

    var pri = 1
    val priMap: MutableMap<Char, Int> = mutableMapOf()
    for (ch in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ") {
        priMap[ch] = pri
        pri += 1
    }

    var output = 0
    for ((key, value) in groups) {
        val badge = value[0].intersect(value[1]).intersect(value[2])
        output += priMap[badge.elementAt(0)]!!
    }
    println(output)
}

fun main(args: Array<String>) {
    solution1()
    solution2()
    // Kind of interesting, here is the same solution in inperative style. I
    // think I actually prefer this over the more functional solution
    solution2Imp()
    // Here is a more compact functional version of this solution, this one
    // makes me lean back towards the functional version slightly. Hard to say
    // which is 'better' overall
    solution2B()
}


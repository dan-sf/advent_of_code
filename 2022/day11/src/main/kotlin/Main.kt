import java.io.File

class Monkey(val items: MutableList<Int>, val operationOp: String, val operationVal: String, var testVal: Int, val trueTest: Int, val falseTest: Int) {
    var inspectCount = 0
    fun getOpOutput(old: Int): Int {
        val rhs = if (operationVal == "old") old else operationVal.toInt()

        if (operationOp == "+")
            return old + rhs
        if (operationOp == "*")
            return old * rhs

        // Should be un-reachable
        assert(false)
        return rhs
    }

    fun getOpOutput2(old: Int): Int {
        val rhs = if (operationVal == "old") 1 else operationVal.toInt()

        if (operationOp == "+")
            return old + rhs
        if (operationOp == "*") {
            return if (old % rhs == 0) old else old * rhs
        }

        // Should be un-reachable
        assert(false)
        return rhs
    }
}

val cache = HashMap<Int, Int>()

fun primeFactorization(number: Int): Int {
    if (cache.contains(number)) {
        return cache[number]!!
    }
    val result = mutableListOf<Int>(1)
    var n = number
    for (i in 2..n) {
        while (n % i == 0) {
            result.add(i)
            n /= i
        }
    }
    val output = result.distinct().reduce({a, b -> a * b})
    cache[number] = output
    return output
}

fun solution1() {
    val lines = File("input.txt").readLines()

    var lineIndex = 0

    val monkeys: MutableList<Monkey> = mutableListOf()

    while (lineIndex < lines.size) {
        if (lines[lineIndex].startsWith("Monkey")) {
            lineIndex += 1
            var line = lines[lineIndex]
            val items = line.split(":")[1].replace(" ", "").split(',').map({it.toInt()}).toMutableList()
            //println(items)

            lineIndex += 1
            line = lines[lineIndex]
            val operation = line.split(" ")
            val operationVal = operation[operation.size-1]
            val operationOp = operation[operation.size-2]

            lineIndex += 1
            line = lines[lineIndex]
            val test = line.split(" ")
            val testVal = test[test.size-1].toInt()

            lineIndex += 1
            line = lines[lineIndex]
            val trueLine = line.split(" ")
            val trueTest = trueLine[trueLine.size-1].toInt()

            lineIndex += 1
            line = lines[lineIndex]
            val falseLine = line.split(" ")
            val falseTest = falseLine[falseLine.size-1].toInt()

            monkeys.add(Monkey(items, operationOp, operationVal, testVal, trueTest, falseTest))
        } else {
            lineIndex += 1
        }
    }

    for (round in 1..20) {
        for (i in 0..monkeys.size-1) {
            while (monkeys[i].items.size > 0) {
                var item = monkeys[i].items[0]

                monkeys[i].items.removeAt(0)
                monkeys[i].inspectCount += 1
                item = monkeys[i].getOpOutput(item)

                item /= 3

                if (item % monkeys[i].testVal == 0) monkeys[monkeys[i].trueTest].items.add(item)
                else monkeys[monkeys[i].falseTest].items.add(item)
            }
        }
    }

    // println()
    // for (i in 0..monkeys.size-1) {
    //     println(monkeys[i].items)
    //     println(monkeys[i].inspectCount)
    // }

    val business = monkeys
        .map({it.inspectCount})
        .sortedDescending()
        .take(2)
        .reduce({a, b -> a * b})

    println(business)

}

// This solution finds the correct answer at round 20 for the test data.
// However, it is far too slow. For 20 rounds it takes ~1min, I think the
// solution has something to do with prime numbers but I'm not sure how to work
// that in in a fast way. Moving on, will try to come back to this one.
fun solution2() {
    val lines = File("input.test.txt").readLines()

    var lineIndex = 0

    val monkeys: MutableList<Monkey> = mutableListOf()

    var opPlus = 0
    var opMul = 0


    while (lineIndex < lines.size) {
        if (lines[lineIndex].startsWith("Monkey")) {
            lineIndex += 1
            var line = lines[lineIndex]
            val items = line.split(":")[1].replace(" ", "").split(',').map({it.toInt()}).toMutableList()
            //println(items)

            lineIndex += 1
            line = lines[lineIndex]
            val operation = line.split(" ")
            val operationVal = operation[operation.size-1]
            val operationOp = operation[operation.size-2]

            lineIndex += 1
            line = lines[lineIndex]
            val test = line.split(" ")
            val testVal = test[test.size-1].toInt()

            lineIndex += 1
            line = lines[lineIndex]
            val trueLine = line.split(" ")
            val trueTest = trueLine[trueLine.size-1].toInt()

            lineIndex += 1
            line = lines[lineIndex]
            val falseLine = line.split(" ")
            val falseTest = falseLine[falseLine.size-1].toInt()

            monkeys.add(Monkey(items, operationOp, operationVal, testVal, trueTest, falseTest))
        } else {
            lineIndex += 1
        }
    }

    for (round in 1..20) {
        for (i in 0..monkeys.size-1) {
            while (monkeys[i].items.size > 0) {
                var item = monkeys[i].items[0]
                monkeys[i].items.removeAt(0)
                monkeys[i].inspectCount += 1

                if (monkeys[i].operationOp == "+") opPlus += 1
                else opMul += 1


                // var newItem = 1
                // val _item = monkeys[i].getOpOutput2(item)

                // if (_item % 2 == 0) newItem *= 2
                // if (_item % 3 == 0) newItem *= 3
                // if (_item % 5 == 0) newItem *= 5
                // if (_item % 7 == 0) newItem *= 7
                // if (_item % 11 == 0) newItem *= 11
                // if (_item % 13 == 0) newItem *= 13
                // if (_item % 17 == 0) newItem *= 17
                // if (_item % 19 == 0) newItem *= 19
                // if (_item % 21 == 0) newItem *= 21

                // item = newItem


                //val newItem = monkeys[i].getOpOutput2(item)
                //if (newItem < item) {
                //    item = primeFactorization(item)
                //} else {
                //    item = newItem
                //}

                item = monkeys[i].getOpOutput2(item)
                item = primeFactorization(item)

                if (item % monkeys[i].testVal == 0) monkeys[monkeys[i].trueTest].items.add(item)
                else monkeys[monkeys[i].falseTest].items.add(item)
            }
        }
        println(monkeys
            .map({it.inspectCount})
            .sortedDescending()
            .take(2)
            .reduce({a, b -> a * b}))
    }

    // println()
    // for (i in 0..monkeys.size-1) {
    //     println(monkeys[i].items)
    //     println(monkeys[i].inspectCount)
    // }

    val business = monkeys
        .map({it.inspectCount})
        .sortedDescending()
        .take(2)
        .reduce({a, b -> a * b})

    println(business)
    println("OpPlus: ${opPlus}, opMul: ${opMul}")
}

fun main(args: Array<String>) {
    solution1()
    solution2()
}


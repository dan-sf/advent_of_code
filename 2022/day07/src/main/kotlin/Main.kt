import java.io.File

enum class FileType {
    file, dir
}

class FileNode(val name: String, val fileType: FileType, val parent: FileNode? = null, var size: Int = 0) {
    val contents: MutableList<FileNode>?
    init {
        when (fileType) {
            FileType.file -> {
                contents = null
            }
            FileType.dir -> {
                size = 0
                contents = mutableListOf()
            }
        }
    }
}

fun getTotalSize(fileNode: FileNode): Int {
    when (fileNode.fileType) {
        FileType.dir -> {
            var output = 0
            for (ch in fileNode.contents!!) {
                output += getTotalSize(ch)
            }
            return output
        }
        FileType.file -> {
            return fileNode.size
        }
    }
}

fun getDirSum(fileNode: FileNode): Int {
    var output = 0
    when (fileNode.fileType) {
        FileType.dir -> {
            val dirSize = getTotalSize(fileNode)
            if (dirSize <= 100000) {
                output += dirSize
            }

            for (ch in fileNode.contents!!) {
                if (ch.fileType == FileType.dir) {
                    output += getDirSum(ch)
                }
            }

            return output
        }
        FileType.file -> {
            return output
        }
    }
}

fun getDirToDelete(fileNode: FileNode, minDelete: Int, candidate: Int): Int {
    var output = 0
    when (fileNode.fileType) {
        FileType.dir -> {
            val dirSize = getTotalSize(fileNode)

            if (dirSize < candidate && dirSize >= minDelete) {
                output = dirSize
            } else {
                output = candidate
            }

            for (ch in fileNode.contents!!) {
                if (ch.fileType == FileType.dir) {
                    output = getDirToDelete(ch, minDelete, output)
                }
            }

            return output
        }
        FileType.file -> {
            return output
        }
    }
}


fun solution1() {
    val lines = File("input.txt").readLines()

    var cur = FileNode("/", FileType.dir)
    val root = cur
    var skip = true // Skip the first line

    for (line in lines) {
        if (skip) {
            skip = false
            continue
        }

        if (line.startsWith('$')) {
            var splitCmd = line.split(' ')
            if (splitCmd[1] == "cd") {
                if (splitCmd[2] == "..") {
                    cur = cur.parent!!
                } else {
                    cur = cur.contents!!.filter({it.name == splitCmd[2]})[0]
                }
            } else {
                continue
            }
        } else {
            val splitLs = line.split(' ')
            if (splitLs[0] == "dir") {
                cur.contents!!.add(FileNode(splitLs[1], FileType.dir, cur))
            } else {
                cur.contents!!.add(FileNode(splitLs[1], FileType.file, cur, splitLs[0].toInt()))
            }
        }
    }

    println(getDirSum(root))
}

fun solution2() {
    val lines = File("input.txt").readLines()

    var cur = FileNode("/", FileType.dir)
    val root = cur
    var skip = true // Skip the first line

    for (line in lines) {
        if (skip) {
            skip = false
            continue
        }

        if (line.startsWith('$')) {
            var splitCmd = line.split(' ')
            if (splitCmd[1] == "cd") {
                if (splitCmd[2] == "..") {
                    cur = cur.parent!!
                } else {
                    cur = cur.contents!!.filter({it.name == splitCmd[2]})[0]
                }
            } else {
                continue
            }
        } else {
            val splitLs = line.split(' ')
            if (splitLs[0] == "dir") {
                cur.contents!!.add(FileNode(splitLs[1], FileType.dir, cur))
            } else {
                cur.contents!!.add(FileNode(splitLs[1], FileType.file, cur, splitLs[0].toInt()))
            }
        }
    }

    val totalSize = getTotalSize(root)
    val freeSpace = 70000000 - totalSize
    val minDelete = 30000000 - freeSpace

    println(getDirToDelete(root, minDelete, 70000000))
}

fun main(args: Array<String>) {
    solution1()
    solution2()
}


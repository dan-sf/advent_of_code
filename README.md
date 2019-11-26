Advent of Code Solutions
========================

Here are my solutions to the [Advent of Code](https://adventofcode.com/)
programming puzzles. These Christmas themed programming puzzles are released
each December by [Eric Wastl](http://was.tl/) and are a great was to learn a
new programming language.

I took a crack at these problems with the hopes of becoming more comfortable
with [Rust](https://www.rust-lang.org/). I'm [still
learning](https://www.dsfcode.com/posts/learning-rust/), (and have a ways to
go), but I'm in much better shape after working through these problems than
before I began.

My original idea was to do all the problems in both Python and Rust. I'm much
more comfortable with Python and I thought it'd be nice to have example
problems in both languages. However, doing the problems in Python then
converting to Rust felt awkward and was more time consuming than I expected
(these problems are hard, at least for me :). You can get away with a lot in
Python that won't fly in Rust. So, I ended up using only Rust after the first
few days.

Each day sub folder contains the input data and code for the puzzles of that
day. You can execute the solutions using `rustc` to compile, then run the
binary as follows:

```
git clone https://github.com/dan-sf/advent_of_code.git
cd advent_of_code/day15
rustc solution1.rs && ./solution1
rustc solution2.rs && ./solution2
```

I decided against using [Cargo](https://doc.rust-lang.org/stable/cargo/)
because I didn't want to depend on any libraries or share code between any
solutions (mostly because I wanted to get reps with the language). I also
wanted to keep things as simple as possible. I think next time I'll make use of
Cargo. It's really ergonomic and has nice features, especially if you want to
pull in external dependencies.

This was a fun experience and I learned a lot. Thanks Eric for putting together
these awesome puzzles I'm sure it's no easy task. Looking forward to next year.


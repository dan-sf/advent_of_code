use std::collections::HashSet;

// I used the decompiled output from part1 to re-write the program in rust. This program is pretty
// close to the original however I don't use a loop to devide c by 256. The vars a,b,c,... map to
// the registers 0,1,2,... I use this program to detect any repeats in the halting values, if we
// see a repeat we can just use the previous value since we know that will have the most
// instructions run before stopping

fn main() {
    let mut a: u64 = 0;
    let mut c: u64 = 0;
    let mut d: u64 = 0;
    let mut e: u64 = 0;
    let mut f: u64 = 0;

    let mut unique: HashSet<u64> = HashSet::new();

    loop {
        c = f | 65536;
        f = 7571367;

        loop {
            e = c & 255;
            f = f + e;
            f &= 16777215;
            f *= 65899;
            f &= 16777215;

            if c < 256 {
                break;
            } else {
                c /= 256;
            }
        }

        if !unique.insert(f) {
            break;
        } else {
            println!("{}", f);
        }
    }
}


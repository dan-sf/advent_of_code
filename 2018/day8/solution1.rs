use std::fs;
use std::io::Read;
use std::slice::Iter;

/// Recursively walk the tree adding up the metadata as we go
fn process_metadata(iter: &mut Iter<'_, i32>, sum: &mut i32) {
    // Pull header out
    let header: (i32, i32);
    if let Some(header_a) = iter.next() {
        if let Some(header_b) = iter.next() {
            header = (*header_a, *header_b);
        } else {
            return;
        }
    } else {
        return;
    }

    if header.0 == 0 { // Leaf node, add up metadata entries (base case)
        for _ in 0..header.1 {
            if let Some(meta) = iter.next() {
                *sum += meta;
            }
        }
        return;
    } else { // Recurse into the child nodes
        for _ in 0..header.0 {
            process_metadata(iter, sum);
        }
    }

    // Add the remaining parent node's metadata entries
    for _ in 0..header.1 {
        if let Some(meta) = iter.next() {
            *sum += meta;
        }
    }
}

fn main() {
    let mut input = fs::File::open("input.txt")
        .expect("Something went wrong reading the file");

    let mut serialized_data = String::new();
    input.read_to_string(&mut serialized_data).unwrap();

    let tree: Vec<i32> = serialized_data
        .as_str().trim_end().split(' ')
        .map(|r| r.parse::<i32>().unwrap()).collect();
    let mut metadata_sum: i32 = 0;
    process_metadata(&mut tree.iter(), &mut metadata_sum);

    println!("Metadata sum: {}", metadata_sum);
}


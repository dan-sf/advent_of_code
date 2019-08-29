use std::fs;
use std::io::Read;
use std::slice::Iter;

/// Recursively walk the tree returning leaf node sums to their parent nodes so we can sum the
/// index of each of the parent's child nodes
fn process_metadata(iter: &mut Iter<'_, i32>) -> Vec<i32> {
    // Pull header out
    let header: (i32, i32);
    if let Some(header_a) = iter.next() {
        if let Some(header_b) = iter.next() {
            header = (*header_a, *header_b);
        } else {
            return vec![];
        }
    } else {
        return vec![];
    }

    // Here we recursively get a list of the child node's values
    let mut child_node_vals: Vec<i32> = Vec::new();
    if header.0 == 0 { // Leaf node, add up metadata entries (base case)
        let mut sum = 0;
        for _ in 0..header.1 {
            if let Some(meta) = iter.next() {
                sum += meta;
            }
        }
        return vec![sum];
    } else { // Recurse into the child nodes
        for _ in 0..header.0 {
            for val in process_metadata(iter) {
                child_node_vals.push(val);
            }
        }
    }

    // Sum the metadata indexed child nodes to get and return the parent node value
    let mut parent_node_val: i32 = 0;
    for _ in 0..header.1 {
        if let Some(meta) = iter.next() {
            let index = (meta-1) as usize;
            if index < child_node_vals.len() {
                parent_node_val += child_node_vals[index];
            }
        }
    }
    return vec![parent_node_val];
}

fn main() {
    let mut input = fs::File::open("input.txt")
        .expect("Something went wrong reading the file");

    let mut serialized_data = String::new();
    input.read_to_string(&mut serialized_data).unwrap();

    let tree: Vec<i32> = serialized_data
        .as_str().trim_end().split(' ')
        .map(|r| r.parse::<i32>().unwrap()).collect();
    let sum = process_metadata(&mut tree.iter())[0];

    println!("Root node value: {:?}", sum);
}


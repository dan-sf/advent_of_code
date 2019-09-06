// This solution is pretty much brute force, however, when getting the max value for each point we
// keep track of the previously summed square and use that along with the 'L' of the new square
// moving diagonally. For example, for the following 3x3 square at point 0,0 we know the sum till
// the star at 1,1, we can remember that and sum it with the line points for the next square.
//
//              . . |
//              . * |
//              _ _ |
//
// This solution is quite slow but does work.

use std::cmp;


fn get_power(x: i32, y: i32, serial: i32) -> i32 {
    let a = x + 10;
    let b = a * y;
    let c = serial + b;
    let d = a * c;
    let e = d % 1000 / 100;
    let power = e - 5;
    power
}

fn generate_grid(serial: i32, size: usize) -> Vec<Vec<i32>> {
    let mut grid: Vec<Vec<i32>> = vec![vec![0;size];size];

    for row in 0..size {
        for col in 0..size {
            let x = col as i32;
            let y = row as i32;
            grid[row][col] = get_power(x, y, serial);
        }
    }
    grid
}

// Given a point, get max square value at that point for any square using previous sums and the new
// row/cols layered on with each new square
fn get_diagonal_max(grid: &Vec<Vec<i32>>, row: usize, col: usize) -> (i32, i32) {
    let mut max_value = grid[row][col];
    let mut previous = grid[row][col];
    let mut size = 0;

    for i in 1..(grid.len()-cmp::max(row,col)) {
        let mut current = previous;

        // Sum the new row/col of this square
        for j in 0..i {
            current += grid[row+j][col+i];
            current += grid[row+i][col+j];
        }
        current += grid[row+i][col+i];

        if current > max_value {
            max_value = current;
            size = i as i32;
        }
        previous = current;
    }
    (max_value, size+1)
}

fn get_max_coord(grid: &Vec<Vec<i32>>) -> (i32, i32, i32, i32) {
    let mut max_square = std::i32::MIN;
    let mut max_size = 0;
    let mut max_coord = (-1, -1);
    let dim = grid.len();

    // Check for the max value keeping track of the size and coord as well
    for row in 0..dim {
        for col in 0..dim {
            let (max_here, size_here) = get_diagonal_max(&grid, row, col);
            if max_here > max_square { 
                max_square = max_here;
                max_size = size_here;
                max_coord = (col as i32, row as i32); // Swap row col for x y coord
            }
        }
    }

    (max_square, max_coord.0, max_coord.1, max_size)
}

fn main() {
    let serial = 7403; // input
    let grid = generate_grid(serial, 300);
    let max_coord = get_max_coord(&grid);
    println!("Largest total power coord: {:?}", max_coord);
}


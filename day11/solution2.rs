// For this problem we just do the math, generate the grid,
// and calculate the max 3x3 square

// Power calculation
// X + 10 = A
// A * Y = B
// B + S = C
// C * A = D
// D % 1000 / 100 = E
// E - 5 = P
// 
// GRID: 300x300

fn get_power(x: i32, y: i32, serial: i32) -> i32 {
    let a = x + 10;
    let b = a * y;
    let c = serial + b;
    let d = a * c;
    let e = d % 1000 / 100;
    let power = e - 5;
    power
}

fn generate_grid(serial: i32) -> Vec<Vec<i32>> {
    let mut grid: Vec<Vec<i32>> = vec![vec![0;300];300];

    for row in 0..300 {
        for col in 0..300 {
            let x = col as i32;
            let y = row as i32;
            grid[row][col] = get_power(x, y, serial);
        }
    }
    grid
}

fn get_max_coord(grid: &Vec<Vec<i32>>) -> (i32, i32, i32) {
    let mut max_square = std::i32::MIN;
    let mut max_size = 0;
    let mut max_coord = (-1, -1);
    for size in 80..100 {
        for row in 0..(300-size) {
            for col in 0..(300-size) {
                let mut square: i32 = 0;
                for i in 0..size {
                    for j in 0..size {
                        if row+i < 300 && col+j < 300 {
                            square += grid[row+i][col+j];
                        }
                    }
                }

                if square > max_square {
                    max_square = square;
                    max_coord = ((col) as i32, (row) as i32);
                    max_size = size;
                }
            }
        }
    }
    (max_coord.0, max_coord.1, max_size as i32)
}

fn main() {
    let serial = 7403; // input
    let grid = generate_grid(serial);
    let max_coord = get_max_coord(&grid);
    println!("Largest total power coord: {:?}", max_coord);
}



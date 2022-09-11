use rand::prelude::*;
//use num::Integer;

fn get_adjacent_element_pos(pos: Vec<usize>, size: Vec<usize>) -> Vec<Vec<usize>> {
    let mut result = Vec::<Vec::<usize>>::new();
    if pos[0] > 0 {
        result.push(vec![pos[0] - 1, pos[1]]);
    }
    if pos[0] < size[0] - 1 {
        result.push(vec![pos[0] + 1, pos[1]]);
    }
    if pos[1] > 0 {
        result.push(vec![pos[0], pos[1] - 1]);
    }
    if pos[1] < size[1] - 1 {
        result.push(vec![pos[0], pos[1] + 1]);
    }
    result
}

fn populate_matrix(matrix: &mut Vec<Vec<i8>>, num_buds: u8) {
    let mut rng = thread_rng();
    for _ in 0..num_buds {
        let i: usize = rng.gen_range(1..8);
        let j: usize = rng.gen_range(1..8);
        // populate buds
        matrix[i][j] = 2;
        // populate shards
        populate_matrix_shards(matrix, vec![i, j]);
    }
}

fn populate_matrix_shards(matrix: &mut Vec<Vec<i8>>, pos: Vec<usize>) {
    for p in get_adjacent_element_pos(pos, vec![matrix.len(); 2]) {
        let e = &mut matrix[p[0]][p[1]];
        if *e == 0 {
            *e = 1;
        }
    }
}

fn print_matrix(matrix: &Vec<Vec<i8>>) {
    for i in matrix {
        for j in i {
            print!("{} ", j);
        }
        println!();
    }
}

fn main() {
    let size = 9;
    let num_buds = 9;
    let mut matrix: Vec<Vec<i8>> = vec![vec![0; size]; size];
    //print_matrix(&matrix);
    populate_matrix(&mut matrix, num_buds);
    print_matrix(&matrix);
}

// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut tr_matrix: [[i32; 3]; 3] = [[0; 3]; 3];
    for r in 0..3 {
        for c in 0..3 {
            tr_matrix[c][r] = matrix[r][c];
        }
    }
    return tr_matrix;
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for r in 0..3 {
        print!("|");
        for v in matrix[r] {
            print!(" {v}");
        }
        println!(" |");
    }
    println!("");
    return;
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);

    return;
}

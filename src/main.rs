mod library;
use rand;
pub use crate::library::*;



fn main() -> () {
    print!("Hello World!\n");
    sudoku_3d::test_lib();
    let mut rand_sudoku: Vec<Vec<Vec<Option<usize>>>> = Vec::new();
    for _x in 0..27 {
        let mut y_vec: Vec<Vec<Option<usize>>> = Vec::new();

        for _y in 0..27 {
            let mut z_vec: Vec<Option<usize>> = Vec::new();

            for _z in 0..27 {
                let val = rand::random_range(0..=3);

                if val == 0 {
                    z_vec.push(None);
                }
                else {
                    z_vec.push(Some(val));
                }
            }
            y_vec.push(z_vec);
        }
        rand_sudoku.push(y_vec);
    }
    sudoku_3d::print_sudoku(&rand_sudoku);
    sudoku_3d::solve_sudoku(rand_sudoku);

                
}

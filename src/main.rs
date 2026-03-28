mod library;
pub use crate::library::*;



fn main() -> () {
    print!("Hello World!\n");
    sudoku_3d::test_lib();
    let rand_sudoku = sudoku_3d::rand_sudoku_no_rules(27);
    sudoku_3d::print_sudoku(&rand_sudoku);
    sudoku_3d::solve_sudoku(&rand_sudoku);
}

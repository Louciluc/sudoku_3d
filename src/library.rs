pub mod sudoku_3d {
    pub fn solve_sudoku(sdk: Vec<Vec<Vec<Option<usize>>>>) -> Vec<Vec<Vec<Option<usize>>>> {
        // return 3-dimensional vector of int
        get_remaining(&sdk, &(0, 0, 0));
        sdk
    }

    pub fn print_sudoku(sdk: &Vec<Vec<Vec<Option<usize>>>>) -> () {
        print!("\n");
        let sdk_size = sdk.len();
        print!("Size of Sudoku: {}\n", sdk_size);
        let box_size = usize::try_from((sdk_size).ilog(3)).unwrap();
        print!("Size of one Box: {}\n", box_size);
        print!("\n");
        for x in (0..sdk_size).step_by(box_size) {
            print!("Layer {x}\n");
            for y in 0..sdk_size {
                for z in 0..sdk_size {
                    print!("[");
                    for w in 0..box_size {
                        print_option(&sdk[x+w][y][z]);
                        if w<box_size { print!(","); }
                    }
                    print!("] ");
                    if (z+1) % box_size == 0 {
                        print!("| "); }
                }
                print!("\n");
                if (y+1) % box_size == 0 {print!("\n");}
            }
            print!("\n");
        }
    }
    fn print_option<T: std::fmt::Display>(opt: &Option<T>) -> () {
        match opt {
            Some(x) => print!("{x}"),
            None => print!("."),
        }
    }
    fn get_remaining(sdk: &Vec<Vec<Vec<Option<usize>>>>, pos: &(usize, usize, usize)) -> Vec<usize> {
        // Size should be the same in all dimensions
        let sdk_size = sdk.len();

        let mut remain_values: Vec<usize> = (1..sdk_size + 1).map(usize::from).collect(); // turn it from range<usize> to usize
        print!("{:?}", remain_values);

        // loop through each dimension and remove all occuring numbers
        for x in 0..sdk_size {
            match sdk[x][pos.1][pos.2] {
                Some(val) => remain_values.retain(|&x| x != val),
                None => {}
            }
        }
        for y in 0..sdk_size {
            match sdk[pos.0][y][pos.2] {
                Some(val) => remain_values.retain(|&x| x != val),
                None => {}
            }
        }
        for z in 0..sdk_size {
            match sdk[pos.0][pos.1][z] {
                Some(val) => remain_values.retain(|&x| x != val),
                None => {}
            }
        }

        print!("Remaining values at positon {:?}: {:?}", pos, remain_values);
        // loop in the current box
        let box_size = usize::try_from(sdk_size.ilog(3)).unwrap();
        /*
        let x = pos.0 % &box_size;
        let y = pos.1 % &box_size;
        let z = pos.2 % &box_size;
        let pos_in_box: (usize, usize, usize) = (x, y, z);
        */
        let x = pos.0 / &box_size;
        let y = pos.1 / &box_size;
        let z = pos.2 / &box_size;
        let box_: (usize, usize, usize) = (x, y, z);

        // remove all numbers which are in this box
        // might be box_size -1 has to be tested!!!!!!!!
        for x in 0..box_size {
            for y in 0..box_size {
                for z in 0..box_size {
                    match sdk[x + box_.0 * box_size][y + box_.0 * box_size][z + box_.0 * box_size] {
                        Some(val) => remain_values.retain(|&x| x != val),
                        None => {}
                    }
                }
            }
        }

        return remain_values;
    }

    pub fn test_lib() -> () {
        print!("Hello World from library.rs!");
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/

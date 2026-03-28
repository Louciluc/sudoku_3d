
pub use color_print;
pub mod sudoku_3d {
    #[derive(Clone, PartialEq, Eq)]
    pub struct Sudoku3D {
        sdk_size: usize,
        box_size: usize,
        grid: Vec<Vec<Vec<Option<usize>>>>,
    }
    impl Sudoku3D {
        pub fn new(sdk: Vec<Vec<Vec<Option<usize>>>>) -> Sudoku3D {
            let size = sdk.len();
            Sudoku3D{sdk_size: size, box_size: Sudoku3D::sdk_size_to_box_size(size), grid: sdk}
        }

        pub fn solve_sudoku_from_grid(sdk: Vec<Vec<Vec<Option<usize>>>>) -> (bool, Vec<Vec<Vec<Option<usize>>>>){
            // A wrapper function to not have to worry about types so much
            let sudoku = Sudoku3D::new(sdk);
            let result = sudoku.solve_sudoku();
            return (result.0, result.1.grid);
        }

        pub fn solve_sudoku(&self) -> (bool, Sudoku3D) {
            let mut mut_sdk = self.clone();
            let result = mut_sdk.solve_sudoku_recurs();

            let res_sdk: Sudoku3D;
            if result {
                mut_sdk.print_sudoku();
                res_sdk = mut_sdk;
                color_print::cprintln!("<g>Sudoku solved!</>");
            } else {
                self.print_sudoku();
                res_sdk = self.clone();
                color_print::cprintln!("<r>Sudoku not solvable</>");
            }
            return (result, res_sdk);
        }

        fn solve_sudoku_recurs(&mut self) -> bool {
            let mut list_remaining: Vec<RemainingAtPos> = Vec::new();
            let mut found_single_remain = false;

            'x_es: for x in 0..self.sdk_size {
                for y in 0..self.sdk_size {
                    for z in 0..self.sdk_size {
                        if self.grid[x][y][z].is_some() {continue;} 

                        let cur_remaining = RemainingAtPos{pos: (x,y,z), list: self.get_remaining(&(x,y,z))};
                        if cur_remaining.list.len() == 0 {continue;}
                        list_remaining.push(cur_remaining);

                        if list_remaining.last().unwrap().list.len() == 1 {
                            // only one option possible, put this option in the sudoku
                            found_single_remain = true;
                            break 'x_es;
                        }
                    }
                }
            }
            println!("Next iteration: length of {}", list_remaining.len());
            self.print_sudoku();
            for each in &list_remaining {
                each.print_info();
                println!("");
            } 
            if list_remaining.len() == 0 {
                // No option available, either finished or error
               return self.is_sudoku_full();
            }

            if found_single_remain {
                println!("Found just one remaining!");
                let chosen_r: &RemainingAtPos = list_remaining.last().unwrap();
                self.grid[chosen_r.pos.0][chosen_r.pos.1][chosen_r.pos.2] = Some(chosen_r.list.last().unwrap().clone());
                return self.solve_sudoku_recurs();
            } else {
                list_remaining.sort();
                let chosen_r: &RemainingAtPos = &list_remaining[0];
                for val in chosen_r.list.clone() {
                    // try every option (val) found for this cell
                    self.grid[chosen_r.pos.0][chosen_r.pos.1][chosen_r.pos.2] = Some(val.clone());
                    println!("Trying {} at {:?}:", val, chosen_r.pos);

                    let has_worked = self.solve_sudoku_recurs();
                    if has_worked {
                        return true;
                    }
                    println!("Trying {} at {:?} failed, moving on", val, chosen_r.pos);
                }
            }

            self.print_sudoku();
            false 
        }
        
        fn is_sudoku_full(&self) -> bool {
            for x in 0..self.sdk_size {
                for y in 0..self.sdk_size {
                    for z in 0..self.sdk_size {
                        match self.grid[x][y][z] {
                            Some(_) => {},
                            None => {return false;},
                        }
                    }
                }
            }
            return true;
        }

        pub fn print_sudoku(&self) -> () {
            print!("\n");

            /*
            println!("Some quick explanation:");
            color_print::cprintln!("0 -- <g>z</> -- <g>z+1</> -- <g>...</>");
            color_print::cprintln!("|\n<b>y</>\n|\n<b>y+1</>\n|\n<b>...</>");
            color_print::cprintln!("\n[<r>x</>, <r>x+1</>, <r>...</>, <r>x+box_size-1</>]");
            color_print::cprintln!("The <r>layer</> number is added to the <r>x</> coordinate\n");
            color_print::cprintln!("You're basically seeing a whole box at once\n");
            color_print::cprintln!("[ 1, <r>2</>, <r>3</>] [ <g>4</>, 5, 6] [ <g>7</>, 8, 9]  ...");
            color_print::cprintln!("[ <b>2</>, 3, 4] [ 5, 6, 7] [ 8, 9,10]  ...");
            color_print::cprintln!("[ <b>3</>, 4, 5] [ 6, 7, 8] [ 9,10,11]  ...");
            println!("...\n");

            println!("Size of Sudoku: {}", self.sdk_size);

            println!("Size of one Box: {}", self.box_size);
            print!("\n");
            */
            let str_max_size = self.sdk_size.to_string().len();

            for x in (0..self.sdk_size).step_by(self.box_size) {
                print!("Layer {x}\n");
                for y in 0..self.sdk_size {
                    for z in 0..self.sdk_size {
                        print!("[");
                        // s_x stands for small_x because its the x-coord in the box
                        for s_x in 0..self.box_size {
                            Sudoku3D::print_option(&self.grid[x+s_x][y][z], str_max_size);
                            if s_x < self.box_size-1 { print!(","); }
                        }
                        print!("] ");
                        if (z+1) % self.box_size == 0 {
                            print!("  "); }
                    }
                    print!("\n");
                    if (y+1) % self.box_size == 0 {print!("\n");}
                }
                print!("\n");
            }
        }
        fn print_option<T: std::fmt::Display>(opt: &Option<T>, str_len: usize) -> () {
            match opt {
                Some(x) => {
                    let string = x.to_string();
                    print!("{}{}", string, " ".repeat(str_len-string.len()));
                },
                None => print!("{}", (".".repeat(str_len))),
            };
            
        }
        fn get_remaining(&self, pos: &(usize, usize, usize)) -> Vec<usize> {
            let mut found_values: Vec<usize> = Vec::new();

            // loop through each dimension and remove all occuring numbers
            for x in 0..self.sdk_size {
                match self.grid[x][pos.1][pos.2] {
                    Some(val) => found_values.push(val),
                    None => {}
                }
            }
            for y in 0..self.sdk_size {
                match self.grid[pos.0][y][pos.2] {
                    Some(val) => found_values.push(val),
                    None => {}
                }
            }
            for z in 0..self.sdk_size {
                match self.grid[pos.0][pos.1][z] {
                    Some(val) => found_values.push(val),
                    None => {}
                }
            }

            // loop in the current box
            let x = pos.0 / &self.box_size;
            let y = pos.1 / &self.box_size;
            let z = pos.2 / &self.box_size;
            let box_: (usize, usize, usize) = (x, y, z);

            // remove all numbers which are in this box
            for x in 0..self.box_size {
                for y in 0..self.box_size {
                    for z in 0..self.box_size {
                        match self.grid[x + box_.0 * self.box_size]
                            [y + box_.1 * self.box_size]
                            [z + box_.2 * self.box_size] {
                                Some(val) => found_values.push(val),
                                None => {}
                        }
                    }
                }
            }
            if found_values.len() <= 1 { return found_values; }

            // remove duplicates:
            found_values.sort();
            found_values.dedup();

            //print!("Found values are: {:?}\n", found_values);

            let mut remain_values: Vec<usize> = (1..&self.sdk_size + 1).map(usize::from).collect(); // turn it from range<usize> to usize
            //print!("all values: {:?}\n", remain_values);

            // retain() keeps true and deletes false
            remain_values.retain(|x| !found_values.contains(x));

            //println!("Remaining values at positon {:?}: {:?}", pos, remain_values);

            return remain_values;
        }

        fn sdk_size_to_box_size(sdk_size: usize) -> usize {
            (sdk_size as f64).cbrt().round() as usize
        }

        pub fn rand_sudoku_no_rules(size: usize) -> Sudoku3D {
            use rand;

            let mut rand_sudoku = Sudoku3D{sdk_size : size, box_size : Sudoku3D::sdk_size_to_box_size(size), grid : Vec::new()};
            for _x in 0..size {
                let mut y_vec: Vec<Vec<Option<usize>>> = Vec::new();

                for _y in 0..size {
                    let mut z_vec: Vec<Option<usize>> = Vec::new();

                    for _z in 0..size {
                        let val = rand::random_range(0..=size);

                        if val == 0 {
                            z_vec.push(None);
                        }
                        else {
                            z_vec.push(Some(val));
                        }
                    }
                    y_vec.push(z_vec);
                }
                rand_sudoku.grid.push(y_vec);
            }
            rand_sudoku
        }
    }
    #[derive(PartialEq, Eq)]
    struct RemainingAtPos {
        pos: (usize, usize, usize),
        list: Vec<usize>,
    }
    impl RemainingAtPos {
        fn print_info(&self) {
            print!("{:?}: ", self.pos);
            for val in &self.list {
                print!("{val} ");
            }
        }
    }
    impl Ord for RemainingAtPos {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.list.len().cmp(&other.list.len())
        }
    }
    impl PartialOrd for RemainingAtPos {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(&other))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_2x2x2 () -> Vec<Vec<Vec<Option<usize>>>> {
        return vec![
            vec![vec![Some(1),Some(2),Some(3),Some(4)],
                 vec![Some(4),Some(3),Some(2),Some(1)],
                 vec![Some(3),Some(4),Some(1),Some(2)],
                 vec![Some(2),Some(1),Some(4),Some(3)]],
            vec![vec![Some(2),Some(3),Some(4),Some(1)],
                 vec![Some(3),Some(2),Some(1),Some(4)],
                 vec![Some(4),Some(1),Some(2),Some(3)],
                 vec![Some(1),Some(4),Some(3),Some(2)]],
            vec![vec![Some(4),Some(1),Some(2),Some(3)],
                 vec![Some(1),Some(4),Some(3),Some(2)],
                 vec![Some(2),Some(3),Some(4),Some(1)],
                 vec![Some(3),Some(2),Some(1),Some(4)]],
            vec![vec![Some(3),Some(4),Some(1),Some(2)],
                 vec![Some(2),Some(1),Some(4),Some(3)],
                 vec![Some(1),Some(2),Some(3),Some(4)],
                 vec![Some(4),Some(3),Some(2),Some(1)]]];
    }

    #[test]
    fn remaining_in_small_sdk() {
        let mut sdk: Vec<Vec<Vec<Option<usize>>>> = default_2x2x2();
        sdk [0][0][0] = None;
        let result = sudoku_3d::Sudoku3D::solve_sudoku_from_grid(sdk);
        assert_eq!(result, (true, default_2x2x2()));
    }


    #[test]
    fn remain_compl_in_small_sdk() {
        let mut sdk: Vec<Vec<Vec<Option<usize>>>> = default_2x2x2();
        for _i in 0..10 {
            sdk[rand::random_range(0..4)][rand::random_range(0..4)][rand::random_range(0..4)] = None;
        }
        let result = sudoku_3d::Sudoku3D::solve_sudoku_from_grid(sdk);
        assert_eq!(result, (true, default_2x2x2()));
    }
}


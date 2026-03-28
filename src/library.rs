
pub use color_print;
pub mod sudoku_3d {
    

    pub fn solve_sudoku(sdk:  &Vec<Vec<Vec<Option<usize>>>>) -> (bool, Vec<Vec<Vec<Option<usize>>>>) {
        let sdk_size = sdk.len();
        let mut mut_sdk = sdk.clone();
        (solve_sudoku_recurs(&mut mut_sdk, sdk_size), mut_sdk)
    }

    fn solve_sudoku_recurs(sdk: &mut Vec<Vec<Vec<Option<usize>>>>, sdk_size: usize) -> bool {

        let mut list_remaining: Vec<RemainingAtPos> = Vec::new();
        let mut found_single_remain = false;
        'x_es: for x in 0..sdk_size {
            for y in 0..sdk_size {
                for z in 0..sdk_size {
                    if sdk[x][y][z].is_some() {continue;} 

                    let cur_remaining = RemainingAtPos{pos: (x,y,z), list: get_remaining(&sdk, &(x,y,z))};
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
        for each in &list_remaining {
            each.print_info();
            println!("");
        } 
        if list_remaining.len() == 0 {
            // No option available, abort!
           return false;
        }

        if found_single_remain {
            println!("Found just one remaining!");
            let chosen_r: &RemainingAtPos = list_remaining.last().unwrap();
            sdk[chosen_r.pos.0][chosen_r.pos.1][chosen_r.pos.2] = Some(chosen_r.list.last().unwrap().clone());
            return solve_sudoku_recurs(sdk, sdk_size);
        } else {
            list_remaining.sort();
            let chosen_r: &RemainingAtPos = &list_remaining[0];
            for val in chosen_r.list.clone() {
                // try every option (val) found for this cell
                sdk[chosen_r.pos.0][chosen_r.pos.1][chosen_r.pos.2] = Some(val.clone());
                let has_worked = solve_sudoku_recurs(sdk, sdk_size);
                if has_worked {
                    return true;
                }
                println!("Trying {} at {:?} failed, moving on", val, chosen_r.pos);
            }
        }

       false 
    }
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
    impl PartialEq for RemainingAtPos {
        fn eq(&self, other: &Self) -> bool {
            if !self.pos.eq(&other.pos) {
                return false;
            }
            self.list.eq(&other.list)
        }
    }
    impl Eq for RemainingAtPos {}


    pub fn print_sudoku(sdk: &Vec<Vec<Vec<Option<usize>>>>) -> () {
        print!("\n");

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

        let sdk_size = sdk.len();
        let str_max_size = sdk_size.to_string().len();
        println!("Size of Sudoku: {}", sdk_size);

        let box_size = usize::try_from((sdk_size).ilog(3)).unwrap();
        println!("Size of one Box: {}", box_size);
        print!("\n");


        for x in (0..sdk_size).step_by(box_size) {
            print!("Layer {x}\n");
            for y in 0..sdk_size {
                for z in 0..sdk_size {
                    print!("[");
                    // s_x stands for small_x because its the x-coord in the box
                    for s_x in 0..box_size {
                        print_option(&sdk[x+s_x][y][z], str_max_size);
                        if s_x < box_size-1 { print!(","); }
                    }
                    print!("] ");
                    if (z+1) % box_size == 0 {
                        print!("  "); }
                }
                print!("\n");
                if (y+1) % box_size == 0 {print!("\n");}
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
    fn get_remaining(sdk: &Vec<Vec<Vec<Option<usize>>>>, pos: &(usize, usize, usize)) -> Vec<usize> {
        // Size should be the same in all dimensions
        let sdk_size = sdk.len();

        //print!("len: {}\n", sdk_size);
        
        let mut found_values: Vec<usize> = Vec::new();

        // loop through each dimension and remove all occuring numbers
        for x in 0..sdk_size {
            match sdk[x][pos.1][pos.2] {
                Some(val) => found_values.push(val),
                None => {}
            }
        }
        for y in 0..sdk_size {
            match sdk[pos.0][y][pos.2] {
                Some(val) => found_values.push(val),
                None => {}
            }
        }
        for z in 0..sdk_size {
            match sdk[pos.0][pos.1][z] {
                Some(val) => found_values.push(val),
                None => {}
            }
        }

        // loop in the current box
        let box_size = usize::try_from(sdk_size.ilog(3)).unwrap();
        
        let x = pos.0 / &box_size;
        let y = pos.1 / &box_size;
        let z = pos.2 / &box_size;
        let box_: (usize, usize, usize) = (x, y, z);

        // remove all numbers which are in this box
        for x in 0..box_size {
            for y in 0..box_size {
                for z in 0..box_size {
                    match sdk[x + box_.0 * box_size][y + box_.0 * box_size][z + box_.0 * box_size] {
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

        let mut remain_values: Vec<usize> = (1..&sdk_size + 1).map(usize::from).collect(); // turn it from range<usize> to usize
        //print!("all values: {:?}\n", remain_values);

        // retain() keeps true and deletes false
        remain_values.retain(|x| !found_values.contains(x));

        //println!("Remaining values at positon {:?}: {:?}", pos, remain_values);

        return remain_values;
    }

    pub fn test_lib() -> () {
        print!("Hello World from library.rs!");
    }

    pub fn rand_sudoku_no_rules(size: usize) -> Vec<Vec<Vec<Option<usize>>>> {
        use rand;

        let mut rand_sudoku: Vec<Vec<Vec<Option<usize>>>> = Vec::new();
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
            rand_sudoku.push(y_vec);
        }
        rand_sudoku
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

pub fn solve_sudoku(sdk: Vec<Vec<Vec<Option<usize>>>>) -> Vec<Vec<Vec<Option<usize>>>> {
    // return 3-dimensional vector of int
    get_remaining(&sdk, &(0, 0, 0));
    sdk
}

fn get_remaining(sdk: &Vec<Vec<Vec<Option<usize>>>>, pos: &(usize, usize, usize)) -> Vec<usize> {
    // Size should be the same in all dimensions
    let sdk_size = sdk.len();

    let mut remain_values: Vec<usize> = (1..sdk_size).map(usize::from).collect(); // turn it from range<usize> to usize
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

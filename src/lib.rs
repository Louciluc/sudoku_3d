pub fn solve_sudoku( sdk : Vec<Vec<Vec<Option<usize>>>>) -> Vec<Vec<Vec<Option<usize>>>>{
    // return 3-dimensional vector of int
    sdk
}

fn get_remaining(sdk : &Vec<Vec<Vec<Option<usize>>>>, pos : &(usize, usize, usize)) -> Vec<usize>{
    // Size should be the same in all dimensions
    let sdk_size = sdk.len();

    let mut remain_values : Vec<usize> = (1..sdk_size).map(usize::from).collect(); // turn it from range<usize> to usize 
    print!("{:?}", remain_values);

    // loop through each dimension and remove all occuring numbers
    //let sdk_iter = sdk.iter();
    for y in 0..sdk_size{
        for z in 0..sdk_size{
            match sdk[pos.0][y][z]{
                Some(val) => remain_values.retain(|&x| x != val),
                None => {}
            }
        }
    }

    print!("Remaining values at positon {:?}: {:?}", pos,  remain_values);
    // loop in the current box
    // TODO


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
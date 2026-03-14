pub fn solve_sudoku(&mut sdk : [[[Option<usize>]]]) -> [[[usize]]]{
    
}

fn get_remaining(&sdk : [[[Option<usize>]]], &pos : (usize, usize, usize)) -> [usize]{
    // Size should be the same in all dimensions
    sdk_size = sdk.len();

    all_values = vec![1..sdk_size];
    print!(all_values);

    // loop through each dimension and remove all occuring numbers
    
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
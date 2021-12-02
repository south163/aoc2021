fn difference_window(input_list:&Vec<u32>, window_size:u32) -> u32 {
    //want to compare the values at i and i+window_size; the intervening numbers are in both sums so can be ignored.
    let mut iter_left = input_list.iter();
    let mut iter_right = input_list.iter().skip(window_size.try_into().unwrap()).peekable();
    let mut result = 0;
    while iter_right.peek().is_some() {
        if iter_left.next().unwrap() < iter_right.next().unwrap() {
            result = result + 1;
        }
    };
    return result;
}

fn main() {
    let input: String = std::fs::read_to_string(std::env::args().skip(1).next().unwrap()).unwrap();
    let depth_list: Vec<u32> = input.lines().map(|x| x.parse().unwrap()).collect();
    println!("part 1: {}\npart 2: {}", difference_window(&depth_list, 1), difference_window(&depth_list, 3));
}

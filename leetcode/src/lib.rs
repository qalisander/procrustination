mod container_with_most_water;
mod largest_rectangle_in_histogram;
mod longest_palindromic_substring;
mod remove_nth_node_from_end_of_list;
mod rotate_image;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

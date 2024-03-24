mod container_with_most_water;
mod largest_rectangle_in_histogram;
mod longest_palindromic_substring;
mod remove_nth_node_from_end_of_list;
mod rotate_image;
mod search_in_rotated_sorted_array;
mod sort_colors;
mod unique_paths;

/// Binary search with rust api. Recursive implementation
pub fn binary_search1<T: Ord>(slice: &[T], target: &T) -> Result<usize, usize> {
    fn binary_search_rec<T: Ord>(
        slice: &[T],
        start: usize,
        end: usize,
        target: &T,
    ) -> Result<usize, usize> {
        match end - start {
            0 => Err(0),
            1 => {
                if &slice[start] == target {
                    Ok(start)
                } else {
                    Err(end)
                }
            }
            _ => {
                let mid = (start + end) / 2;
                if target < &slice[mid] {
                    binary_search_rec(slice, start, mid, target)
                } else {
                    binary_search_rec(slice, mid, end, target)
                }
            }
        }
    }
    binary_search_rec(slice, 0, slice.len(), target)
}

/// Binary search with rust api. Iterative implementation
pub fn binary_search2<T: Ord>(slice: &[T], target: &T) -> Result<usize, usize> {
    let mut start = 0;
    let mut end = slice.len();
    while end - start > 1 {
        let mid = (start + end) / 2;
        if &slice[mid] <= target {
            start = mid;
        } else {
            end = mid;
        }
    }

    if &slice[start] == target {
        Ok(start)
    } else {
        Err(start + 1)
    }
}

#[cfg(test)]
mod test {
    use crate::{binary_search1, binary_search2};

    #[test]
    pub fn check_binary_search1() {
        let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

        assert_eq!(binary_search1(&s, &13), Ok(9));
        assert_eq!(binary_search1(&s, &4), Err(7));
        assert_eq!(binary_search1(&s, &100), Err(13))
    }

    #[test]
    pub fn check_binary_search2() {
        let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

        assert_eq!(binary_search2(&s, &13), Ok(9));
        assert_eq!(binary_search2(&s, &4), Err(7));
        assert_eq!(binary_search2(&s, &100), Err(13))
    }
}

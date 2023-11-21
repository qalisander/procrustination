// https://leetcode.com/problems/rotate-image/
struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        fn rotate_circle_of_four(matrix: &mut Vec<Vec<i32>>, (i, j): (usize, usize)) {
            let last_elem_index = matrix.len() - 1;
            let mut val = matrix[i][j];
            let (mut i, mut j) = (i, j);
            for _ in 0..4 {
                let new_i = j;
                let new_j = last_elem_index - i;
                std::mem::swap(&mut matrix[new_i][new_j], &mut val);
                i = new_i;
                j = new_j;
            }
        }

        let len = matrix.len();
        // if len will be uneven we should rotate numbers adjacent to mid
        // center number if exist shouldn't be rotated
        let rotation_block_with = len / 2;
        let rotation_block_height = len / 2 + len % 2;
        for i in 0..rotation_block_with {
            for j in 0..rotation_block_height {
                rotate_circle_of_four(matrix, (i, j))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::rotate_image::Solution;

    #[test]
    fn check1() {
        let mut arg = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let exp = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
        Solution::rotate(&mut arg);
        assert_eq!(arg, exp);
    }

    #[test]
    fn check2() {
        let mut arg = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        let exp = vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ];
        Solution::rotate(&mut arg);
        assert_eq!(arg, exp);
    }
}

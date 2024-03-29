use itertools::Itertools;
use std::convert::TryInto;
use std::fmt::{Debug, Display};
use std::iter;
//use trace::trace;
//trace::init_depth_var!();

//https://www.codewars.com/kata/5a479247e6be385a41000064/train/rust
//TODO: try run Rust code from python test

type Bit = u8;
const FILLED: Bit = 1;
const EMPTY: Bit = 0;

#[derive(Debug, Copy, Clone)]
pub(crate) enum Shift {
    Available(usize),
    Mandatory(usize),
    Banned,
}

#[derive(Debug)]
struct Clues<const T: usize>([FlatClues; T]);

impl<const T: usize> From<[&[u8]; T]> for Clues<T> {
    fn from(from: [&[u8]; T]) -> Self {
        let slice_vec = from
            .iter()
            .map(|&clues| FlatClues {
                stack: clues
                    .iter()
                    .flat_map(|&clue| std::iter::repeat(1).take(clue.into()).chain([0]))
                    .collect_vec(),
                index: 0,
            })
            .collect_vec()
            .try_into()
            .unwrap();
        Clues(slice_vec)
    }
}

impl<const T: usize> Clues<T> {
    fn get_next_possible_bit_shifts(&self) -> Box<[Shift; T]> {
        let slice = self
            .0
            .iter()
            .map(|clues| {
                match clues.next_bit() {
                    Some(&EMPTY) | None => Shift::Banned,
                    Some(&FILLED) => {
                        let rest_len = clues.stack.len() - clues.index - 1;
                        match clues.current_bit() {
                            None | Some(&EMPTY) => Shift::Available(rest_len),
                            Some(&FILLED) => Shift::Mandatory(rest_len),
                            _ => unreachable!(), // TODO: remove
                        }
                    }
                    _ => unreachable!(),
                }
            })
            .collect_vec()
            .try_into()
            .unwrap();
        Box::new(slice)
    }

    fn apply_permutation(&mut self, permutation: &[Bit; T]) -> Vec<bool> {
        self.0
            .iter_mut()
            .zip(permutation)
            .map(
                |(clues, permutation_bit)| match (clues.next_bit(), *permutation_bit) {
                    (Some(&EMPTY), EMPTY) | (_, FILLED) if clues.stack.len() > 1 => {
                        // TODO: refactor
                        clues.index += 1;
                        true
                    }
                    _ => false,
                },
            )
            .collect_vec()
    }

    fn undo_permutation(&mut self, altered_bits: Vec<bool>) {
        for (clues, altered_bit) in self.0.iter_mut().zip(altered_bits) {
            if altered_bit {
                clues.index -= 1;
            }
        }
    }
}

#[derive(Debug)]
struct FlatClues {
    stack: Vec<Bit>,
    index: usize,
}

impl FlatClues {
    fn next_bit(&self) -> Option<&Bit> {
        self.stack.get(self.index)
    }
    fn current_bit(&self) -> Option<&Bit> {
        self.stack.get(self.index.checked_sub(1)?)
    }
}

pub fn solve_nonogram<const T: usize>(
    (top_clues, left_clues): ([&[Bit]; T], [&[Bit]; T]),
) -> [[Bit; T]; T] {
    let mut processed_top_clues = Clues::from(top_clues);
    let mut permutations_stack: Vec<[u8; T]> = Vec::with_capacity(T);

    let is_solved = solve_nongram_rec(
        &mut processed_top_clues,
        &left_clues,
        &mut permutations_stack,
    );

    if is_solved {
        return permutations_stack.try_into().unwrap(); // https://stackoverflow.com/questions/29570607/is-there-a-good-way-to-convert-a-vect-to-an-array
    } else {
        panic!("Solution not found")
    };

    fn solve_nongram_rec<const T: usize>(
        top_clues: &mut Clues<T>,
        left_clues: &[&[u8]; T],
        permutations_stack: &mut Vec<[u8; T]>,
    ) -> bool {
        let current_permutation_index = permutations_stack.len();
        let next_possible_bits = top_clues.get_next_possible_bit_shifts();

        let has_not_enough_len = next_possible_bits.iter().any(|shift| match shift {
            Shift::Available(size) | Shift::Mandatory(size) => {
                *size > T - current_permutation_index
            }
            _ => false,
        });
        if has_not_enough_len {
            return false;
        }

        for permutation in
            get_permutations(&next_possible_bits, left_clues[current_permutation_index])
        {
            let altered_bits = top_clues.apply_permutation(&permutation);
            permutations_stack.push(permutation);

            if permutations_stack.len() == T
                || solve_nongram_rec(top_clues, left_clues, permutations_stack)
            {
                return true;
            }
            top_clues.undo_permutation(altered_bits);
            permutations_stack.pop();
        }
        false
    }
}

pub(crate) fn get_permutations<'a, const T: usize>(
    next_possible_shifts: &'a [Shift; T],
    clues: &'a [u8],
) -> Box<dyn Iterator<Item = [u8; T]> + 'a> {
    let permutation = [0u8; T];
    return get_permutations_rec(permutation, next_possible_shifts, clues, 0);

    fn get_permutations_rec<'a, const T: usize>(
        permutation: [u8; T],
        next_possible_shifts: &'a [Shift; T],
        clues: &'a [u8],
        init_offset: usize,
    ) -> Box<dyn Iterator<Item = [u8; T]> + 'a> {
        let current_clue = match clues.first() {
            None => return Box::new(iter::once(permutation)),
            Some(&clue) => clue as usize,
        };
        let clues_sum = clues.iter().sum::<u8>() as usize;
        let clues_borders = clues.len() - 1;

        let iter = (0..T + 1 - (init_offset + clues_sum + clues_borders))
            .filter_map(move |offset| {
                let offset = init_offset + offset;

                let zeroes_range = init_offset..offset;
                let has_zeroes_valid = next_possible_shifts[zeroes_range]
                    .iter()
                    .chain(next_possible_shifts.get(current_clue + offset))
                    .all(|shift| matches!(shift, Shift::Available(_) | Shift::Banned));

                let ones_range = offset..current_clue + offset;
                let has_ones_valid = next_possible_shifts[ones_range.clone()]
                    .iter()
                    .all(|shift| matches!(shift, Shift::Available(_) | Shift::Mandatory(_)));

                if has_ones_valid && has_zeroes_valid {
                    let mut permutation = permutation;
                    permutation[ones_range].fill(1);
                    Some(get_permutations_rec(
                        permutation,
                        next_possible_shifts,
                        &clues[1..],
                        1 + offset + current_clue,
                    ))
                } else {
                    None
                }
            })
            .flatten();
        Box::new(iter)
    }
}

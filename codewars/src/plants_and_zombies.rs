//https://www.codewars.com/kata/5a5db0f580eba84589000979/train/rust
use itertools::Itertools;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
enum Tile {
    Shooter { power: usize },
    SShooter,
    Zombie { hp: usize },
    Empty,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:^3}",
            match self {
                Tile::Shooter { power } => power.to_string(),
                Tile::SShooter => "S".to_string(),
                Tile::Zombie { hp } => format!("Z{}", hp),
                Tile::Empty => ".".to_string(),
            }
        )
    }
}

#[derive(Debug)]
struct Field {
    tiles: Vec<Vec<Tile>>,
    zombies_outside: VecDeque<(usize, usize, usize)>,
    turn: usize,
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            writeln!(
                f,
                "{}\n",
                row.iter().map(|tile| tile.to_string()).collect::<String>()
            )?;
        }
        writeln!(
            f,
            "{}",
            self.zombies_outside
                .iter()
                .map(|(i, row, hp)| format!("(i: {i}; row: {row}, hp: {hp}); "))
                .collect::<String>()
        )?;
        writeln!(f, "turn: {}\n", self.turn)?;
        Ok(())
    }
}

enum GameStatus {
    ZombiesWon,
    ZombiesLost,
    Unknown,
}

impl Field {
    pub fn new(lawn: &Vec<&str>, zombies: &Vec<Vec<usize>>) -> Self {
        let tiles = lawn
            .iter()
            .map(|str| {
                str.chars()
                    .map(|ch| match ch {
                        'S' => Tile::SShooter,
                        ' ' => Tile::Empty,
                        ch => {
                            let power = ch.to_digit(10).expect("Invalid char!") as usize;
                            Tile::Shooter { power }
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();
        let zombies_outside = zombies
            .iter()
            .map(|vec| {
                vec.iter()
                    .cloned()
                    .collect_tuple::<(_, _, _)>()
                    .expect("Invalid zombie info!")
            })
            .collect::<VecDeque<(_, _, _)>>();
        Self {
            tiles,
            zombies_outside,
            turn: 0,
        }
    }

    fn zombies_make_step(&mut self) -> GameStatus {
        let mut is_zombie_exist = false;
        for i in 0..self.i_max() {
            for j in 0..self.j_max() {
                if let Tile::Zombie { .. } = self.tiles[i][j] {
                    if j == 0 {
                        return GameStatus::ZombiesWon;
                    }
                    self.tiles[i][j - 1] = self.tiles[i][j];
                    self.tiles[i][j] = Tile::Empty;
                    is_zombie_exist = true;
                }
            }
        }

        while let &Some(&(i, row, hp)) = &self.zombies_outside.front() {
            is_zombie_exist = true;
            if i != self.turn {
                break;
            }
            *self.tiles[row].last_mut().unwrap() = Tile::Zombie { hp };
            self.zombies_outside.pop_front();
        }

        if !is_zombie_exist {
            GameStatus::ZombiesLost
        } else {
            GameStatus::Unknown
        }
    }

    fn shooters_fire(&mut self) {
        self.simple_shooters_fire();
        self.s_shooters_fire();
    }

    fn simple_shooters_fire(&mut self) {
        for i in 0..self.i_max() {
            let mut total_power = 0;
            for j in 0..self.j_max() {
                match self.tiles[i][j] {
                    Tile::Shooter { power } => total_power += power,
                    _ => {
                        self.try_shoot_zombie((i, j), &mut total_power);
                    }
                }
            }
        }
    }

    fn s_shooters_fire(&mut self) {
        for j in (0..self.j_max()).rev() {
            for i in 0..self.i_max() {
                if let Tile::SShooter = self.tiles[i][j] {
                    for j1 in j.. {
                        if j1 >= self.j_max() || self.try_shoot_zombie((i, j1), &mut 1) {
                            break;
                        }
                    }
                    for d in 0.. {
                        if i + d >= self.i_max()
                            || j + d >= self.j_max()
                            || self.try_shoot_zombie((i + d, j + d), &mut 1)
                        {
                            break;
                        }
                    }
                    for d in 0.. {
                        if i < d
                            || i - d >= self.i_max()
                            || j + d >= self.j_max()
                            || self.try_shoot_zombie((i - d, j + d), &mut 1)
                        {
                            break;
                        }
                    }
                }
            }
        }
    }

    fn try_shoot_zombie(&mut self, (i, j): (usize, usize), power: &mut usize) -> bool {
        if let Tile::Zombie { hp } = self.tiles[i][j] {
            self.tiles[i][j] = if *power >= hp {
                Tile::Empty
            } else {
                Tile::Zombie { hp: hp - *power }
            };
            *power = power.saturating_sub(hp);
            true
        } else {
            false
        }
    }

    fn i_max(&mut self) -> usize {
        self.tiles.len()
    }
    fn j_max(&mut self) -> usize {
        self.tiles[0].len()
    }
}

// game rules
// - zombie makes a step. And if it's position intersect with a shooter. It dies.
// - simple shooters fire straight
// - S shooters fire straight
// - S shooters fire diagonally
// - zombies dies instantly and don't absorb any additional pellets
pub fn plants_and_zombies(lawn: &Vec<&str>, zombies: &Vec<Vec<usize>>) -> usize {
    let mut field = Field::new(lawn, zombies);
    loop {
        println!("{}", field);
        match field.zombies_make_step() {
            GameStatus::ZombiesWon => break field.turn,
            GameStatus::ZombiesLost => break 0,
            GameStatus::Unknown => {}
        }
        field.shooters_fire();
        field.turn += 1;
    }
}

#[cfg(test)]
pub mod example_tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    pub fn tests() {
        let example_tests: Vec<(Vec<&str>,Vec<Vec<usize>>,usize)> = vec![
            (
                vec![
                    "2       ",
                    "  S     ",
                    "21  S   ",
                    "13      ",
                    "2 3     "],
                vec![
                    vec![0,4,28],
                    vec![1,1,6],
                    vec![2,0,10],
                    vec![2,4,15],
                    vec![3,2,16],
                    vec![3,3,13]],
                10
            ),
            (
                vec![
                    "11      ",
                    " 2S     ",
                    "11S     ",
                    "3       ",
                    "13      "],
                vec![
                    vec![0,3,16],
                    vec![2,2,15],
                    vec![2,1,16],
                    vec![4,4,30],
                    vec![4,2,12],
                    vec![5,0,14],
                    vec![7,3,16],
                    vec![7,0,13]],
                12
            ),
            (
                vec![
                    "12        ",
                    "3S        ",
                    "2S        ",
                    "1S        ",
                    "2         ",
                    "3         "],
                vec![
                    vec![0,0,18],
                    vec![2,3,12],
                    vec![2,5,25],
                    vec![4,2,21],
                    vec![6,1,35],
                    vec![6,4,9],
                    vec![8,0,22],
                    vec![8,1,8],
                    vec![8,2,17],
                    vec![10,3,18],
                    vec![11,0,15],
                    vec![12,4,21]],
                20
            ),
            (
                vec![
                    "12      ",
                    "2S      ",
                    "1S      ",
                    "2S      ",
                    "3       "],
                vec![
                    vec![0,0,15],
                    vec![1,1,18],
                    vec![2,2,14],
                    vec![3,3,15],
                    vec![4,4,13],
                    vec![5,0,12],
                    vec![6,1,19],
                    vec![7,2,11],
                    vec![8,3,17],
                    vec![9,4,18],
                    vec![10,0,15],
                    vec![11,4,14]],
                19
            ),
            (
                vec![
                    "1         ",
                    "SS        ",
                    "SSS       ",
                    "SSS       ",
                    "SS        ",
                    "1         "],
                vec![
                    vec![0,2,16],
                    vec![1,3,19],
                    vec![2,0,18],
                    vec![4,2,21],
                    vec![6,3,20],
                    vec![7,5,17],
                    vec![8,1,21],
                    vec![8,2,11],
                    vec![9,0,10],
                    vec![11,4,23],
                    vec![12,1,15],
                    vec![13,3,22]],
                0
            ), 
            (
                vec![
                "2S12    ",
                "22S1    ",
                "5SS     ",
                "51      ",
                " 3 1    ",
            ], vec![
                vec![1,0,21,],
                vec![1,2,25,],
                vec![1,3,21,],
                vec![1,4,14,],
                vec![3,0,14,],
                vec![3,2,17,],
                vec![3,3,14,],
                vec![3,4,9,],
                vec![5,3,11,],
                vec![5,4,7,],
                vec![8,0,13,],
                vec![8,1,29,],
                vec![8,2,15,],
                vec![9,1,19,],
                vec![9,3,11,],
                vec![9,4,8,],
                vec![11,0,16,],
                vec![11,1,16,],
                vec![11,2,18,],
                vec![11,3,12,],
                vec![12,0,11,],
                vec![12,1,11,],
                vec![12,2,13,],
                vec![12,4,9,],
                vec![13,3,12,],
                vec![14,0,12,],
                vec![14,2,13,],
            ], 17),
            (
                vec![
                "2SS3         ", //0
                "SS13 1       ", //1
                "  21         ",
                "5S2S         ",
                "S123S1       ",
                "1S3S         ",
                "22   1       ",
                "5S2 S1       ",
                "13S  1       ",
                "32SS         ",
                " 4           ",
                "3 SS         ",
                "S2           ",
                "222S         ", //13
            ], 
                vec![
                    vec![0,0,56,],
                    vec![0,1,56,],
                    vec![0,2,24,],
                    vec![0,3,72,],
                    vec![0,4,72,],
                    vec![0,5,48,],
                    vec![0,6,40,],
                    vec![0,7,80,],
                    vec![0,8,48,],
                    vec![0,12,24,],
                    vec![3,10,34,],
                    vec![3,11,43,],
                    vec![4,0,30,],
                    vec![4,1,30,],
                    vec![4,2,13,],
                    vec![4,3,39,],
                    vec![4,6,21,],
                    vec![4,8,26,],
                    vec![4,9,65,],
                    vec![5,5,29,],
                    vec![5,7,49,],
                    vec![5,9,29,],
                    vec![5,10,18,],
                    vec![5,11,22,],
                    vec![5,12,14,],
                    vec![5,13,69,],
                    vec![7,13,31,], // -- winner
                    vec![8,4,55,],
                    vec![9,0,29,],
                    vec![9,1,29,],
                    vec![9,2,12,],
                    vec![9,3,37,],
                    vec![9,6,21,],
                    vec![9,8,24,],
                    vec![10,7,44,],
                    vec![10,9,28,],
                    vec![10,10,17,],
                    vec![11,4,38,],
                    vec![11,5,30,],
                    vec![11,12,15,],
                    vec![12,13,33,], // -- should be winner
                    vec![13,0,28,],
                    vec![13,1,28,],
                    vec![13,2,12,],
                    vec![13,6,20,],
                    vec![13,8,24,],
                    vec![13,11,31,],
                    vec![14,7,41,],
                    vec![14,9,28,],
                    vec![14,10,16,],
                    vec![16,3,47,],
                    vec![16,4,37,],
                    vec![16,5,26,],
                    vec![16,12,13,],
                    vec![17,13,30,],
                    vec![18,1,28,],
                    vec![18,2,12,],
                    vec![18,6,20,],
                    vec![18,8,24,],
                    vec![19,0,32,],
                    vec![19,7,40,],
                    vec![19,9,28,],
                    vec![19,10,16,],
                    vec![21,3,40,],
                    vec![21,4,36,],
                    vec![21,12,13,],
                    vec![22,5,29,],
                    vec![22,11,33,],
                    vec![23,1,28,],
                    vec![23,2,12,],
                    vec![23,6,20,],
                    vec![23,8,24,],
                    vec![23,13,33,],
                    vec![25,7,40,],
                    vec![25,9,28,],
                    vec![25,10,16,],
                    vec![26,3,38,],
                    vec![26,4,36,],
                    vec![28,0,39,],
                    vec![28,11,25,],
            ], 25),
        ];

        example_tests.into_iter().for_each(|(grid,zqueue,sol)| assert_eq!(plants_and_zombies(&grid,&zqueue),sol));
    }
}

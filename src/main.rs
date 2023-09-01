use std::{fmt::Display, time::Duration};

use Cell::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Obstacle,
    Start,
    End,
    Step(u32),
    Path,
}

impl ToString for Cell {
    fn to_string(&self) -> String {
        let c = match self {
            Empty => '_',
            Obstacle => '█',
            Start => 'S',
            End => 'E',
            Step(n) => {
                if *n > 9 {
                    '9'
                } else { 
                    match n.to_string().chars().nth(0) {
                        Some(c) => c,
                        None => '?',
                    }
                }
            },
            Path => '+',
        };
        c.to_string()
    }
}

// Coordinate system is like a screen, x is descending vertically and y increasing horizontally.
fn main() {
    let map: [[Cell; 10]; 10] = create_map(&[
        ['E', 'X', 'X', '_', '_', '_', 'X', '_', '_', '_'],
        ['_', '_', 'X', '_', 'X', '_', 'X', '_', 'X', '_'],
        ['X', '_', 'X', '_', 'X', '_', 'X', '_', 'X', '_'],
        ['_', '_', 'X', '_', 'X', '_', 'X', '_', 'X', '_'],
        ['_', 'X', 'X', '_', 'X', '_', 'X', '_', 'X', '_'],
        ['_', '_', 'X', '_', 'X', '_', 'X', '_', 'X', '_'],
        ['X', '_', 'X', '_', 'X', '_', 'X', '_', 'X', '_'],
        ['_', '_', 'X', '_', 'X', '_', 'X', '_', 'X', '_'],
        ['_', 'X', 'X', '_', 'X', '_', 'X', '_', 'X', '_'],
        ['_', '_', '_', '_', 'X', '_', '_', '_', 'X', 'S'],
    ]);
    let map2: [[Cell; 10]; 10] = create_map(&[
        ['E', '_', '_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'],
        ['_', '_', '_', '_', '_', '_', '_', '_', '_', 'S'],
    ]);
    let new_map = compute_path(&map, 100);
    display_map(&new_map);
    println!();
    let solved_map = solve_path(&new_map, (0, 0));
    display_map(&solved_map);
}

fn create_map<const M: usize, const N: usize>(map: &[[char; M]; N]) -> [[Cell; M]; N] {
    let mut new_map = [[Empty; M]; N];
    for i in 0..N {
        for j in 0..M {
            let cell = match map[i][j] {
                '_' => Empty,
                'X' => Obstacle,
                'S' => Start,
                'E' => End,
                c => panic!("Error while parsing map, unknown character '{}'", c),
            };
            new_map[i][j] = cell;
        }
    }
    new_map
}

fn display_map<const M: usize, const N: usize>(map: &[[Cell; M]; N]) {
    for i in 0..N {
        let mut line = String::new();
        for j in 0..M {
            line.push_str(&format!("{} ", &map[i][j].to_string()));
        }
        println!("{}", line);
    }
}

fn compute_path<const M: usize, const N: usize>(map: &[[Cell; M]; N], depth: u32) -> [[Cell; M]; N] {
    let mut new_map = map.clone();
    let mut found_end = false;
    let mut count = 0;
    let mut last_step_pos: Vec<(usize, usize)> = vec![];

    // Find start
    'outer: for (i, line) in new_map.iter().enumerate() {
        for (j, e) in line.iter().enumerate() {
            if *e == Start {
                last_step_pos.push((i, j));
                // dbg!(last_step_pos);
                break 'outer;
            }
        }
    }
    let (x, y) = last_step_pos[0];
    new_map[x][y] = Step(count); // Put start to Step(0)
    while !found_end && count < depth {
        std::thread::sleep(Duration::from_millis(100));
        display_map(&new_map);
        println!();
        count += 1;
        // Analyse all 4 adjacent cases
        let mut current_step_pos: Vec<(usize, usize)> = vec![];
        for (x, y) in &last_step_pos {
            if x + 1 < N {
                match new_map[x + 1][*y] {
                    Empty => {
                        new_map[x + 1][*y] = Step(count);
                        current_step_pos.push((x + 1, *y));
                    },
                    Obstacle => (),
                    Start => (),
                    End => {
                        new_map[x + 1][*y] = Step(count);
                        current_step_pos.push((x + 1, *y));
                        found_end = true;
                    },
                    Step(n) => {
                        if n > count {
                            new_map[x + 1][*y] = Step(count);
                            current_step_pos.push((x + 1, *y));
                        }
                    },
                    Path => (),
                }
            }
            if *x != 0 {
                match new_map[x - 1][*y] {
                    Empty => {
                        new_map[x - 1][*y] = Step(count);
                        current_step_pos.push((x - 1, *y));
                    },
                    Obstacle => (),
                    Start => (),
                    End => {
                        new_map[x - 1][*y] = Step(count);
                        current_step_pos.push((x - 1, *y));
                        found_end = true;
                    },
                    Step(n) => {
                        if n > count {
                            new_map[x - 1][*y] = Step(count);
                            current_step_pos.push((x - 1, *y));
                        }
                    },
                    Path => (),
                }
            }
            if y + 1 < M {
                match new_map[*x][y + 1] {
                    Empty => {
                        new_map[*x][y + 1] = Step(count);
                        current_step_pos.push((*x, y + 1));
                    },
                    Obstacle => (),
                    Start => (),
                    End => {
                        new_map[*x][y + 1] = Step(count);
                        current_step_pos.push((*x, y + 1));
                        found_end = true;
                    },
                    Step(n) => {
                        if n > count {
                            new_map[*x][y + 1] = Step(count);
                            current_step_pos.push((*x, y + 1));
                        }
                    },
                    Path => (),
                }
            }
            if *y != 0 {
                match new_map[*x][y - 1] {
                    Empty => {
                        new_map[*x][y - 1] = Step(count);
                        current_step_pos.push((*x, y - 1));
                    },
                    Obstacle => (),
                    Start => (),
                    End => {
                        new_map[*x][y - 1] = Step(count);
                        current_step_pos.push((*x, y - 1));
                        found_end = true;
                    },
                    Step(n) => {
                        if n > count {
                            new_map[*x][y - 1] = Step(count);
                            current_step_pos.push((*x, y - 1));
                        }
                    },
                    Path => (),
                }
            }
        }
        last_step_pos = current_step_pos;
    }
    new_map
}

fn solve_path<const M: usize, const N: usize>(map: &[[Cell; M]; N], end_pos: (usize, usize)) -> [[Cell; M]; N] {
    let mut new_map = map.clone();
    let (mut x, mut y) = end_pos;
    let mut count = match map[x][y] {
        Step(n) => n,
        _ => 0,
    };
    while count != 0 {
        std::thread::sleep(Duration::from_millis(100));
        display_map(&new_map);
        println!();
        if x + 1 < N {
            match new_map[x + 1][y] {
                Step(n) => {
                    if n < count {
                        new_map[x][y] = Path;
                        x = x + 1;
                        count = n;
                        continue;
                    }
                },
                _ => (),
            }
        }
        if x != 0 {
            match new_map[x - 1][y] {
                Step(n) => {
                    if n < count {
                        new_map[x][y] = Path;
                        x = x - 1;
                        count = n;
                        continue;
                    }
                },
                _ => (),
            }
        }
        if y + 1 < M {
            match new_map[x][y + 1] {
                Step(n) => {
                    if n < count {
                        new_map[x][y] = Path;
                        y = y + 1;
                        count = n;
                        continue;
                    }
                },
                _ => (),
            }
        }
        if y != 0 {
            match new_map[x][y - 1] {
                Step(n) => {
                    if n < count {
                        new_map[x][y] = Path;
                        y = y - 1;
                        count = n;
                        continue;
                    }
                },
                _ => (),
            }
        }
    }
    new_map[x][y] = Path;
    new_map
}
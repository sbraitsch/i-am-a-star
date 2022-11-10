use std::{fmt, time::Instant};

#[derive(Debug, Default, Copy, Clone)]
struct Node {
    idx: usize,
    cost: usize,
    g_cost: usize,
    h_cost: usize,
    prev: Option<usize>,
    direction: Direction
}

enum ANSII {
    Clear,
    Red,
    White,
    Yellow
}

impl fmt::Display for ANSII {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Clear => "\u{001b}[0m",
                Self::Red => "\u{001b}[31m",
                Self::White => "\u{001b}[37m",
                Self::Yellow => "\u{001b}[33m"
            }
        )
    }
}

#[derive(Debug, Default, Copy, Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    #[default] NONE
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::UP => "↑",
                Self::DOWN => "↓",
                Self::LEFT => "←",
                Self::RIGHT => "→",
                Self::NONE => "☩"
            }
        )
    }
}

type Grid = Vec<Node>;

fn main() {
    let walls = vec![5, 15, 25, 35, 45, 46, 47, 57, 64, 65, 66, 67, 77, 48];
    let mut maze = Vec::new();
    let start = (0, 0);
    let end = (8, 0);

    for i in 0..10 {
        for k in 0..10 {
            let val = if walls.contains(&((i* 10) + k)) { 100 } else { 1 };
            maze.push(Node { idx: (i * 10) + k, cost: val, h_cost: calc_h_cost((k, i), end), ..Default::default()});
        }
    }

    let now = Instant::now();
    astar(&mut maze, start, end);
    let elapsed = now.elapsed();

    print_maze(&maze);
    println!("\nTime: {:.2?}", elapsed);
}

fn astar(maze: &mut Grid, (start_x, start_y): (usize, usize), (target_x, target_y): (usize, usize)) {
    let mut open_list: Vec<usize> = Vec::new();
    let mut closed_list: Vec<usize> = Vec::new();

    let target = get_idx(target_x, target_y);

    open_list.push(get_idx(start_x, start_y));

    while !open_list.is_empty() {
        let (open_idx, current_idx) = min_by_fcost(&maze, &open_list);
        let mut current_node = maze[current_idx];
        open_list.remove(open_idx);
        
        if current_idx == target {
            maze[current_idx].cost = 0;
            while let Some(idx) = current_node.prev {
                let diff = (current_node.idx as isize) - (idx as isize);
                let dir = match diff {
                    10 => { Direction::DOWN },
                    -10 => { Direction::UP },
                    1 => { Direction::RIGHT },
                    -1 => { Direction::LEFT },
                    _ => {Direction::NONE}
                };
                maze[idx].cost = 0;
                maze[idx].direction = dir;
                current_node = maze[idx];
            };
            return;
        } else {
            closed_list.push(current_idx);

            for adj in get_adjacent(current_idx) {
                if !closed_list.contains(&adj) {
                    let cost_through_current = current_node.g_cost + maze[adj].cost;
                    if !open_list.contains(&adj) {
                        maze[adj].g_cost = cost_through_current;
                        maze[adj].prev = Some(current_idx);
                        open_list.push(adj);
                    } else {
                        if cost_through_current < maze[adj].g_cost {
                            maze[adj].g_cost = cost_through_current;
                            maze[adj].prev = Some(current_idx)
                        }
                    }
                }
            }
        }
    }
}

fn get_adjacent<'a>(idx: usize) -> Vec<usize> {
    let mut adjacent = Vec::new();
    if idx > 10 {
        adjacent.push(idx - 10)
    }
    if idx < 90 {
        adjacent.push(idx + 10)
    }
    if idx % 10 != 0 {
        adjacent.push(idx - 1)
    }
    if idx % 10 != 9 {
        adjacent.push(idx + 1)
    }

    adjacent
}

fn calc_h_cost(start: (usize, usize), end: (usize, usize)) -> usize {
    end.0.abs_diff(start.0) + end.1.abs_diff(start.1)
}

fn print_maze(maze : &Grid) {
    for node in maze {
        if node.idx % 10 == 0 { print!("\n") }
        match node.cost {
            1 => { print!("{} · {}", ANSII::White, ANSII::Clear) },
            0 => { 
                print!("{} {} {}", ANSII::Yellow, node.direction, ANSII::Clear) 
            },
            _ => { print!("{} ■ {}", ANSII::Red, ANSII::Clear) }
        }
    }
}

fn get_idx(x: usize, y: usize) -> usize {
    (y * 10) + x
}

fn min_by_fcost(maze: &Grid, open: &Vec<usize>) -> (usize, usize) {
    let mut min_idx = (0, 0);
    let mut current_min = 1000;
    for (i, idx) in open.iter().enumerate() {
        let check = &maze[*idx];
        let f_cost = check.g_cost + check.h_cost;
        if f_cost < current_min {
            current_min = f_cost;
            min_idx = (i, *idx);
        }
    }
    min_idx
}

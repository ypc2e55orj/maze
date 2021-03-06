extern crate rand;

use rand::Rng;
use std::env;
use std::process;

type Coords = Vec<Coord>;

struct Coord {
    y: usize,
    x: usize,
}

impl Coord {
    fn new(y: usize, x: usize) -> Coord {
        Coord { y: y, x: x }
    }
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

type MazeMap = Vec<Vec<bool>>;

struct MazeHelper;

impl MazeHelper {
    fn empty_map(height: usize, width: usize, edge_fill: bool, even_pillar: bool) -> MazeMap {
        let mut map = vec![vec![false; width]; height];

        for y in 0..height {
            for x in 0..width {
                if (y == 0 || y == height - 1 || x == 0 || x == width - 1) && edge_fill {
                    map[y][x] = true;
                } else if y % 2 == 0 && x % 2 == 0 && even_pillar {
                    map[y][x] = true;
                }
            }
        }

        map
    }

    fn check_input(input: usize) -> usize {
        if input % 2 == 0 {
            input + 1
        } else if input < 5 {
            5
        } else {
            input
        }
    }

    fn random(max: usize) -> usize {
        rand::thread_rng().gen_range(0, max)
    }

    fn even_random(max: usize) -> usize {
        let mut even: usize = MazeHelper::random(max);

        while even % 2 != 0 {
            even = MazeHelper::random(max);
        }

        even
    }

    // 与えられたmapにstate(true|false)と等しい座標が存在するかを確認する.
    fn is_available(map: &MazeMap, state: bool) -> bool {
        let mut is_available = false;

        for (y, y_val) in map.iter().enumerate() {
            for (x, _) in y_val.iter().enumerate() {
                if map[y][x] == state {
                    is_available = true
                }
            }
        }

        is_available
    }

    // Wall構造体が入ったベクターを受け取り,与えられた座標を含むかかどうかを確認する.
    fn is_coord_included(y: usize, x: usize, wall_coords: &Coords) -> bool {
        let mut is_coord_included = false;

        for coord in wall_coords {
            if coord.y == y && coord.x == x {
                is_coord_included = true;
            }
        }

        is_coord_included
    }

    fn encode_coord(coord: &Coord, constant: usize) -> usize {
        constant * coord.y + coord.x
    }

    fn decode_encoded_coord(encode_coord: usize, constant: usize) -> Coord {
        Coord::new(encode_coord / constant, encode_coord % constant)
    }
}

struct Maze {
    height: usize,
    width: usize,
    start: Coord,
    goal: Coord,
}

impl Maze {
    fn set_wall(y: usize, x: usize, map: &mut MazeMap, wall_coords: &mut Coords) {
        println!("generate: set: ({}, {})", y, x);
        map[y][x] = true;

        if x % 2 == 0 && y % 2 == 0 {
            println!("generate: add: ({}, {})", y, x);

            wall_coords.push(Coord::new(y, x));
        }
    }

    fn extend_wall(_y: usize, _x: usize, map: &mut MazeMap, wall_coords: &mut Coords) {
        let mut x = _x;
        let mut y = _y;

        loop {
            let mut directions = vec![];

            if !map[y - 1][x] && !MazeHelper::is_coord_included(y - 2, x, wall_coords) {
                directions.push(Direction::Up)
            }
            if !map[y + 1][x] && !MazeHelper::is_coord_included(y + 2, x, wall_coords) {
                directions.push(Direction::Down)
            }
            if !map[y][x - 1] && !MazeHelper::is_coord_included(y, x - 2, wall_coords) {
                directions.push(Direction::Right)
            }
            if !map[y][x - 1] && !MazeHelper::is_coord_included(y, x + 2, wall_coords) {
                directions.push(Direction::Left)
            }

            if directions.len() > 0 {
                Maze::set_wall(y, x, map, wall_coords);

                let mut is_wall = false;
                let random_index = MazeHelper::random(directions.len());

                match directions[random_index] {
                    Direction::Up => {
                        println!("generate: up: ({}, {})", y - 2, x);
                        is_wall = map[y - 2][x];
                        y -= 1;
                        println!("generate: y decrement: ({}, {})", y, x);
                        Maze::set_wall(y, x, map, wall_coords);
                        y -= 1;
                        println!("generate: y decrement: ({}, {})", y, x);
                        Maze::set_wall(y, x, map, wall_coords);
                    }
                    Direction::Down => {
                        println!("generate: down: ({}, {})", y + 2, x);
                        is_wall = map[y + 2][x];
                        y += 1;
                        println!("generate: y increment: ({}, {})", y, x);
                        Maze::set_wall(y, x, map, wall_coords);
                        y += 1;
                        println!("generate: y increment: ({}, {})", y, x);
                        Maze::set_wall(y, x, map, wall_coords);
                    }
                    Direction::Right => {
                        println!("generate: right: ({}, {})", y, x - 2);
                        is_wall = map[y][x - 2];
                        x -= 1;
                        println!("generate: x decrement: ({}, {})", y, x);
                        Maze::set_wall(y, x, map, wall_coords);
                        x -= 1;
                        println!("generate: x decrement: ({}, {})", y, x);
                        Maze::set_wall(y, x, map, wall_coords);
                    }
                    Direction::Left => {
                        println!("generate: left: ({}, {})", y, x + 2);
                        is_wall = map[y][x + 2];
                        x += 1;
                        println!("generate: x increment: ({}, {})", y, x);
                        Maze::set_wall(y, x, map, wall_coords);
                        x += 1;
                        println!("generate: x increment: ({}, {})", y, x);
                        Maze::set_wall(y, x, map, wall_coords);
                    }
                }

                if is_wall {
                    println!("generate: reach existing wall");
                    break;
                }
            } else {
                if wall_coords.len() > 0 {
                    let prev = wall_coords.pop().unwrap();
                    y = prev.y;
                    x = prev.x;
                    println!("generate: back: ({}, {})", y, x);
                }
            }
        }
    }

    fn generate(&self) -> MazeMap {
        let height = self.height;
        let width = self.width;
        let mut map = MazeHelper::empty_map(height, width, true, false);
        let mut even_map = MazeHelper::empty_map(height, width, false, true);
        let mut wall_coords: Coords = vec![];

        // 無効化されていない偶数の座標がある限りループする.
        while MazeHelper::is_available(&even_map, true) {
            let (x, y) = (
                MazeHelper::even_random(width),
                MazeHelper::even_random(height),
            );

            // 指定された,x,y共に偶数である座標を無効化.
            even_map[y][x] = false;

            // 指定座標が道の場合.
            if map[y][x] == false {
                println!("generate: start: ({}, {})", y, x);
                wall_coords = vec![];
                Maze::extend_wall(y, x, &mut map, &mut wall_coords);
            }
        }

        map
    }

    fn serialize(&self, map: &MazeMap, wall: &str, road: &str, start: &str, goal: &str) -> String {
        let height = self.height;
        let width = self.width;

        let mut map_str = String::from("");

        for y in 0..height {
            for x in 0..width {
                if y == self.start.y && x == self.start.x {
                    map_str.push_str(start);
                } else if y == self.goal.y && x == self.goal.x {
                    map_str.push_str(goal);
                } else if map[y][x] {
                    map_str.push_str(wall);
                } else {
                    map_str.push_str(road);
                }
            }
            map_str.push_str("\n");
        }

        map_str
    }
}

struct MazeSolverDfs {
    height: usize,
    width: usize,
    start: Coord,
    goal: Coord,
}

impl MazeSolverDfs {
    fn solve(&self, map: &MazeMap) -> Vec<usize> {
        let height = self.height;
        let width = self.width;
        let start = Coord::new(height - 2, width - 2);
        let goal = Coord::new(1, 1);

        // 探索待ちスタック
        let mut search_coords: Coords = vec![];
        search_coords.push(start);

        // 移動履歴
        let mut moves_encoded: Vec<usize> = vec![0; height * width];

        // 探索待ちスタックがある かつ ゴールしていない限りループする.
        while search_coords.len() > 0 {
            let target = search_coords.pop().unwrap();

            for direction in vec![
                Direction::Up,
                Direction::Down,
                Direction::Right,
                Direction::Left,
            ] {
                let mut next_target = Coord::new(target.y, target.x);

                match direction {
                    Direction::Up => {
                        next_target.y -= 1;
                        println!("solve: y decrement: target({}, {})", target.y, target.x);
                    }
                    Direction::Down => {
                        next_target.y += 1;
                        println!("solve: y increment: target({}, {})", target.y, target.x);
                    }
                    Direction::Right => {
                        next_target.x -= 1;
                        println!("solve: x decrement: target({}, {})", target.y, target.x);
                    }
                    Direction::Left => {
                        next_target.x += 1;
                        println!("solve: x increment: target({}, {})", target.y, target.x);
                    }
                }

                // ここでのnext_target.* 1 > 0は>= 0とほぼ同義
                if next_target.y + 1 > 0
                    && next_target.x + 1 > 0
                    && next_target.y < height
                    && next_target.x < width
                {
                    // falseの場合道, かつ移動履歴にない未探索の場合
                    if !map[next_target.y][next_target.x]
                        && moves_encoded[MazeHelper::encode_coord(&next_target, self.width)] == 0
                    {
                        moves_encoded[MazeHelper::encode_coord(&next_target, self.width)] =
                            MazeHelper::encode_coord(&target, self.width);
                        println!(
                            "solve: moves: ({}, {}), ({}, {})",
                            next_target.y, next_target.x, target.y, target.x
                        );

                        search_coords.push(Coord::new(next_target.y, next_target.x));
                        println!("solve: explored: ({}, {})", next_target.y, next_target.x);

                        if goal.y == self.height && goal.x == self.width {
                            println!("solve: goal");
                            break;
                        }
                    }
                }
            }
        }

        moves_encoded
    }

    fn ans_route(&self, moves: &Vec<usize>) -> Coords {
        let mut ans_coords: Coords = vec![];
        let mut current_index = MazeHelper::encode_coord(&self.goal, self.width);

        while current_index > 0 {
            let current_coord = MazeHelper::decode_encoded_coord(current_index, self.width);

            if current_coord.y == self.start.y && current_coord.x == self.start.x {
                break;
            }

            ans_coords.push(Coord::new(current_coord.y, current_coord.x));
            println!("ans_route: ({}, {})", current_coord.y, current_coord.x);

            current_index = moves[current_index];
            println!("ans_route2: ({}, {})", current_coord.y, current_coord.x);
        }

        ans_coords
    }

    fn serialize(
        &self,
        map: &MazeMap,
        moves: &Coords,
        wall: &str,
        road: &str,
        start: &str,
        goal: &str,
        ans: &str,
    ) -> String {
        let height = map.len();
        let width = map[0].len();

        let mut move_map: MazeMap = MazeHelper::empty_map(height, width, false, false);
        let mut map_str = String::from("");

        for y in 0..height {
            for x in 0..width {
                for move_val in moves {
                    if y == move_val.y && x == move_val.x {
                        move_map[y][x] = true;
                    }
                }
            }
        }

        for y in 0..height {
            for x in 0..width {
                if y == 0 && x == 0 {
                    map_str.push_str("O ");
                } else if y == self.height - 1 && x == 0 {
                    map_str.push_str("Y ");
                } else if y == 0 && x == self.width - 1 {
                    map_str.push_str("X ");
                } else if y == self.start.y && x == self.start.x {
                    map_str.push_str(start);
                } else if y == self.goal.y && x == self.goal.x {
                    map_str.push_str(goal);
                } else if map[y][x] {
                    map_str.push_str(wall);
                } else if move_map[y][x] {
                    map_str.push_str(ans);
                } else if !map[y][x] {
                    map_str.push_str(road);
                }
            }
            map_str.push_str("\n");
        }

        map_str
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        let _height: usize = args[1].parse::<usize>().unwrap_or(5);
        let _width: usize = args[2].parse::<usize>().unwrap_or(5);

        let height: usize = MazeHelper::check_input(_height);
        let width: usize = MazeHelper::check_input(_width);

        let maze = Maze {
            height: height,
            width: width,
            start: Coord::new(height - 2, width - 2),
            goal: Coord::new(1, 1),
        };

        let map = maze.generate();

        let dfs = MazeSolverDfs {
            height: height,
            width: width,
            start: Coord::new(height - 2, width - 2),
            goal: Coord::new(1, 1),
        };

        let dfs_solve = dfs.ans_route(&dfs.solve(&map));

        println!(
            "{}",
            maze.serialize(&map, "\x1b[42m  \x1b[m", "\x1b[47m  \x1b[m", "\x1b[41mS \x1b[m", "\x1b[46mG \x1b[m")
        );

        println!(
            "{}",
            dfs.serialize(
                &map,
                &dfs_solve,
                "\x1b[42m  \x1b[m",
                "\x1b[47m  \x1b[m",
                "\x1b[41mS \x1b[m",
                "\x1b[44mG \x1b[m",
                "\x1b[43m  \x1b[m"
            )
        );
    } else {
        eprintln!("Usage: <this program> <height> <width>");
        process::exit(1);
    }
}

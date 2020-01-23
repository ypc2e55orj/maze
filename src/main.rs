extern crate rand;

use rand::Rng;

type WallCoords = Vec<WallCoord>;

struct WallCoord {
    y: usize,
    x: usize,
}

impl WallCoord {
    fn new(y: usize, x: usize) -> WallCoord {
        WallCoord { y: y, x: x }
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

    // 与えられたmapに有効な偶数座標(true)が存在するかを確認する.
    fn is_even(map: &MazeMap) -> bool {
        let mut even = false;

        for (y, y_val) in map.iter().enumerate() {
            for (x, _) in y_val.iter().enumerate() {
                if map[y][x] == true {
                    even = true
                }
            }
        }

        even
    }

    // Wall構造体が入ったベクターを受け取り,拡張中の自分自身かどうかを確認する.
    fn is_wall_myself(y: usize, x: usize, wall_coords: &WallCoords) -> bool {
        let mut is_wall_myself = false;

        for coord in wall_coords {
            if coord.y == y && coord.x == x {
                is_wall_myself = true;
            }
        }

        is_wall_myself
    }
}

struct Maze {
    height: usize,
    width: usize,
}

impl Maze {
    fn set_wall(y: usize, x: usize, map: &mut MazeMap, wall_coords: &mut WallCoords) {
        println!("set: ({}, {})", y, x);
        map[y][x] = true;

        if x % 2 == 0 && y % 2 == 0 {
            println!("add: ({}, {})", y, x);
            wall_coords.push(WallCoord::new(y, x));
        }
    }

    fn extend_wall(_y: usize, _x: usize, map: &mut MazeMap, wall_coords: &mut WallCoords) {
        let mut is_wall = false;

        loop {
            let mut x = _x;
            let mut y = _y;
            let mut directions = vec![];

            if !map[y - 1][x] && !MazeHelper::is_wall_myself(y - 2, x, wall_coords) {
                directions.push(Direction::Up)
            }
            if !map[y + 1][x] && !MazeHelper::is_wall_myself(y + 2, x, wall_coords) {
                directions.push(Direction::Down)
            }
            if !map[y][x - 1] && !MazeHelper::is_wall_myself(y, x - 2, wall_coords) {
                directions.push(Direction::Right)
            }
            if !map[y][x - 1] && !MazeHelper::is_wall_myself(y, x + 2, wall_coords) {
                directions.push(Direction::Left)
            }

            if directions.len() > 0 {
                Maze::set_wall(y, x, map, wall_coords);

                let random_index = MazeHelper::random(directions.len());
                match directions[random_index] {
                    Direction::Up => {
                        println!("up: ({}, {})", y - 2, x);
                        is_wall = map[y - 2][x];
                        y -= 1;
                        println!("y deinc: ({}, {})", y, x);
                        Maze::set_wall(y, x, map, wall_coords);
                        y -= 1;
                        println!("y deinc: ({}, {})", y, x);
                        Maze::set_wall(y, x, map, wall_coords);
                    }
                    Direction::Down => {
                        println!("down: ({}, {})", y + 2, x);
                        is_wall = map[y + 2][x];
                        y += 1;
                        println!("y inc: ({}, {})", y, x);
                        Maze::set_wall(y, x, map, wall_coords);
                        y += 1;
                        println!("y inc: ({}, {})", y, x);
                        Maze::set_wall(y, x, map, wall_coords);
                    }
                    Direction::Right => {
                        println!("direction: right, ({}, {})", y, x - 2);
                        is_wall = map[y][x - 2];
                        x -= 1;
                        println!("x deinc: ({}, {})", y, x);
                        Maze::set_wall(y, x, map, wall_coords);
                        x -= 1;
                        println!("x inc: ({}, {})", y, x);
                        Maze::set_wall(y, x, map, wall_coords);
                    }
                    Direction::Left => {
                        println!("direction: left, ({}, {})", y, x + 2);
                        is_wall = map[y][x + 2];
                        x += 1;
                        println!("x inc: ({}, {})", y, x);
                        Maze::set_wall(y, x, map, wall_coords);
                        x += 1;
                        println!("x inc: ({}, {})", y, x);
                        Maze::set_wall(y, x, map, wall_coords);
                    }
                }

                if is_wall {
                    println!("attach existed wall");
                    break;
                }
            } else {
                if wall_coords.len() > 0 {
                    let prev = wall_coords.pop().unwrap();
                    y = prev.y;
                    x = prev.x;
                    println!("back: ({}, {})", y, x);
                } else {
                    println!("can't back, break");
                    break;
                }
            }
        }
    }

    fn generate(&self) -> MazeMap {
        let height = MazeHelper::check_input(self.height);
        let width = MazeHelper::check_input(self.width);
        let mut map = MazeHelper::empty_map(height, width, true, false);
        let mut even_map = MazeHelper::empty_map(height, width, false, true);
        let mut wall_coords: WallCoords = vec![];

        // 無効化されていない偶数の座標がある限りループする.
        while MazeHelper::is_even(&even_map) {
            let (x, y) = (
                MazeHelper::even_random(height),
                MazeHelper::even_random(width),
            );

            // 指定された,x,y共に偶数である座標を無効化.
            even_map[y][x] = false;

            // 指定座標が道の場合.
            if map[y][x] == false {
                println!("start: ({}, {})", y, x);
                wall_coords = vec![];
                Maze::extend_wall(y, x, &mut map, &mut wall_coords);
            }
        }

        map
    }

    fn serialize(map: MazeMap, wall_str: &str, road_str: &str) -> String {
        let mut map_str = String::from("");

        for (y, y_val) in map.iter().enumerate() {
            for (x, _) in y_val.iter().enumerate() {
                if map[y][x] == true {
                    map_str.push_str(wall_str);
                } else {
                    map_str.push_str(road_str);
                }
            }
            map_str.push_str("\n");
        }

        map_str
    }
}

fn main() {
    let maze = Maze {
        height: 11,
        width: 11,
    };
    println!("{}", Maze::serialize(maze.generate(), "■ ", "  "));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rotation {
    direction: Direction,
    distance: i32,
}

fn parse_line(line: &str) -> Rotation {
    let (dir_char, dist_str) = line.split_at(1);
    let direction = match dir_char {
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("Invalid direction character"),
    };

    let distance: i32 = dist_str.parse().expect("Bruh this should be a i32.. ong");

    Rotation {
        direction,
        distance,
    }
}

fn parse(input: &str) -> Vec<Rotation> {
    input.lines().map(parse_line).collect()
}

#[derive(Debug, Clone, Copy)]
struct Dial {
    pos: i32, // 0 to 99
}

impl Dial {
    fn new(start: i32) -> Self {
        Self {
            pos: start.rem_euclid(100),
        }
    }

    fn rotate(&mut self, rotation: Rotation) -> bool {
        match rotation.direction {
            Direction::Left => {
                self.pos = (self.pos - rotation.distance).rem_euclid(100);
            }
            Direction::Right => {
                self.pos = (self.pos + rotation.distance).rem_euclid(100);
            }
        }

        self.pos == 0
    }

    fn rotate_count_clicks(&mut self, rotation: Rotation) -> i32 {
        let mut hits_zero = 0;
        for _ in 0..rotation.distance {
            match rotation.direction {
                Direction::Left => {
                    self.pos = (self.pos - 1).rem_euclid(100);
                }
                Direction::Right => {
                    self.pos = (self.pos + 1).rem_euclid(100);
                }
            }
            if self.pos == 0 {
                hits_zero += 1;
            }
        }
        hits_zero
    }
}

pub fn part_1(input: &str) -> i32 {
    let rotations = parse(input);
    let mut dial = Dial::new(50);
    let mut hits_zero = 0;

    for rotation in rotations {
        if dial.rotate(rotation) {
            hits_zero += 1;
        }
    }

    hits_zero
}

pub fn part_2(input: &str) -> i32 {
    let rotations = parse(input);
    let mut dial = Dial::new(50);
    let mut hits_zero = 0;

    for rotation in rotations {
        hits_zero += dial.rotate_count_clicks(rotation);
    }

    hits_zero
}

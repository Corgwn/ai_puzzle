use core::fmt;
use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};

//Farm code

pub struct Farm {
    field: Vec<Vec<char>>,
    max_cows: i32,
    num_cows: i32,
    pub space_left: i32,
    pub size: usize,
}

impl Farm {
    pub fn Copy(&self) -> Farm {
        let new = Farm {
            field: self.field.clone(),
            max_cows: self.max_cows,
            num_cows: self.num_cows,
            space_left: self.space_left,
            size: self.size,
        };
        new
    }

    pub fn get_field(&self) -> Vec<Vec<char>> {
        self.field.clone()
    }

    pub fn add_cow(&mut self, loc: Vec<usize>) -> bool {
        if self.space_left == 0 {
            return false;
        }
        match self.field[loc[0 as usize]][loc[1 as usize]] {
            'C' | '@' | '#' => return false,
            '.' => {}
            _ => panic!("Invalid Symbol"),
        }
        self.field[loc[0 as usize]][loc[1 as usize]] = 'C';
        self.space_left -= 1;
        return true;
    }

    pub fn remove_cow(&mut self, loc: Vec<usize>) -> bool {
        if self.space_left == self.max_cows {
            return false;
        }
        match self.field[loc[0 as usize]][loc[1 as usize]] {
            '.' | '@' | '#' => return false,
            'C' => {}
            _ => panic!("Invalid Symbol"),
        }
        self.field[loc[0 as usize]][loc[1 as usize]] = '.';
        self.space_left += 1;
        return true;
    }
}

impl fmt::Display for Farm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let mut temp1: String = String::new();
        let mut temp2 = String::new();
        for i in self.field.as_slice() {
            for j in i {
                temp1.push(*j);
            }
            temp1.push('\n');
            temp2 += temp1.as_str();
            temp1.clear();
        }
        write!(f, "{}", temp2)
    }
}

pub fn read_file(path: String) -> Farm {
    let mut field: Vec<Vec<char>> = Vec::new();
    let max_cows: i32;
    let mut num_cows: i32;
    let mut space_left: i32;
    let size: usize;

    //Read file in
    let file = BufReader::new(File::open(&path).unwrap());
    let mut lines: Vec<_> = file.lines().collect();

    //Set the size
    size = lines[0].as_ref().unwrap().parse::<i32>().unwrap() as usize;
    lines.remove(0);

    //Build the field from the given input file
    for line in lines {
        field.push(line.as_ref().unwrap().chars().collect::<Vec<char>>());
    }

    //Find max cows
    let mut temp: i32 = 0;
    for line in field.as_slice() {
        for tile in line {
            if *tile == '@' {
                temp = temp + 1;
            }
        }
    }
    max_cows = temp;

    let farm = Farm {
        field: field,
        max_cows: max_cows,
        num_cows: 0,
        space_left: max_cows,
        size: size,
    };
    farm
}

fn in_bounds(f: &Farm, r: i32, c: i32) -> bool {
    if r < 0 || r >= f.size as i32 || c < 0 || c >= f.size as i32 {
        return false;
    }
    return true;
}

fn score_cow(f: &Farm, r: usize, c: usize) -> i32 {
    let mut cow = false;
    let mut hay = false;
    let mut water = false;
    let mut offsets: Vec<Vec<i32>> = Vec::new();
    for i in -1..2 {
        for j in -1..2 {
            let mut temp = Vec::new();
            if i == 0 && j == 0 {
                continue;
            }
            temp.push(i);
            temp.push(j);
            offsets.push(temp);
        }
    }
    let field = f.get_field();
    let mut score = 0;
    for offset in offsets {
        let testx = r as i32 + offset[0];
        let testy = c as i32 + offset[1];
        if in_bounds(f, testx, testy) {
            if field[testx as usize][testy as usize] == 'C' {
                cow = true;
            }
            if offset[0] == 0 || offset[1] == 0 {
                if field[testx as usize][testy as usize] == '@' {
                    hay = true;
                } else if field[testx as usize][testy as usize] == '#' {
                    water = true;
                }
            }
        }
    }
    if cow {
        score -= 3;
    }
    if hay {
        if water {
            score += 3;
        } else {
            score += 1;
        }
    }
    score
}

pub fn score_farm(f: &Farm) -> i32 {
    let mut sum = 0;
    let field = f.get_field();
    for i in 0..f.size {
        for j in 0..f.size {
            match field[i][j] {
                //If grass, water, or hay move on
                '.' | '@' | '#' => continue,
                //If cow, get score for this cow
                'C' => sum += score_cow(f, i, j),
                //If not standard symbol, panic
                _ => panic!(),
            }
        }
    }
    sum
}

//AI code

pub struct Intel {}

impl Intel {
    pub fn random_move(board: &Farm) -> Vec<usize> {
        let mut rng = rand::thread_rng();
        let mut res: Vec<usize> = Vec::new();
        res.push(rng.gen_range(0..board.size));
        res.push(rng.gen_range(0..board.size));
        res
    }

    pub fn BFS(board: &Farm) -> Vec<usize> {
        let best_move: Vec<usize> = Vec::new();
        let best_score: i32;

        return best_move;
    }
}

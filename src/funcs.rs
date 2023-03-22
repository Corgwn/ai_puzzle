use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::Rng;

//Farm code

pub struct Farm {
    field: Vec<Vec<char>>,
    max_cows: i32,
    num_cows: i32,
    pub space_left: i32,
    pub size: usize,
}

impl Farm {
    pub fn Copy(&self) -> Farm{
        let new = Farm {field: self.field.clone(), max_cows: self.max_cows, num_cows: self.num_cows, space_left: self.space_left, size: self.size};
        new
    }

    pub fn get_field(&self) -> Vec<Vec<char>> {
        self.field.clone()
    }

    pub fn add_cow(&mut self, loc: Vec<usize>) -> bool {
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

pub fn score_farm(f: Farm) -> i32 {
    let mut sum = 0;
    let field = f.get_field();
    for i in 0..f.size {
        for j in 0..f.size {
            match field[i][j] {
                //If grass, water, or hay move on
                '.' | '@' | '#' => continue,
                //If cow, get score for this cow
                'C' => {
                    let right: bool;
                    let left: bool;
                    let top: bool;
                    let bottom: bool;
                    let max: usize = f.size - 1;
                    let mut cow: bool = false;
                    let mut hay: bool = false;
                    let mut water: bool = false;

                    match i {
                        0 => {
                            top = false;
                            bottom = true;
                        }
                        max => {
                            top = true;
                            bottom = false;
                        }
                        _ => {
                            top = true;
                            bottom = true;
                        }
                    }
                    match j {
                        0 => {
                            left = false;
                            right = true;
                        }
                        max => {
                            left = true;
                            right = false;
                        }
                        _ => {
                            left = true;
                            right = true;
                        }
                    }

                    if left && top {
                        if field[i - 1][j - 1] == 'C' {
                            cow = true;
                        }
                    }
                    if top {
                        match field[i - 1][j] {
                            'C' => cow = true,
                            '@' => hay = true,
                            '#' => water = true,
                            _ => panic!(),
                        }
                    }
                    if right && top {
                        if field[i - 1][j + 1] == 'C' {
                            cow = true;
                        }
                    }
                    if left {
                        match field[i][j - 1] {
                            'C' => cow = true,
                            '@' => hay = true,
                            '#' => water = true,
                            _ => panic!(),
                        }
                    }
                    if right {
                        match field[i][j + 1] {
                            'C' => cow = true,
                            '@' => hay = true,
                            '#' => water = true,
                            _ => panic!(),
                        }
                    }
                    if left && bottom {
                        if field[i + 1][j - 1] == 'C' {
                            cow = true;
                        }
                    }
                    if bottom {
                        match field[i + 1][j] {
                            'C' => cow = true,
                            '@' => hay = true,
                            '#' => water = true,
                            _ => panic!(),
                        }
                    }
                    if right && bottom {
                        if field[i + 1][j + 1] == 'C' {
                            cow = true;
                        }
                    }

                    if cow {
                        sum -= 3;
                    }
                    if hay {
                        if water {
                            sum += 3;
                        } else {
                            sum += 1;
                        }
                    }
                }
                //If not standard symbol, panic
                _ => panic!(),
            }
        }
    }

    sum
}

//AI code

pub struct Intel {

}

impl Intel {
    pub fn random_move(board: &Farm) -> Vec<usize> {
        let mut rng = rand::thread_rng();
        let mut res: Vec<usize> = Vec::new();
        res.push(rng.gen_range(0..board.size));
        res.push(rng.gen_range(0..board.size));
        res
    }
}

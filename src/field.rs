use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

pub struct Farm {
    field: Vec<Vec<char>>,
    max_cows: i32,
    num_cows: i32,
    pub space_left: i32,
    pub size: usize,
}

impl Farm {
    pub fn get_field(&self) -> Vec<Vec<char>> {
        self.field.clone()
    }
    
    pub fn add_cow(&mut self, loc: Vec<i32>) -> bool {
        false
    }
    pub fn remove_cow(&mut self, loc: Vec<i32>) -> bool {
        false
    }
}

pub fn read_file(path: String) -> Farm {
    let mut field: Vec<Vec<char>> = Vec::new();
    let mut score: i32;
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

    let farm = Farm {field: field, max_cows: max_cows, num_cows: 0, space_left: max_cows, size: size};
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
                    let mut set = HashSet::new();
                    
                    //TODO: This needs logic for handling the edges of the field
                    set.insert(field[i][(j + 1)]);
                    set.insert(field[i][(j - 1)]);
                    set.insert(field[(i + 1)][j]);
                    set.insert(field[(i - 1)][j]);
                    if set.contains(&'@') {
                        if set.contains(&'#') {
                            sum += 3;
                        }
                        else {
                            sum += 1;
                        }
                    }
                    if set.contains(&'C') {
                        sum -= 3;
                    }
                }
                //If not standard symbol, panic
                _ => panic!(),
            }
        }
    }

    sum
}
use core::fmt;
use std::cmp::Ordering;
use rand::Rng;
use std::collections::{HashSet, VecDeque, BinaryHeap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

//Farm code
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Farm {
    field: Vec<Vec<char>>,
    pub max_cows: usize,
    pub space_left: usize,
    pub size: usize,
}

impl Farm {
    pub fn get_field(&self) -> Vec<Vec<char>> {
        self.field.clone()
    }

    pub fn add_cow(&mut self, loc: [usize; 2]) -> bool {
        if self.space_left == 0 {
            return false;
        }
        match self.field[loc[0_usize]][loc[1_usize]] {
            'C' | '@' | '#' => return false,
            '.' => {}
            _ => panic!("Invalid Symbol"),
        }
        self.field[loc[0_usize]][loc[1_usize]] = 'C';
        self.space_left -= 1;
        true
    }

    pub fn remove_cow(&mut self, loc: [usize; 2]) -> bool {
        if self.space_left == self.max_cows {
            return false;
        }
        match self.field[loc[0_usize]][loc[1_usize]] {
            '.' | '@' | '#' => return false,
            'C' => {}
            _ => panic!("Invalid Symbol"),
        }
        self.field[loc[0_usize]][loc[1_usize]] = '.';
        self.space_left += 1;
        true
    }

    pub fn add_many_cow(&mut self, locs: &HashSet<[usize; 2]>) {
        //Will assume all moves given to it are valid
        for loc in locs {
            self.add_cow(*loc);
        }
    }

    pub fn _remove_many_cow(&mut self, locs: &HashSet<[usize; 2]>) {
        //Will remove cows from each location given
        for loc in locs {
            self.remove_cow(*loc);
        }
    }
}

//AI code

pub struct Intel {}

#[allow(dead_code)]
impl Intel {
    pub fn random_move(board: &Farm) -> [usize; 2] {
        let mut rng = rand::thread_rng();
        [rng.gen_range(0..board.size), rng.gen_range(0..board.size)]
    }

    pub fn bfs(board: &Farm) -> HashSet<[usize; 2]> {
        let now = Instant::now();
        let mut result: HashSet<[usize; 2]> = HashSet::new();
        let mut frontier: VecDeque<HashSet<[usize; 2]>> = VecDeque::from([HashSet::new()]);
        
        while let Some(temp_path) = frontier.pop_front() {
            let mut test_board = board.clone();
            test_board.add_many_cow(&temp_path);

            //Test if the popped state fits the goal
            if goal(&test_board, 7) {
                result = temp_path;
                break;
            }

            //Add all neighbors to frontier
            //Finding furthest move from origin
            let mut high_pos = [0, 0];
            for pos in temp_path.iter() {
                if pos[0] >= high_pos[0] && pos[1] >= high_pos[1] {
                    high_pos = *pos;
                }
            }
            let first_move: bool;
            if temp_path.len() == 0 {
                first_move = true;
            }
            else {
                first_move = false;
            }
            
            let field = test_board.get_field();
            //Adding all moves after this move
            for i in high_pos[0]..board.size {
                for j in 0..board.size {
                    if (!first_move && i == high_pos[0] && j <= high_pos[1]) || (field[i][j] != '.') {
                        continue;
                    }
                    let mut new_move: HashSet<[usize; 2]> = temp_path.clone();
                    new_move.insert([i, j]);
                    frontier.push_back(new_move);
                }
            }
        }

        println!("Elapsed Time: {} ns", now.elapsed().as_nanos());
        result
    }

    fn bdfs(board: &Farm, depth: usize) -> Option<HashSet<[usize; 2]>> {
            let mut frontier: Vec<HashSet<[usize; 2]>> = vec!(HashSet::new());
            
            while let Some(temp_path) = frontier.pop() {
                let mut test_board = board.clone();
                test_board.add_many_cow(&temp_path);

                //Test if the popped state fits the goal
                if temp_path.len() == depth && goal(&test_board, 7) {
                    return Some(temp_path);
                }

                //If the path is already max length, don't add more child paths
                if temp_path.len() == depth {
                    continue;
                }

                //Add all neighbors to frontier
                //Finding furthest move from origin
                let mut high_pos = [0, 0];
                for pos in temp_path.iter() {
                    if pos[0] >= high_pos[0] && pos[1] >= high_pos[1] {
                        high_pos = *pos;
                    }
                }

                let first_move: bool;
                if temp_path.len() == 0 {
                    first_move = true;
                }
                else {
                    first_move = false;
                }
                

                let field = test_board.get_field();
                //Adding all moves after this move
                for i in high_pos[0]..board.size {
                    for j in 0..board.size {
                        if (!first_move && i == high_pos[0] && j <= high_pos[1]) || (field[i][j] != '.') {
                            continue;
                        }
                        let mut new_move: HashSet<[usize; 2]> = temp_path.clone();
                        new_move.insert([i, j]);

                        frontier.push(new_move);
                    }
                }
            }
        return None;
    }

    pub fn id_dfs(board: &Farm) -> HashSet<[usize; 2]> {
        let now = Instant::now();
        let mut result = HashSet::new();
        for depth in 1..=board.max_cows {
            if let Some(temp) = Intel::bdfs(board, depth) {
                result = temp;
                break;
            }
        }
        println!("Elapsed Time: {} ns", now.elapsed().as_nanos());
        result
    }

    pub fn best_fs(board: Farm) -> HashSet<[usize; 2]> {
        let now = Instant::now();
        let mut result: HashSet<[usize; 2]> = HashSet::new();
        let mut frontier: BinaryHeap<Move> = BinaryHeap::new();
        let mut checked_boards = vec![];
        frontier.push(Move {moves: HashSet::new(), score: 0});
        
        while let Some(test_path) = frontier.pop() {
            let mut test_board = board.clone();
            test_board.add_many_cow(&test_path.moves);
            println!("{:?}", test_path);
            let test_field = test_board.get_field();
            if checked_boards.contains(&test_field) {
                continue;
            }

            //Test if the popped state fits the goal
            if test_board.space_left == 0 && goal(&test_board, 12) {
                result = test_path.moves;
                break;
            }

            checked_boards.push(test_field);

            //Add all neighbors to frontier
            let size = test_board.size;
            for i in 0..size {
                for j in 0..size {
                    if test_board.get_field()[i][j] == '.' && test_path.moves.len() < test_board.max_cows {
                        let mut new_move: HashSet<[usize; 2]> = test_path.moves.clone();
                        new_move.insert([i, j]);
                        //println!("Adding move {:?} to the frontier", new_move);
                        let mut temp_board = test_board.clone();
                        temp_board.add_cow([i, j]);
                        frontier.push(Move {moves: new_move, score: score_farm(&temp_board)});
                    }
                }
            }
        }
        println!("Elapsed Time: {} ns", now.elapsed().as_nanos());
        result
    }
}

//Supporting Functions
impl fmt::Display for Farm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut temp1 = String::new();
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
    let mut _num_cows: usize;
    let mut _space_left: usize;

    //Read file in
    let file = BufReader::new(File::open(&path).unwrap());
    let mut lines = file.lines();

    //Set the size
    let size: usize = lines.next().expect("Must contain at least one line").expect("Line must contain a value").trim().parse().unwrap();

    //Build the field from the given input file
    while let Some(line) = lines.next() {
        field.push(line.unwrap().trim().chars().collect::<Vec<char>>());
    }

    //Find max cows
    let mut temp: usize = 0;
    for line in field.iter() {
        for tile in line {
            if *tile == '@' {
                temp += 1;
            }
        }
    }
    let max_cows = temp;

    Farm {
        field,
        max_cows,
        space_left: max_cows,
        size,
    }
}

fn in_bounds(f: &Farm, r: i32, c: i32) -> bool {
    if r < 0 || r >= f.size as i32 || c < 0 || c >= f.size as i32 {
        return false;
    }
    true
}

fn score_cow(f: &Farm, r: usize, c: usize) -> i32 {
    let mut cow = false;
    let mut hay = false;
    let mut water = false;
    let offsets = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];
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
    for (i, row) in field.iter().enumerate() {
        for (j, elem) in row.iter().enumerate() {
            match elem {
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

fn goal(board: &Farm, value: i32) -> bool {
    if score_farm(board) >= value {
        return true;
    }
    false
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
            .then_with(|| self.moves.len().cmp(&other.moves.len()))
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Move {
    pub moves: HashSet<[usize; 2]>,
    pub score: i32,
}
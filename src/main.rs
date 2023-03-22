use std::env;

mod funcs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut farm = funcs::read_file(args[1].clone());
    while farm.space_left > 0 {
        farm.add_cow(funcs::Intel::random_move(&farm));
    }
    
}

use std::env;
use std::fs::File;
use std::io::Write;

mod funcs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut farm = funcs::read_file(args[1].clone());
    while farm.space_left > 0 {
        farm.add_cow(funcs::Intel::random_move(&farm));
    }
    let mut f = File::create(args[2].clone()).unwrap();
    write!(f, "{}\n", farm.size);
    write!(f, "{}", farm);
    write!(f, "{}\n", funcs::score_farm(&farm));
}

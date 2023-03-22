use std::env;
use std::fs::File;
use std::io::Write;

mod funcs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut farm = funcs::read_file(args[1].clone());

    //Code for random moves
    //while farm.space_left > 0 {
    //    farm.add_cow(funcs::Intel::random_move(&farm));
    //}

    //Code for BFS
    let moves = funcs::Intel::bfs(farm.clone());
    farm.add_many_cow(&moves);

    let mut f = File::create(args[2].clone()).unwrap();
    writeln!(f, "{}", farm.size).unwrap();
    write!(f, "{}", farm).unwrap();
    writeln!(f, "{}", funcs::score_farm(&farm)).unwrap();
}

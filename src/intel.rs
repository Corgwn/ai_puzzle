use rand::Rng;
#[path = "./field.rs"] mod field;

pub struct Intel {

}

impl Intel {
    pub fn random_move(board: field::Farm) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let mut res: Vec<i32> = Vec::new();
        res.push(rng.gen_range(0..board.size as i32));
        res.push(rng.gen_range(0..board.size as i32));
        res
    }
}
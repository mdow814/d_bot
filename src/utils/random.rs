use std::convert::TryFrom;
use rand::Rng;

pub fn get_random_num(limit: u32) -> usize {
    let mut rng = rand::thread_rng();
    let num: u32 = rng.gen_range(0..limit);
    let index = usize::try_from(num).unwrap();
    index
}
use std::fs;
use serde::{Serialize, Deserialize};
use rand::Rng;
use std::convert::TryFrom;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Build {
    item1: String,
    item2: String,
    item3: String,
    item4: String,
    item5: String,
    item6: String,
}

impl Build {

    pub fn get_items(self) -> String {
        let message = format!(
            "Here's your build!\n\n{}\n{}\n{}\n{}\n{}\n{}\n",
            self.item1,
            self.item2,
            self.item3,
            self.item4,
            self.item5,
            self.item6,
        );
        message
    }
}

fn get_num() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..150)
}

pub fn get_build() -> Box<Build> {
    let num = get_num();
    let build_file_string = fs::read_to_string("builds.yaml").unwrap();
    let mut b: Vec<Build> = serde_yaml::from_str(&build_file_string).unwrap();
    let index = usize::try_from(num).unwrap();
    let build = b.remove(index);
    Box::from(build)
}


use std::fs;
use serde::{Serialize, Deserialize};
use rand::Rng;
use std::convert::TryFrom;
use std::env;

type Items = Vec<String>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Build {
    items: Items
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Builds {
    builds: Vec<Build>
}

impl Build {
    pub fn get_items(&mut self) -> String {
        let mut message = String::new();
        let items = self.items
            .iter_mut()
            .map(|x| add_newline(x))
            .collect::<Vec<&String>>();
        message.push_str("Here's your build!\n");
        for item in items {
            message.push_str(item);
        }
        message
    }
}

fn add_newline(text: &mut String) -> &String {
    text.push_str("\n");
    text
}

fn get_num(limit: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..limit)
}

pub fn get_build() -> Box<Build> {
    let path = env::var("BUILDS").expect("token");
    let build_file_string = fs::read_to_string(path).unwrap();
    let mut build_list: Builds = serde_yaml::from_str(&build_file_string).unwrap();
    let num = get_num(build_list.builds.len() as u32);
    let index = usize::try_from(num).unwrap();
    let build = build_list.builds.remove(index);
    Box::from(build)
}


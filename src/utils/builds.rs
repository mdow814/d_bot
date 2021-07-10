use std::fs;
use std::env;
use serde::{Serialize, Deserialize};
use crate::utils::random::get_random_num;

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
        let items = self.items.iter_mut().map(|item| add_newline(item)).collect::<Vec<&String>>();
        message.push_str("Here's your build!\n\n");
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

fn read_build_file_to_string() -> String {
    let path = env::var("BUILDS").expect("builds");
    let build_file_string = match fs::read_to_string(path) {
        Ok(build_file_string) => build_file_string,
        Err(err)              => panic!("Could not read yaml file {}", err)
    };
    build_file_string
}

fn parse_build_yaml(build_file_string: &str) -> Builds {
    let build_list = match serde_yaml::from_str(&build_file_string) {
        Ok(build_list) => build_list,
        Err(err)       => panic!("Could not deserialize yaml file {}", err)
    };
    build_list
}

pub fn get_build() -> Box<Build> {
    let build_file_string = read_build_file_to_string();
    let mut build_list = parse_build_yaml(&build_file_string);
    let index = get_random_num(build_list.builds.len() as u32 - 1);
    let build = build_list.builds.remove(index);
    Box::from(build)
}


use crate::utils::random::get_random_num;

pub fn get_game() -> String {
    let games = vec!["LoL", "SOT", "Rocket League", "Battlefield", "Valheim"];
    let num = get_random_num(games.len() as u32 - 1);
    games.get(num).unwrap().to_string()
}
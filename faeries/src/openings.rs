use games::gamerules::GameRules;

pub fn generate(pos: &impl GameRules, depth: i32) {
    if depth == 0 {
        println!("{}", pos.get_fen());
    } else {
        pos.move_generator(|mv| {
            let npos = pos.after_move(&mv);
            generate(&npos, depth - 1);
        });
    }
}

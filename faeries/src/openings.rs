use games::gamerules::GameRules;

pub fn generate(pos: &mut impl GameRules, depth: i32) {
    if depth == 0 {
        println!("{}", pos.get_fen());
    } else {
        for mv in pos.legal_moves() {
            pos.makemove(&mv);
            generate(pos, depth - 1);
            pos.undomove(&mv);
        }
    }
}

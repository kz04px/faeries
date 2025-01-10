use games::gamerules::GameRules;

#[must_use]
pub fn random<G: GameRules>(
    pos: &G,
    mut random_generator: impl FnMut() -> u64,
) -> Option<G::MoveType> {
    let moves = pos.legal_moves();
    let idx = random_generator() as usize % moves.len();
    Some(moves[idx])
}

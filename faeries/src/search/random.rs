use games::gamerules::GameRules;
use rand::seq::SliceRandom;

#[must_use]
fn random_impl<G: GameRules>(pos: &G) -> Option<G::MoveType> {
    pos.legal_moves().choose(&mut rand::thread_rng()).copied()
}

#[must_use]
pub fn random<G: GameRules>(pos: &G) -> Option<G::MoveType> {
    random_impl(pos)
}

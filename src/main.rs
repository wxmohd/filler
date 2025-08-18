mod game;
mod ai;

use ai::FillterAI;

fn main() {
    let mut ai = FillterAI::new();
    ai.run();
}
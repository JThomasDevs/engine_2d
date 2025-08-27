mod engine;
mod ecs;
mod render;
mod input;
mod utils;

use engine::Engine;

fn main() {
    let mut engine = Engine::new();
    engine.run();
}
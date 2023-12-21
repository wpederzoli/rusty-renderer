use ::renderer::prelude::*;

fn main() {
    env_logger::init();
    let renderer = renderer::Renderer::new("Rusty Renderer");
    renderer.run();
}

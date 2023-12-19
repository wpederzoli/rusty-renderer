mod renderer;

fn main() {
    env_logger::init();
    let renderer = renderer::Renderer::new("Rusty Renderer");
    renderer.run();
}

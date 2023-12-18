mod renderer;

fn main() {
    let renderer = renderer::RendererWindow::new().unwrap();
    renderer.run();
}

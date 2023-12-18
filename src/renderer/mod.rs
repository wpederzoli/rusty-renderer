use winit::{
    error::EventLoopError,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct RendererWindow {
    event_loop: EventLoop<()>,
    window: Window,
}

impl RendererWindow {
    pub fn new() -> Result<Self, EventLoopError> {
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        event_loop.set_control_flow(ControlFlow::Poll);

        Ok(RendererWindow { event_loop, window })
    }

    pub fn run(self) {
        self.event_loop
            .run(move |event, elwt| match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("Close");
                    elwt.exit();
                }
                Event::AboutToWait => {
                    self.window.request_redraw();
                }
                _ => (),
            })
            .unwrap()
    }
}

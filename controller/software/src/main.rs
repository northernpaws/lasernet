use color_eyre::eyre::Result;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId}
};

use vello::{
    kurbo::{Affine, Rect, Vec2},
    peniko::{Color, Fill},
    util::{RenderContext, RenderSurface},
    Renderer, RendererOptions, Scene, SceneBuilder,
};

struct App<'a> {
    window: Option<Window>,
    render_ctx: RenderContext,
    surface: Option<RenderSurface<'a>>
}

impl App<'_> {
    fn new(render_ctx: RenderContext) -> Self {
        Self {
            render_ctx,
            window: None,
            surface: None,
        }
    }
}

impl<'a> ApplicationHandler for App<'a> {
    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Wait);
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
        }

        if let Some(window) = &self.window {
            let size = window.inner_size();
            let surface_future = self.render_ctx.create_surface(window, size.width, size.height, wgpu::PresentMode::Immediate);
            self.surface = Some(pollster::block_on(surface_future).expect("Error creating surface"));
        } else {
            println!("Failed to create window!"); 
        }

        event_loop.set_control_flow(ControlFlow::Poll);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::Resized(new_size) => {
                if let Some(surface) = &mut self.surface {
                    self.render_ctx.resize_surface(surface, new_size.width, new_size.height);
                }
                
                //render_state.window.request_redraw();
            },
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    println!("Hello, world!");

    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    // event_loop.set_control_flow(ControlFlow::Wait);

    // Create the Vello rendering context.
    let render_ctx = RenderContext::new();

    // Construct a new app instance using the rendering context.
    let mut app = App::new(render_ctx);

    // Run the event loop for the app.
    event_loop.run_app(&mut app)?;

    Ok(())
}

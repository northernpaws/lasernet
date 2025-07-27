use std::sync::Arc;

use color_eyre::eyre::Result;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
    dpi::LogicalSize
};

use vello::{
    kurbo::{Affine, Rect, Vec2},
    peniko::{Color, Fill},
    util::{RenderContext, RenderSurface},
    Renderer, RendererOptions, Scene
};

enum RenderState<'s> {
    /// `RenderSurface` and `Window` for active rendering.
    Active {
        // The `RenderSurface` and the `Window` must be in this order, so that the surface is dropped first.
        surface: Box<RenderSurface<'s>>,
        window: Arc<Window>,
    },
    /// Cache a window so that it can be reused when the app is resumed after being suspended.
    Suspended(Option<Arc<Window>>),
}

struct App<'a> {
    context: RenderContext,
    
    state: RenderState<'a>
}

impl App<'_> {
    fn new(context: RenderContext) -> Self {
        Self {
            context,
            state: RenderState::Suspended(None),
        }
    }
}

/// Helper function that creates a Winit window and returns it (wrapped in an Arc for sharing between threads)
fn create_winit_window(event_loop: &ActiveEventLoop) -> Arc<Window> {
    let attr = Window::default_attributes()
        .with_inner_size(LogicalSize::new(1044, 800))
        .with_resizable(true)
        .with_title("Lasernet");
    Arc::new(event_loop.create_window(attr).unwrap())
}

impl ApplicationHandler for App<'_> {
    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        if let RenderState::Active { window, .. } = &self.state {
            self.state = RenderState::Suspended(Some(window.clone()));
        }

        event_loop.set_control_flow(ControlFlow::Wait);
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let RenderState::Suspended(cached_window) = &mut self.state else {
            return;
        };

        // Either acquired the previously suspended window, or take a new handle.
        let window = cached_window
            .take()
            .unwrap_or_else(|| create_winit_window(event_loop));

        let size = window.inner_size();
        let surface_future = self.context.create_surface(
            window.clone(),
            size.width,
            size.height,
            wgpu::PresentMode::AutoVsync,
        );
        let surface = pollster::block_on(surface_future).expect("Error creating surface");

        // Save the Window and Surface to a state variable
        self.state = RenderState::Active {
            surface: Box::new(surface),
            window,
        };

        event_loop.set_control_flow(ControlFlow::Poll);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
         // Only process events for our window, and only when we have a surface.
        let surface = match &mut self.state {
            RenderState::Active { surface, window } if window.id() == window_id => surface,
            _ => return,
        };

        match event {
            WindowEvent::Resized(new_size) => {
                self.context.resize_surface(surface, new_size.width, new_size.height);
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
                // self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
pub (crate) fn run_scoreboard() -> Result<()> {
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

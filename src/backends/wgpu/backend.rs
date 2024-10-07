use crate::{
    backends::{wgpu::state::WGPUState, GameBackend},
    core::screen::Screen,
    input::InputAction,
};
use winit::{
    dpi::LogicalSize,
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

pub struct WGPUBackend {
    screen: Screen,
    title: String,
}

impl GameBackend for WGPUBackend {
    fn new(screen: Screen, title: String) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
            } else {
                env_logger::init();
            }
        }
        Self { screen, title }
    }
    async fn run<F>(&mut self, mut game: F)
    where
        F: FnMut(&mut dyn FnMut(usize, usize, u32), &[InputAction], f64),
    {
        let size = LogicalSize::new(self.screen.width as u32, self.screen.height as u32);
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
            .with_inner_size(size)
            .with_title(self.title.clone())
            .build(&event_loop)
            .unwrap();

        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            let _ = window.request_inner_size(size);

            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("_wgpu")?;

                    // for fps counting in the webpage
                    let fps_counter = doc
                        .create_element("div")
                        .expect("Couldn't create FPS counter div");
                    fps_counter.set_id("_wgpu-fps");
                    fps_counter.set_inner_html("FPS: 0.0");
                    dst.append_child(&fps_counter)
                        .expect("Couldn't append FPS counter to document body.");

                    let canvas = web_sys::Element::from(window.canvas()?);
                    canvas.set_id("_wgpu-canvas");

                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

        let mut state = WGPUState::new(&window, &size, &self.screen).await;
        let mut surface_configured = false;
        let mut actions = Vec::new();

        event_loop
            .run(move |event, control_flow| {
                #[cfg(target_arch = "wasm32")]
                {
                    web_sys::window()
                        .and_then(|win| win.document())
                        .and_then(|doc| {
                            let fps_counter_element = doc
                                .get_element_by_id("_wgpu-fps")
                                .expect("Couldn't find FPS counter element");
                            fps_counter_element
                                .set_inner_html(&format!("FPS: {:.2}", state.fps_counter.fps()));
                            Some(())
                        })
                        .expect("Couldn't update FPS counter in document body.");
                }

                match event {
                    Event::WindowEvent {
                        ref event,
                        window_id,
                    } if window_id == state.window().id() => {
                        match event {
                            WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                event:
                                    KeyEvent {
                                        state: ElementState::Pressed,
                                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => control_flow.exit(),
                            WindowEvent::KeyboardInput {
                                event:
                                    KeyEvent {
                                        state: key_state,
                                        physical_key: keycode,
                                        ..
                                    },
                                ..
                            } => {
                                let action = match keycode {
                                    PhysicalKey::Code(KeyCode::KeyT) => Some(InputAction::T),
                                    PhysicalKey::Code(KeyCode::KeyC) => Some(InputAction::C),
                                    PhysicalKey::Code(KeyCode::KeyS) => Some(InputAction::S),
                                    PhysicalKey::Code(KeyCode::ShiftLeft) => {
                                        Some(InputAction::Sprint)
                                    }
                                    PhysicalKey::Code(KeyCode::ArrowUp) => {
                                        Some(InputAction::MoveForward)
                                    }
                                    PhysicalKey::Code(KeyCode::ArrowDown) => {
                                        Some(InputAction::MoveBackward)
                                    }
                                    PhysicalKey::Code(KeyCode::ArrowLeft) => {
                                        Some(InputAction::TurnLeft)
                                    }
                                    PhysicalKey::Code(KeyCode::ArrowRight) => {
                                        Some(InputAction::TurnRight)
                                    }
                                    _ => None,
                                };

                                if let Some(action) = action {
                                    match key_state {
                                        ElementState::Pressed => {
                                            if !actions.contains(&action) {
                                                actions.push(action);
                                            }
                                        }
                                        ElementState::Released => {
                                            actions.retain(|&a| a != action);
                                        }
                                    }
                                }
                            }
                            WindowEvent::Resized(physical_size) => {
                                log::info!("physical_size: {physical_size:?}");
                                surface_configured = true;
                                state.resize(physical_size.to_logical(1.0));
                            }
                            WindowEvent::RedrawRequested => {
                                // This tells winit that we want another frame after this one
                                state.window().request_redraw();

                                if !surface_configured {
                                    return;
                                }

                                let frame_time = state.fps_counter.frame_time() / 1000.0;

                                game(
                                    &mut |x, y, color| {
                                        let index =
                                            (y * state.texture_extent.width as usize + x) * 4;
                                        let r = ((color >> 16) & 0xFF) as u8;
                                        let g = ((color >> 8) & 0xFF) as u8;
                                        let b = (color & 0xFF) as u8;

                                        // todo: better matching
                                        #[cfg(not(target_arch = "wasm32"))]
                                        {
                                            state.pixels[index] = b;
                                            state.pixels[index + 1] = g;
                                            state.pixels[index + 2] = r;
                                            state.pixels[index + 3] = 255; // Alpha channel
                                        }

                                        #[cfg(target_arch = "wasm32")]
                                        {
                                            state.pixels[index] = r;
                                            state.pixels[index + 1] = g;
                                            state.pixels[index + 2] = b;
                                            state.pixels[index + 3] = 255; // Alpha channel
                                        }
                                    },
                                    &actions,
                                    frame_time,
                                );

                                match state.render() {
                                    Ok(_) => {}
                                    // Reconfigure the surface if it's lost or outdated
                                    Err(
                                        wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                                    ) => state.resize(state.size),
                                    // The system is out of memory, we should probably quit
                                    Err(wgpu::SurfaceError::OutOfMemory) => {
                                        log::error!("OutOfMemory");
                                        control_flow.exit();
                                    }

                                    // This happens when the a frame takes too long to present
                                    Err(wgpu::SurfaceError::Timeout) => {
                                        log::warn!("Surface timeout")
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            })
            .unwrap();
    }
}

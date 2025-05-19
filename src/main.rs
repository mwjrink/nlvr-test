use cgmath::{vec3, Matrix4, SquareMatrix};
use nlvr::lib::{renderinstance::*, surface::OutputSurface, ubo::CameraUBO};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use test_game::lib::camera::Camera;
use test_game::lib::math;

// use winit::monitor::VideoMode;
// use winit::window::Fullscreen::Exclusive;

use simple_logger::SimpleLogger;

use winit::{
    application::ApplicationHandler,
    event::{ElementState, Event, MouseButton, MouseScrollDelta, StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use cgmath::{Deg, Point3, Vector3};

use std::time::Instant;

const FRAMES_AVERAGE: u32 = 5;

// use vulkan;

// #[macro_use] extern crate shrinkwraprs;
//
// #[derive(Shrinkwrap)]
// struct Email(String);

/*
TODO
 - Use shrinkwraprs


*/

#[derive(Default)]
struct NlvrTest {
    window: Option<Window>,
    render_instance: Option<RenderInstance<CameraUBO>>,

    camera: Camera,

    frames: Vec<f64>,
    frame_index: usize,
    last: Option<Instant>,
    fps: f64,

    dirty_swapchain: bool,
    rebuild: bool,

    is_left_clicked: bool,
    cursor_position: [i32; 2],
    last_position: [i32; 2],
    wheel_delta: f32,

    dt: f32,
    seconds: f32,
}

impl ApplicationHandler for NlvrTest {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("A fantastic window!")
            .with_resizable(true)
            .with_title("Vulkan, Hello World");
        // .with_fullscreen(Some(Exclusive(VideoMode { })))
        // .with_decorations(false)

        self.window = Some(event_loop.create_window(window_attributes).unwrap());

        self.render_instance = Some(RenderInstance::<CameraUBO>::create(
            [
                self.window.as_ref().unwrap().inner_size().width,
                self.window.as_ref().unwrap().inner_size().height,
            ],
            &self
                .window
                .as_ref()
                .unwrap()
                .window_handle()
                .unwrap()
                .as_raw(),
            &self
                .window
                .as_ref()
                .unwrap()
                .display_handle()
                .unwrap()
                .as_raw(),
        ));

        let cottage_renderable = self.render_instance.as_mut().unwrap().renderable_from_file(
            "chalet/chalet.obj".to_string(),
            Some("chalet/chalet.jpg".to_string()),
        );

        // let rock_assembly_cliffs_renderable = render_instance.renderable_from_file(
        // "quixel/Rock_Assembly_Cliffs_siEoZ_8K_3d_ms/siEoZ_High.obj".to_string(),
        // None,
        // );
        // let fire_pit_renderable = render_instance.renderable_from_file(
        //     "quixel/fire_pit/fire_pit.obj".to_string(),
        //     Some("quixel/fire_pit/fire_pit_albedo.jpg".to_string()),
        // );

        // let bunny_renderable =
        //     render_instance.renderable_from_cm_file("assets/cmfiles/test_output.cm".to_string());

        // for idx in 0..1190 {
        //     // println!("file: cmfiles/clusters/cluster_{}.obj", idx);
        //     let renderable_inst = render_instance.renderable_from_file(
        //         format!("cmfiles/clusters/cluster_{}.obj", idx).to_string(),
        //         None,
        //     );
        //     render_instance
        //         .get_renderable(renderable_inst)
        //         .create_instance(Matrix4::from_translation(vec3(0.5, 0.0, 0.0)));
        // }

        let base_rot = Matrix4::from_angle_x(Deg(270.0));
        let transform_0 = Matrix4::from_translation(vec3(0.1, 0.0, -1.0)) * base_rot;
        let transform_1 = Matrix4::from_translation(vec3(0.1, 0.0, 1.0)) * Matrix4::from_scale(1.0);
        // let transform_1 = Matrix4::from_scale(1.0);
        // let transform_2 =
        //     Matrix4::from_translation(vec3(0.1, 0.0, 1.0)) * base_rot * Matrix4::from_scale(0.01);

        let _cottage_renderable_instance_0_idx = self
            .render_instance
            .as_mut()
            .unwrap()
            .get_renderable(cottage_renderable)
            .create_instance(transform_0);
        let _cottage_renderable_instance_1_idx = self
            .render_instance
            .as_mut()
            .unwrap()
            .get_renderable(cottage_renderable)
            .create_instance(transform_1);

        // let _bunny_renderable_instance_0_idx = render_instance
        //     .get_renderable(bunny_renderable)
        //     .create_instance(transform_1);

        // let mut bunny_renderable_idx = 0usize;
        // {
        //     let mut bunny = render_instance.get_renderable(bunny_renderable);
        //     for idx in 0..bunny.meshes.len() {
        //         bunny.update(idx, Matrix4::from_scale(0.0));
        //     }
        //     bunny.update(bunny_renderable_idx, transform_1);
        // };

        //     let _fire_pit_renderable_instance_0 = render_instance
        //         .get_renderable(fire_pit_renderable)
        //         .create_instance(transform_2);

        self.frames = vec![0.0; FRAMES_AVERAGE as usize];
        self.frame_index = 0;
        self.last = Some(Instant::now());
        self.fps = 0.0;

        self.dirty_swapchain = true;
        self.rebuild = true;

        self.is_left_clicked = false;
        self.cursor_position = [0, 0];
        self.last_position = self.cursor_position;
        self.wheel_delta = 0.0;

        self.dt = 0.0;
        self.seconds = 5.0;
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {}

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        if self.render_instance.is_none() {
            return;
        }

        // reset input states on new frame
        {
            // is_left_clicked = false;
            self.last_position = self.cursor_position;
            // cursor_position = [0, 0];
            self.wheel_delta = 0.0;
        }
        // frame timing info
        let now = Instant::now();
        let delta = now.duration_since(self.last.unwrap_or(now));
        self.last = Some(now);
        self.dt = delta.as_secs_f32();
        self.frames[self.frame_index] = delta.as_secs_f64();
        self.frame_index = (self.frame_index + 1) % (FRAMES_AVERAGE as usize);
        self.fps = f64::from(FRAMES_AVERAGE) / self.frames.iter().sum::<f64>();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        println!("{event:?}");

        let window = match self.window.as_ref() {
            Some(window) => window,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized { .. } => self.dirty_swapchain = true,
            // Accumulate input events
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => {
                if state == ElementState::Released {
                    self.is_left_clicked = false;
                }
                if state == ElementState::Pressed {
                    self.is_left_clicked = true;
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let position: (i32, i32) = position.into();
                self.cursor_position = [position.0, position.1];
            }
            WindowEvent::MouseWheel {
                delta: MouseScrollDelta::LineDelta(_, v_lines),
                ..
            } => {
                self.wheel_delta = v_lines;
            }
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                //                     if input.state == ElementState::Pressed {
                //                         println!("Hello!");
                //                         let mut bunny = render_instance.get_renderable(bunny_renderable);
                //                         bunny.update(bunny_renderable_idx, Matrix4::from_scale(0.0));
                //                         bunny_renderable_idx += 1;
                //                         bunny.update(bunny_renderable_idx, transform_1);
                //                         // bunny.create_instance(Matrix4::identity());
                //
                //                         rebuild = true;
                //                     }
            }
            WindowEvent::RedrawRequested => {
                // update input state after accumulating event
                // {
                //     if let Some(is_left_clicked) = is_left_clicked {
                //         is_left_clicked = is_left_clicked;
                //     }
                //     if let Some(position) = cursor_position {
                //         cursor_position = position;
                //         cursor_delta = Some([position[0] - last_position[0], position[1] - last_position[1]]);
                //     } else {
                //         cursor_delta = None;
                //     }
                //     wheel_delta = wheel_delta;
                // }

                // render
                {
                    if self.dirty_swapchain || self.rebuild {
                        let size = window.inner_size();
                        if size.width > 0 && size.height > 0 {
                            // app.recreate_swapchain();
                            self.render_instance.as_mut().unwrap().rebuild();
                            self.rebuild = false;
                        } else {
                            return;
                        }
                    }

                    let width = self.render_instance.as_ref().unwrap().render_width();
                    let height = self.render_instance.as_ref().unwrap().render_height();

                    let delta = [
                        self.cursor_position[0] - self.last_position[0],
                        self.cursor_position[1] - self.last_position[1],
                    ];

                    // translate

                    // Update uniform buffers
                    let ubo = {
                        if self.is_left_clicked && (delta[0] != 0 && delta[1] != 0) {
                            let x_ratio = delta[0] as f32 / width as f32;
                            let y_ratio = delta[1] as f32 / height as f32;
                            let theta = x_ratio * 180.0_f32.to_radians();
                            let phi = y_ratio * 90.0_f32.to_radians();
                            self.camera.rotate(theta, phi);
                        }

                        if self.wheel_delta != 0.0 {
                            self.camera.forward(self.wheel_delta * 0.3);
                        }

                        let aspect = width as f32 / height as f32;
                        let ubo = CameraUBO {
                            view: Matrix4::look_at_rh(
                                self.camera.position(),
                                Point3::new(0.0, 0.0, 0.0),
                                Vector3::new(0.0, 1.0, 0.0),
                            ),
                            proj: math::perspective(Deg(45.0), aspect, 0.1, 10.0),
                        };

                        ubo
                    };

                    self.render_instance
                        .as_mut()
                        .unwrap()
                        .update_uniform_buffers(ubo);
                    self.dirty_swapchain = self.render_instance.as_mut().unwrap().draw_frame();
                }

                self.seconds -= self.dt;
                if self.seconds <= 0.0 {
                    println!(
                        "FPS: {} \nCursor_Position: [{}, {}] \nLeft_Clicked: {}",
                        self.fps,
                        self.cursor_position[0],
                        self.cursor_position[1],
                        self.is_left_clicked
                    );

                    self.seconds = 5.0;
                }
            }
            _ => (),
        }
    }
}

fn main() {
    // simple logger
    let mut logger = SimpleLogger::new();
    logger = logger.with_level(log::LevelFilter::Warn);
    logger.init().unwrap();
    // simple logger

    let event_loop = EventLoop::new().unwrap();

    let mut app = NlvrTest::default();

    let result = event_loop.run_app(&mut app);
    if let Err(e) = result {
        log::debug!("Error occured running the application: {}", e);
    }
}

// TODO:
// - start splitting up (into files) and abstracting the vulkan operations (files like, pipeline, command buffer, queue
//   (graphics, present and compute), swapchain, image, image_view, descriptor set, command pool, texture, vector,
//   index, uniform something, descriptor pool etc)
// - model and textures along with their location, rotation scale provided from main
// - input and camera are bad, fix them
// - use a computation pipeline before the render pipeline and use the generated nlvo* files
// - swapvec ecs system

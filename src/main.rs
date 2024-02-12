pub mod render;
pub mod math;
pub mod window;

use std::{arch::x86_64::_mm_crc32_u64, io::Read};

use math::*;

impl std::fmt::Display for Vec3f {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_fmt(format_args!("<{}, {}, {}>", self.x, self.y, self.z));
    }
}

pub struct Surface {
    data: Vec<u32>,
    extent: math::Vec2<usize>
}

impl Surface {
    pub fn new(width: usize, height: usize) -> Surface {
        Surface {
            data: {
                let mut v = Vec::with_capacity(width * height);
                v.resize(width * height, 0xFF000000);
                v
            },
            extent: math::Vec2::<usize>::new(width, height),
        }
    }

    pub fn flush(&mut self, mut sdl_surface: sdl2::video::WindowSurfaceRef) -> Result<(), String> {
        let data_surface = sdl2::surface::Surface::from_data(unsafe {
            let s: &mut [u8] = std::slice::from_raw_parts_mut(std::mem::transmute(self.data.as_mut_ptr()), self.data.len() * 4);

            s
        }, self.extent.x as u32, self.extent.y as u32, self.extent.x as u32 * 4, sdl2::pixels::PixelFormatEnum::RGBX8888)?;

        let dst_size = sdl_surface.size();
        data_surface.blit(
            sdl2::rect::Rect::new(0, 0, self.extent.x as u32, self.extent.y as u32), &mut sdl_surface,
            sdl2::rect::Rect::new(0, 0, dst_size.0, dst_size.1))?;
        sdl_surface.update_window()?;
        Ok(())
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.extent.x = width;
        self.extent.y = height;
        self.data.resize(width * height, 0xFF000000);
    }
}

impl<'a> window::Surface<'a> for Surface {
    fn get_data<'b>(&'b self) -> &'b [u32] {
        self.data.as_slice()
    }

    fn get_data_mut<'b>(&'b mut self) -> &'b mut [u32] {
        self.data.as_mut_slice()
    }

    fn get_extent(&self) -> math::Vec2<usize> {
        self.extent
    }
}
struct Timer {
    start_time_point: std::time::Instant,
    time_point: std::time::Instant,
    fps_time_point: std::time::Instant,
    time: f32,
    delta_time: f32,
    fps: f32,
    fps_counter: u32,
    fps_duration: f32,
}

impl Timer {
    pub fn new() -> Self {
        let now = std::time::Instant::now();
        Self {
            start_time_point: now.clone(),
            time_point: now.clone(),
            fps_time_point: now.clone(),
            time: 0.0,
            delta_time: 0.01,
            fps: 30.0,
            fps_counter: 0,
            fps_duration: 3.0,
        }
    }

    pub fn response(&mut self) {
        let now = std::time::Instant::now();

        self.time = (now - self.start_time_point).as_secs_f32();
        self.delta_time = (now - self.time_point).as_secs_f32();


        self.fps_counter += 1;

        let fps_duration = (now - self.fps_time_point).as_secs_f32();
        if fps_duration >= self.fps_duration {
            self.fps = self.fps_counter as f32 / fps_duration;
            self.fps_time_point = now;
            self.fps_counter = 0;
        }

        self.time_point = now;
    }

    pub fn get_time(&self) -> f32 {
        self.time
    }

    pub fn get_delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn get_fps(&self) -> f32 {
        self.fps
    }
}

pub fn load_obj(path: &str) -> Result<render::Primitive, String> {
    let text = {
        let mut file = std::fs::File::open(path).map_err(|err| err.to_string())?;
        let mut buf = String::new();

        _ = file.read_to_string(&mut buf);

        buf
    };

    let mut positions = Vec::<Vec3f>::new();
    let mut normals = Vec::<Vec3f>::new();

    positions.push(Vec3f {x: 0.0, y: 0.0, z: 0.0});
    normals.push(Vec3f {x: 0.0, y: 1.0, z: 0.0});

    let mut primitive_idx = Vec::<u32>::new();
    let mut primitive_ns = Vec::<Vec3f>::new();

    for (line_number, line) in text.split('\n').enumerate() {
        let line = line.trim();
        let elems: Vec<&str> = line.split(' ').collect();

        if elems.len() < 1 {
            continue;
        }

        match *unsafe { elems.get_unchecked(0) } {
            "v" => {
                if elems.len() >= 4 {
                    unsafe {
                        positions.push(Vec3f {
                            x: elems.get_unchecked(1).parse::<f32>().unwrap_or(0.0),
                            y: elems.get_unchecked(2).parse::<f32>().unwrap_or(0.0),
                            z: elems.get_unchecked(3).parse::<f32>().unwrap_or(0.0),
                        });
                    }
                }

            },
            "vn" => {
                if elems.len() >= 4 {
                    unsafe {
                        normals.push(Vec3f {
                            x: elems.get_unchecked(1).parse::<f32>().unwrap_or(0.0),
                            y: elems.get_unchecked(2).parse::<f32>().unwrap_or(0.0),
                            z: elems.get_unchecked(3).parse::<f32>().unwrap_or(0.0),
                        });
                    }
                }
            },
            "f" => if elems.len() >= 3 {
                let mut vertex_count: usize = 0;
                let mut normal = Vec3f::new(0.0, 0.0, 0.0);
                primitive_idx.push(0); // new vertex
                primitive_idx.push(primitive_ns.len() as u32); // new normal
                for vertex in &elems[1..] {
                    vertex_count += 1;

                    let components: Vec<&str> = vertex.split('/').collect();

                    if components.len() != 3 {
                        return Err(format!("OBJ Parsing error({path}, {line_number}): incorrect number of vertex components"));
                    }
                    unsafe {
                        normal += *normals.get_unchecked(components.get_unchecked(2).parse::<u32>().unwrap_or(0) as usize);
                        primitive_idx.push(components.get_unchecked(0).parse::<u32>().unwrap_or(0));
                    }
                }

                unsafe {
                    let len = primitive_idx.len();
                    *primitive_idx.get_unchecked_mut(len - vertex_count - 2) = vertex_count as u32;
                }
                primitive_ns.push(normal.normalized());
            }
            _ => {},
        }
    }

    Ok(render::Primitive {
        color: 0x00FF00,
        indices: primitive_idx,
        positions,
        normals: primitive_ns,
    })
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    let window = video.window("TioT2 Wire 3D", 800, 600)
        .resizable()
        .build().unwrap();

    let mut render = render::Render::new();
    let mut surface = Surface::new(800, 600);
    let mut timer = Timer::new();
    let mut frame = 0;

    let cow = load_obj("models/e1m1.obj").unwrap();
    let triangle = render::Primitive {
        color: 0x00FF00,
        indices: vec![3, 0, 0, 1, 2],
        normals: vec![Vec3f::new(0.0, 0.0, 1.0)],
        positions: vec![
            Vec3f::new( 0.000,  1.000, 0.000),
            Vec3f::new(-0.866, -0.500, 0.000),
            Vec3f::new( 0.866, -0.500, 0.000),
        ],
    };

    // render.get_camera_mut().set(&Vec3f::new(0.0, 0.0, -50.0), &Vec3f::new(0.0, 0.0, 0.0), &Vec3f::new(0.0, 1.0, 0.0));

    'main_loop: loop {
        'event_loop: loop {
            let event = match event_pump.poll_event() {
                Some(event) => event,
                None => break 'event_loop,
            };

            match event {
                sdl2::event::Event::Window { window_id, win_event, .. } => if window_id == window.id() {
                    match win_event {
                        sdl2::event::WindowEvent::Close => break 'main_loop,
                        sdl2::event::WindowEvent::Resized(width, height) => surface.resize(width.unsigned_abs() as usize, height.unsigned_abs() as usize),
                        _ => {},
                    }
                },
                sdl2::event::Event::Quit{..} => break 'main_loop,
                _ => {},
            }
        }

        // Camera control
        'camera_control: {
            let state = event_pump.keyboard_state();
            let move_axis = Vec3f::new(
                (state.is_scancode_pressed(sdl2::keyboard::Scancode::D) as i32 - state.is_scancode_pressed(sdl2::keyboard::Scancode::A) as i32) as f32,
                (state.is_scancode_pressed(sdl2::keyboard::Scancode::R) as i32 - state.is_scancode_pressed(sdl2::keyboard::Scancode::F) as i32) as f32,
                (state.is_scancode_pressed(sdl2::keyboard::Scancode::W) as i32 - state.is_scancode_pressed(sdl2::keyboard::Scancode::S) as i32) as f32,
            );
            let rotate_axis = Vec2f::new(
              (state.is_scancode_pressed(sdl2::keyboard::Scancode::Right) as i32 - state.is_scancode_pressed(sdl2::keyboard::Scancode::Left) as i32) as f32,
              (state.is_scancode_pressed(sdl2::keyboard::Scancode::Down) as i32 - state.is_scancode_pressed(sdl2::keyboard::Scancode::Up) as i32) as f32,
            );

            if move_axis.length() <= 0.01 && rotate_axis.length() <= 0.01 {
                break 'camera_control;
            }

            let camera = render.get_camera_mut();

            let camera_location= camera.get_location();
            let movement_delta = (
                camera_location.right     * move_axis.x +
                camera_location.up        * move_axis.y +
                camera_location.direction * move_axis.z
            ) * timer.get_delta_time() * 8.0;

            let mut azimuth = camera_location.direction.y.acos();
            let mut elevator = camera_location.direction.z.signum() * (
                camera_location.direction.x / (
                    camera_location.direction.x * camera_location.direction.x +
                    camera_location.direction.z * camera_location.direction.z
                ).sqrt()
            ).acos();

            elevator += rotate_axis.x * timer.get_delta_time() * 2.0;
            azimuth += rotate_axis.y * timer.get_delta_time() * 2.0;

            azimuth = azimuth.clamp(0.01, std::f32::consts::PI - 0.01);

            let new_direction = Vec3f{
                x: azimuth.sin() * elevator.cos(),
                y: azimuth.cos(),
                z: azimuth.sin() * elevator.sin()
            };

            camera.set(&(camera_location.location + movement_delta), &(camera_location.location + movement_delta + new_direction), &Vec3f {x: 0.0, y: 1.0, z: 0.0});
        }

        timer.response();

        let mut context = render.start(&mut surface);

        // rendering
        context.draw(&triangle);
        context.draw(&cow);

        context.finish();

        if let Ok(dst_surface) = window.surface(&event_pump) {
            _ = surface.flush(dst_surface);
        }

        if frame % 1000 == 0 {
            println!("{}", timer.get_fps());
        }
        frame += 1;
    }
}

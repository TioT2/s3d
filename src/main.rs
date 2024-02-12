pub mod render;
pub mod math;
pub mod window;

use math::*;

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
        }, self.extent.x as u32, self.extent.y as u32, self.extent.x as u32 * 4, sdl2::pixels::PixelFormatEnum::ABGR8888)?;

        let dst_size = sdl_surface.size();
        data_surface.blit(
            sdl2::rect::Rect::new(0, 0, self.extent.x as u32, self.extent.y as u32), &mut sdl_surface,
            sdl2::rect::Rect::new(0, 0, dst_size.0, dst_size.1))?;
        sdl_surface.update_window()?;
        Ok(())
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

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    let window = video.window("TioT2 Wire 3D", 800, 600).build().unwrap();

    let mut render = render::Render::new();
    let mut surface = Surface::new(800, 600);
    let mut timer = Timer::new();
    let mut frame = 0;

    let mut triangle = render::Primitive {
        color: 0x00FF00,
        indices: vec![3, 0, 0, 1, 2],
        normals: vec![Vec3f::new(0.0, 0.0, 1.0)],
        positions: vec![
            Vec3f::new(400.0, 500.0, 0.0),
            Vec3f::new(226.8, 200.0, 0.0),
            Vec3f::new(573.2, 200.0, 0.0),
        ],
    };

    'main_loop: loop {
        'event_loop: loop {
            let event = match event_pump.poll_event() {
                Some(event) => event,
                None => break 'event_loop,
            };

            match event {
                sdl2::event::Event::Window { window_id, win_event, .. } => if window_id == window.id() {
                    match win_event {
                        sdl2::event::WindowEvent::Close => {
                            break 'main_loop;
                        }
                        _ => {},
                    }
                },
                sdl2::event::Event::Quit{..} => break 'main_loop,
                _ => {},
            }
        }

        timer.response();

        // Update triangle positions
        triangle.positions = triangle.positions.into_iter().enumerate().map(|(id, mut vt)| {
            const DELTA_ALPHA: f32 = std::f32::consts::PI * (2.0 / 3.0);
            let angle = id as f32 * DELTA_ALPHA + timer.get_time() * 0.0;

            vt.x = angle.sin();
            vt.y = angle.cos();

            return vt;
        }).collect();

        // Setup render camera
        render.lock_camera().set(& {
            let angle = timer.get_time();

            Vec3f::new(angle.cos() * 5.0, 3.0, angle.sin() * 5.0)
        }, &Vec3f::new(0.0, 0.0, 0.0), &Vec3f::new(0.0, -1.0, 0.0));

        let mut context = render.start(&mut surface);

        // rendering
        context.draw(&triangle);

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

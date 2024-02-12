pub use crate::math::*;

pub struct Primitive {
    pub positions: Vec<Vec3f>,
    pub normals: Vec<Vec3f>,
    pub indices: Vec<u32>,
    pub color: u32,
}

pub struct CameraLocation {
    pub direction: Vec3f,
    pub right: Vec3f,
    pub up: Vec3f,
    pub location: Vec3f,
    pub at: Vec3f,
}

pub struct CameraProjection {
    pub size: Vec2f,
    pub near: f32,
    pub far: f32,
}

pub struct Camera {
    location: CameraLocation,
    projection: CameraProjection,

    view_matrix: Mat4x4f,
    projection_matrix: Mat4x4f,
    view_projection_matrix: Mat4x4f,
    extent: Vec2<usize>,
}

impl Camera {
    pub fn new() -> Self {
        let mut cam = Self {
            location: CameraLocation {
                direction: Vec3f::new(0.0, 0.0, -1.0),
                right: Vec3f::new(1.0, 0.0, 0.0),
                up: Vec3f::new(0.0, 1.0, 0.0),

                location: Vec3f::new(0.0, 0.0, 1.0),
                at: Vec3f::new(0.0, 0.0, 0.0),
            },

            projection: CameraProjection {
                size: Vec2f::new(1.0, 1.0),
                near: 1.0,
                far: 100.0,
            },

            view_matrix: Mat4x4f::identity(),
            projection_matrix: Mat4x4f::identity(),
            view_projection_matrix: Mat4x4f::identity(),
            extent: Vec2::<usize>::new(0, 0),
        };

        cam.resize(Vec2::<usize>::new(800, 600));
        cam.set_projection(0.05, 100.0, Vec2f::new(0.1, 0.1));

        cam
    }

    pub fn set(&mut self, location: &Vec3f, at: &Vec3f, approx_up: &Vec3f) {
        let view = Mat4x4::view(location, at, approx_up);

        self.location.right     = Vec3f::new( view.data[0][0],  view.data[1][0],  view.data[2][0]);
        self.location.up        = Vec3f::new( view.data[0][1],  view.data[1][1],  view.data[2][1]);
        self.location.direction = Vec3f::new(-view.data[0][2], -view.data[1][2], -view.data[2][2]);

        self.location.location = *location;
        self.location.at = *at;

        self.view_matrix = view;
        self.view_projection_matrix = self.view_matrix * self.projection_matrix;
    }

    pub fn get_location(&self) -> &CameraLocation {
        &self.location
    }

    pub fn get_projection(&self) -> &CameraProjection {
        &self.projection
    }

    pub fn set_projection(&mut self, near: f32, far: f32, size: Vec2f) {
        self.projection.near = near;
        self.projection.far = far;
        self.projection.size = size;

        let proj_ext = self.projection.size * if self.extent.x > self.extent.y {
            Vec2f::new(self.extent.x as f32 / self.extent.y as f32, 1.0)
        } else {
            Vec2f::new(1.0, self.extent.y as f32 / self.extent.x as f32)
        };

        self.projection_matrix = Mat4x4f::projection_frustum(-proj_ext.x / 2.0, proj_ext.x / 2.0, -proj_ext.y / 2.0, proj_ext.y / 2.0, self.projection.near, self.projection.far);
        self.view_projection_matrix = self.view_matrix * self.projection_matrix;
    }

    fn resize(&mut self, new_extent: Vec2<usize>) {
        if self.extent.x == new_extent.x && self.extent.y == new_extent.y {
            return;
        }
        self.extent = new_extent;

        let proj_ext = self.projection.size * if self.extent.x > self.extent.y {
            Vec2f::new(self.extent.x as f32 / self.extent.y as f32, 1.0)
        } else {
            Vec2f::new(1.0, self.extent.y as f32 / self.extent.x as f32)
        };

        self.projection_matrix = Mat4x4f::projection_frustum(-proj_ext.x / 2.0, proj_ext.x / 2.0, -proj_ext.y / 2.0, proj_ext.y / 2.0, self.projection.near, self.projection.far);
        self.view_projection_matrix = self.view_matrix * self.projection_matrix;
    }
}

pub struct Render {
    camera: Camera,
}

pub struct RenderContext<'a> {
    render: &'a mut Render,
    surface_width: usize,
    surface_height: usize,
    surface_data: *mut u32,
}

impl<'a> RenderContext<'a> {
    unsafe fn set_pixel_unchecked(&mut self, x: usize, y: usize, color: u32) {
        *self.surface_data.add(y * self.surface_width + x) = color;
    }

    /// Line displaying function
    unsafe fn draw_line_unchecked(&mut self, mut x1: usize, mut y1: usize, mut x2: usize, mut y2: usize, color: u32) {
        if y1 > y2 {
            let tmp = y1;
            y1 = y2;
            y2 = tmp;
            let tmp = x1;
            x1 = x2;
            x2 = tmp;
        }
        let dy: usize = y2 - y1;
        let (dx, sx): (usize, usize) = if x2 < x1 {
            (x1 - x2, 1usize.wrapping_neg())
        } else {
            (x2 - x1, 1usize)
        };
        let mut x = x1;
        let mut y = y1;

        self.set_pixel_unchecked(x, y, color);

        if dx >= dy {
            let mut f = (2 * dy).wrapping_sub(dx);
            let ie = 2 * dy;
            let ine = ie.wrapping_sub(2 * dx);
            let mut count = dx;

            while count != 0 {
                if f < std::mem::transmute(isize::MIN) {
                    y += 1;
                    f = f.wrapping_add(ine);
                } else {
                    f = f.wrapping_add(ie);
                }
                x = x.wrapping_add(sx);
                count -= 1;
                self.set_pixel_unchecked(x, y, color);
            }
        } else {
            let mut f = (2 * dx).wrapping_sub(dy);
            let ie = 2 * dx;
            let ine = ie.wrapping_sub(2 * dy);
            let mut count = dy;

            while count != 0 {
                if f < std::mem::transmute(isize::MIN) {
                    x = x.wrapping_add(sx);
                    f = f.wrapping_add(ine);
                } else {
                    f = f.wrapping_add(ie);
                }
                y += 1;
                count -= 1;
                self.set_pixel_unchecked(x, y, color);
            }
        }
    }

    pub fn draw(&mut self, primitive: &Primitive) {
        unsafe {
            let color = primitive.color << 8;
            let positions = primitive.positions.as_ptr();
            let normals = primitive.normals.as_ptr();

            let mut index = primitive.indices.as_ptr();
            let end = index.add(primitive.indices.len());

            // Walk through faces
            while index < end {
                let last_position_index = index.add(*index as usize + 2);
                let _normal_index = *index.add(1);
                index = index.add(2);

                let mut i1 = *last_position_index.sub(1) as usize;
                let mut i2 = *index as usize;

                while index < last_position_index {
                    let mut begin = self.render.camera.view_projection_matrix.transform_4x4(*positions.add(i1));
                    let mut end = self.render.camera.view_projection_matrix.transform_4x4(*positions.add(i2));

                    if begin.z > 0.0 && begin.z < 1.0 && end.z > 0.0 && end.z < 1.0 {
                        begin.x = (begin.x + 1.0) / 2.0 * self.surface_width as f32;
                        begin.y = (begin.y + 1.0) / 2.0 * self.surface_height as f32;

                        end.x = (end.x + 1.0) / 2.0 * self.surface_width as f32;
                        end.y = (end.y + 1.0) / 2.0 * self.surface_height as f32;

                        let begin = (begin.x as usize, begin.y as usize);
                        let end = (end.x as usize, end.y as usize);

                        if begin.0 < self.surface_width && begin.1 < self.surface_height && end.0 < self.surface_width && end.1 < self.surface_height {
                            self.draw_line_unchecked(begin.0, begin.1, end.0, end.1, color);
                        }
                    }

                    index = index.add(1);
                    i1 = i2;
                    i2 = *index as usize;
                }
            }
        }
    }

    pub fn finish(self) {

    }
}

impl Render {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
        }
    }

    pub fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn start<'a>(&'a mut self, surface: &'a mut dyn crate::window::Surface<'a>) -> RenderContext<'a> {
        // Clear canvas
        unsafe {
            let data = surface.get_data_mut();

            std::ptr::write_bytes(data.as_mut_ptr(), 0x00, data.len());
        }

        self.camera.resize(surface.get_extent());
        RenderContext {
            render: self,
            surface_width: surface.get_extent().x,
            surface_height: surface.get_extent().y,
            surface_data: surface.get_data_mut().as_mut_ptr(),
        }
    }
}
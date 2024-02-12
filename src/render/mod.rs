pub use crate::math::*;

pub struct Primitive {
    pub positions: Vec<Vec3f>,
    pub normals: Vec<Vec3f>,
    pub indices: Vec<u32>,
    pub color: u32,
}

#[derive(Copy, Clone)]
pub struct CameraLocation {
    pub direction: Vec3f,
    pub right: Vec3f,
    pub up: Vec3f,
    pub location: Vec3f,
    pub at: Vec3f,
}

#[derive(Copy, Clone)]
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
    /// Line displaying function
    unsafe fn draw_line_unchecked(&self, x1: usize, y1: usize, x2: usize, y2: usize, color: u32) {
        let (mut dy, sy): (usize, usize) = if y2 < y1 {
            (y1 - y2, self.surface_width.wrapping_neg())
        } else {
            (y2 - y1, self.surface_width)
        };
        let (mut dx, sx): (usize, usize) = if x2 < x1 {
            (x1 - x2, 1usize.wrapping_neg())
        } else {
            (x2 - x1, 1usize)
        };

        let mut pptr = self.surface_data.wrapping_add(y1 * self.surface_width + x1);
        pptr.write(color);

        if dx >= dy {
            let ie = 2 * dy;
            let mut f = ie.wrapping_sub(dx);
            let ine = ie.wrapping_sub(2 * dx);

            while dx != 0 {
                pptr = pptr.wrapping_add(sx);
                pptr.write(color);
                dx -= 1;
                if f < std::mem::transmute(isize::MIN) {
                    pptr = pptr.wrapping_add(sy);
                    f = f.wrapping_add(ine);
                } else {
                    f = f.wrapping_add(ie);
                }
            }
        } else {
            let ie = 2 * dx;
            let mut f = ie.wrapping_sub(dy);
            let ine = ie.wrapping_sub(2 * dy);

            while dy != 0 {
                pptr = pptr.wrapping_add(sy);
                pptr.write(color);
                dy -= 1;

                if f < std::mem::transmute(isize::MIN) {
                    pptr = pptr.wrapping_add(sx);
                    f = f.wrapping_add(ine);
                } else {
                    f = f.wrapping_add(ie);
                }
            }
        }
    }

    unsafe fn set_pixel_unchecked(&self, x: usize, y: usize, color: u32) {
        *self.surface_data.add(y * self.surface_width + x) = color;
    }

    unsafe fn draw_polygon_border_unchecked(&self, polygon: &[Vec2<usize>], bottom_index: usize, color: u32) {
        // actually, render face (wireframe at least now)
        let mut fp = polygon.as_ptr();
        let fpe = fp.add(polygon.len() - 1);

        self.draw_line_unchecked(fp.read().x, fp.read().y, fpe.read().x, fpe.read().y, color);
        while fp < fpe {
            self.draw_line_unchecked(fp.read().x, fp.read().y, fp.add(1).read().x, fp.add(1).read().y, color);
            fp = fp.add(1);
        }

        self.set_pixel_unchecked(polygon.get_unchecked(bottom_index).x, polygon.get_unchecked(bottom_index).y, 0xFF000000);
    }

    unsafe fn draw_polygon_unchecked(&self, polygon: &[Vec2<usize>], bottom_index: usize, color: u32) {
        // Do some scanline
        todo!();
    }

    pub fn draw(&mut self, primitive: &Primitive) {
        unsafe {
            let cam_loc = *self.render.camera.get_location();

            let proj = *self.render.camera.get_projection();
            let proj_inv_near = 1.0 / proj.near;
            let proj_inv_far = 1.0 / proj.far;

            let proj_ext_min = usize::min(self.render.camera.extent.x, self.render.camera.extent.y) as f32;
            let proj_x_x = 2.0 * proj.near / proj.size.x * self.render.camera.extent.y as f32 / proj_ext_min;
            let proj_y_y = -2.0 * proj.near / proj.size.y * self.render.camera.extent.x as f32 / proj_ext_min;

            let cam_right = cam_loc.right;
            let cam_up = cam_loc.up;
            let cam_dir = cam_loc.direction;

            let cam_loc_right = cam_loc.location ^ cam_right;
            let cam_loc_up = cam_loc.location ^ cam_up;
            let cam_loc_dir = cam_loc.location ^ cam_dir;

            let proj_x_add = self.surface_width as f32 / 2.0;
            let proj_x_mul = proj_x_add * proj_x_x;

            let proj_y_add = self.surface_height as f32 / 2.0;
            let proj_y_mul = proj_y_add * proj_y_y;

            let color = primitive.color << 8;
            let positions = primitive.positions.as_ptr();
            let normals = primitive.normals.as_ptr();

            let mut index = primitive.indices.as_ptr();
            let index_end = index.add(primitive.indices.len());

            // Projected face data
            let mut face_polygon = Vec::<Vec2<usize>>::with_capacity(10);

            // Walk through faces, build 'em, then render.
            while index < index_end {
                // next begin
                let face_end = index.add(*index as usize + 2);
                let normal = *normals.add(*index.add(1) as usize + 1);
                let light = (1.0 / (normal.x + normal.y + normal.z).clamp(0.1, 1.0)) as u8;
                let face_color: [u8; 4] = std::mem::transmute(color);
                let face_color: u32 = std::mem::transmute([
                    face_color[0] / light,
                    face_color[1] / light,
                    face_color[2] / light,
                    face_color[3] / light,
                ]);

                // Iterate through vertices
                'face_rendering: {
                    index = index.add(2);

                    // detect projected polygon bottom
                    let mut bottom_y = usize::MAX;
                    let mut bottom_index = 0usize;
                    let mut i = 0usize;

                    // Build face polygon
                    while index < face_end {
                        let pt = *positions.add(*index as usize);

                        let z = 1.0 / (pt.x * cam_dir.x   + pt.y * cam_dir.y   + pt.z * cam_dir.z   - cam_loc_dir);
                        let px = ((pt.x * cam_right.x + pt.y * cam_right.y + pt.z * cam_right.z - cam_loc_right) * z * proj_x_mul + proj_x_add).to_int_unchecked::<usize>();
                        let py = ((pt.x * cam_up.x    + pt.y * cam_up.y    + pt.z * cam_up.z    - cam_loc_up   ) * z * proj_y_mul + proj_y_add).to_int_unchecked::<usize>();

                        // face clipping
                        if px >= self.surface_width || py >= self.surface_height || z >= proj_inv_near || z <= proj_inv_far {
                            break 'face_rendering;
                        }

                        face_polygon.push(Vec2::<usize> { x: px, y: py });

                        if py < bottom_y {
                            bottom_y = py;
                            bottom_index = i;
                        }

                        i += 1;
                        index = index.add(1);
                    }

                    // Perform rendering
                    self.draw_polygon_border_unchecked(&face_polygon, bottom_index, face_color);
                }

                face_polygon.clear();
                index = face_end;
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
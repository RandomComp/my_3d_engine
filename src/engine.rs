use std::{mem, ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign}};

macro_rules! make_vec_type {
    (
        $name:ident<$t:ty> => $( $coord:ident ),*
    ) => {
        #[derive(Copy, Clone, Debug, Default)]
        pub struct $name {
            $( $coord: $t ),*
        }

        impl $name {
            pub fn new($( $coord: $t ),*) -> Self {
                Self {
                    $(
                        $coord,
                    )*
                }
            }

            #[allow(unused)]
            pub fn gray(val: $t) -> Self {
                Self {
                    $(
                        $coord: val,
                    )*
                }
            }
        }

        impl Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    $(
                        $coord: self.$coord + rhs.$coord,
                    )*
                }
            }
        }

        impl Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    $(
                        $coord: self.$coord - rhs.$coord,
                    )*
                }
            }
        }

        impl Mul for $name {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                Self {
                    $(
                        $coord: self.$coord * rhs.$coord,
                    )*
                }
            }
        }

        impl Div for $name {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                Self {
                    $(
                        $coord: self.$coord / rhs.$coord,
                    )*
                }
            }
        }

        impl AddAssign for $name {
            fn add_assign(&mut self, rhs: Self) {
                $(
                    self.$coord += rhs.$coord;
                )*
            }
        }

        impl SubAssign for $name {
            fn sub_assign(&mut self, rhs: Self) {
                $(
                    self.$coord -= rhs.$coord;
                )*
            }
        }

        impl MulAssign for $name {
            fn mul_assign(&mut self, rhs: Self) {
                $(
                    self.$coord *= rhs.$coord;
                )*
            }
        }

        impl DivAssign for $name {
            fn div_assign(&mut self, rhs: Self) {
                $(
                    self.$coord /= rhs.$coord;
                )*
            }
        }
    };
}

make_vec_type!(Pos2D<f32> => x, y);
make_vec_type!(Pos3D<f32> => x, y, z);

// impl<T> From<(T, T)> for $name<T> {
//     fn from(value: (T, T)) -> Self {
//         Self {
//             x: value.0.into(),
//             y: value.1.into(),
//         }
//     }
// }

// impl<T> From<$name<T>> for (T, T) {
//     fn from(value: $name<T>) -> Self {
//         (value.x, value.y)
//     }
// }

pub type Size = Pos2D;
pub type Color = Pos3D;

#[derive(Default, Debug, Copy, Clone)]
pub struct WorldPos {
    begin: Pos2D,
}

#[derive(Debug, Copy, Clone)]
pub enum ObjectVertices {
    Line {
        begin: Pos2D, end: Pos2D, width: f32,
    },
    Triangle {
        v0: Pos2D,
        v1: Pos2D,
        v2: Pos2D,
    },
}

impl ObjectVertices {
    pub fn bounding_box(&self) -> (Pos2D, Pos2D, Pos2D, Pos2D) {
        match *self {
            ObjectVertices::Line { begin, end, width } => {
                (
                    begin,
                    end,
                    Pos2D::new(begin.x + width, begin.y + width),
                    Pos2D::new(end.x + width, end.y + width),
                )
            }
            ObjectVertices::Triangle { v0, v1, v2 } => {
                let (leftx, lefty) = (v0.x.min(v1.x).min(v2.x), v0.y.min(v1.y).min(v2.y));
                let (rightx, righty) = (v0.x.max(v1.x).max(v2.x), v0.y.max(v1.y).max(v2.y));

                (
                    Pos2D::new(leftx, lefty),
                    Pos2D::new(rightx, lefty),
                    Pos2D::new(leftx, righty),
                    Pos2D::new(rightx, righty),
                )
            }
        }
    }
}

impl std::ops::Add<WorldPos> for ObjectVertices {
    type Output = Self;

    fn add(self, rhs: WorldPos) -> Self {
        match self {
            ObjectVertices::Line { begin, end, width } => {
                ObjectVertices::Line { begin: begin + rhs.begin, end: end + rhs.begin, width }
            }
            ObjectVertices::Triangle { v0, v1, v2 } => {
                ObjectVertices::Triangle {
                    v0: v0 + rhs.begin,
                    v1: v1 + rhs.begin,
                    v2: v2 + rhs.begin,
                }
            }
        }
    }
}

impl std::ops::AddAssign<WorldPos> for ObjectVertices {
    fn add_assign(&mut self, rhs: WorldPos) {
        match self {
            ObjectVertices::Line { begin, end, width: _ } => {
                *begin += rhs.begin;
                *end   += rhs.begin;
            }
            ObjectVertices::Triangle { v0, v1, v2 } => {
                *v0 += rhs.begin;
                *v1 += rhs.begin;
                *v2 += rhs.begin;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    local_pos: WorldPos,
    color: Color,
    vertices: ObjectVertices,
}

impl Object {
    pub fn bounding_box(&self) -> (Pos2D, Pos2D, Pos2D, Pos2D) {
        let vertices = self.vertices + self.local_pos;

        vertices.bounding_box()
    }
}

#[derive(Debug)]
pub struct Engine {
    world_pos: WorldPos,
    objects: Vec<Object>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            world_pos: WorldPos::default(),
            objects: Vec::new()
        }
    }
    pub fn take_objects(&mut self) -> Vec<Object> {
        mem::take(&mut self.objects)
    }
    pub fn take_matrix(&mut self) -> WorldPos {
        mem::take(&mut self.world_pos)
    }
    pub fn load_matrix(&mut self, matrix: WorldPos) {
        self.world_pos = matrix;
    }

    pub fn translate(&mut self, pos: Pos2D) {
        self.world_pos.begin += pos;
    }

    pub fn triangle(&mut self, color: Color, v0: Pos2D, v1: Pos2D, v2: Pos2D) {
        self.objects.push(
            Object {
                local_pos: self.world_pos,
                color,
                vertices: ObjectVertices::Triangle { v0, v1, v2 }
            }
        );
    }
    pub fn wf_triangle(&mut self, color: Color, up: Pos2D, downleft: Pos2D, downright: Pos2D) {
        self.line(color, 1.0, up, downleft);
        self.line(color, 1.0, up, downright);
        self.line(color, 1.0, downleft, downright);
    }

    pub fn line(&mut self, color: Color, width: f32, begin: Pos2D, end: Pos2D) {
        self.objects.push(
            Object {
                local_pos: self.world_pos,
                color,
                vertices: ObjectVertices::Line { begin, end, width }
            }
        );
    }

    pub fn wf_square(&mut self, color: Color, upleft: Pos2D, upright: Pos2D, downleft: Pos2D, downright: Pos2D) {
        self.wf_triangle(color * Color::gray(0.3), downleft, upleft, upright);
        self.wf_triangle(color * Color::gray(0.8), upright, downleft, downright);
    }

    pub fn square(&mut self, color: Color, upleft: Pos2D, upright: Pos2D, downleft: Pos2D, downright: Pos2D) {
        self.triangle(color * Color::gray(0.3), downleft, upleft, upright);
        self.triangle(color * Color::gray(0.8), upright, downleft, downright);
    }
}

#[derive(Debug)]
pub struct Surface<'a> {
    pub frame: &'a mut [u32],
    pub width: u32,
    pub height: u32,
}

impl<'a> Surface<'a> {
    #[allow(unused)]
    fn get(&self, index: (u32, u32)) -> Option<&u32> {
        if index.0 > self.width || index.1 > self.height {
            return None
        }

        let (x, y) = (index.0 as usize, index.1 as usize);

        self.frame.get(x + (y * self.width as usize))
    }

    fn get_mut(&mut self, index: (u32, u32)) -> Option<&mut u32> {
        if index.0 >= self.width || index.1 >= self.height {
            return None
        }

        let (x, y) = (index.0 as usize, index.1 as usize);

        self.frame.get_mut(x + (y * self.width as usize))
    }

    fn fill_all(&mut self, color: u32) {
        self.frame.fill(color);
    }

    fn fill(&mut self, start: (u32, u32), end: (u32, u32), color: u32) {
        let (start, end) = if start.0 > end.0 || start.1 > end.1 {
            (end, start)
        } else {
            (start, end)
        };

        if start.0 >= self.width || start.1 >= self.height {
            return
        }

        let start = start.0 as usize + (start.1 as usize * self.width as usize);
        let end = end.0.min(self.width) as usize + (end.1.min(self.height) as usize * self.width as usize);

        if let Some(frame) = self.frame.get_mut(start..end) {
            frame.fill(color);
        }
    }
}

#[derive(Debug)]
pub struct Renderer();

impl Renderer {
    pub fn render(frame: &mut Surface, objects: Vec<Object>) {
        for Object { local_pos, color, mut vertices } in objects {
            vertices += local_pos;

            // if vertices.bounding_box() {
            //     return
            // }

            match vertices {
                ObjectVertices::Line { begin, end, width } => {
                    Self::render_line(frame, color, width, begin, end);
                }
                ObjectVertices::Triangle { v0, v1, v2 } => {
                    Self::render_triangle(frame, color, v0, v1, v2);
                }
            }
        }
    }

    pub fn clear(frame: &mut Surface, color: Color) {
        let (r, g, b) = (
            (color.x * 255.0).clamp(0.0, 255.0) as u32,
            (color.y * 255.0).clamp(0.0, 255.0) as u32,
            (color.z * 255.0).clamp(0.0, 255.0) as u32,
        );

        frame.fill_all(r + (g << 8) + (b << 16) + (0xff << 24));
    }

    fn render_line(frame: &mut Surface, color: Color, width: f32, start: Pos2D, end: Pos2D) {
        let mut pos = start;

        let d = end - pos;

        let dx = (d.x / d.y).abs();

        let dx = if dx.is_nan() { 0.0 } else { dx };

        let dy = (d.y / d.x).abs();
        let dy = if dy.is_nan() { 0.0 } else { dy };

        let (dx, dy) = if dx < dy {
            (
                d.x.clamp(-dx, dx),
                d.y.clamp(-1.0, 1.0)
            )
        } else {
            (
                d.x.clamp(-1.0, 1.0),
                d.y.clamp(-dy, dy)
            )
        };

        loop {
            // println!("d = {d:?}");

            // println!("diff = {:?}", (pos / end));

            if  (pos.x / end.x) >= (1.0 - 0.001) &&
                (pos.y / end.y) >= (1.0 - 0.001) {
                break
            }

            Self::render_square(frame, color, pos, Pos2D::new(width, width));

            pos.x += dx; pos.y += dy;
        }
    }

    fn render_triangle(frame: &mut Surface, color: Color, mut v0: Pos2D, v1: Pos2D, v2: Pos2D) {
        let (mut v1, mut v2) = if v1.x <= v2.x {
            (v1, v2)
        } else {
            (v2, v1)
        };

        // println!("up = {:?}, downleft = {:?}, downright = {:?}", up, downleft, downright);

        let up = {
            let mut up_copy = v0;

            let up_ref = [&mut v0, &mut v1, &mut v2].into_iter()
                            .min_by(|a, b| a.y.total_cmp(&b.y)).unwrap();

            mem::swap(up_ref, &mut up_copy);

            up_copy
        };

        // println!("up = {up:?}");

        // println!("up = {:?}, downleft = {:?}, downright = {:?}", up, downleft, downright);

        fn flat_bottom(frame: &mut Surface, color: Color, v0: Pos2D, v1: Pos2D, v2: Pos2D) {
            let (v1, v2) = if v1.x < v2.x {
                (v1, v2)
            } else {
                (v2, v1)
            };

            let (r, g, b) = (
                (color.x * 255.0).clamp(0.0, 255.0) as u32,
                (color.y * 255.0).clamp(0.0, 255.0) as u32,
                (color.z * 255.0).clamp(0.0, 255.0) as u32,
            );

            let color = r + (g << 8) + (b << 16) + (0xff << 24);

            let y_height = (v2.y - v0.y).abs();

            let sty = v2.y.min(v0.y);

            let i_normal_inv = 1.0 / y_height;

            let mut y_norm = 0.0;

            let frame_height_f32 = frame.height as f32;

            let mut i = 0.0;

            let (mut stx, mut end_x, mut y) = (0.0, 0.0, 0);

            loop {
                if i >= y_height {
                    break
                }

                if  (i + sty) <= 0.0 ||
                    (i + sty) >= frame_height_f32 {
                    i += 1.0;

                    continue;
                }

                stx = v0.x * (1.0 - y_norm) + v1.x * y_norm;
                end_x = v0.x * (1.0 - y_norm) + v2.x * y_norm;

                y = (i + sty) as u32;

                if let Some(pix) = frame.get_mut((stx as u32, y)) {
                    *pix = color;
                }
                if let Some(pix) = frame.get_mut((end_x as u32, y)) {
                    *pix = color;
                }

                if v0.y <= v2.y {
                    y_norm += i_normal_inv
                } else {
                    y_norm -= i_normal_inv;
                };

                i += 1.0;
            }

            frame.fill((stx as u32, y), (end_x as u32, y), color);
        }

        if (v1.y - v2.y).abs() < f32::EPSILON {
            flat_bottom(frame, color, up, v1, v2);
        } else if v1.y > v2.y {
            let x4 = Pos2D::new(v2.x, v1.y);

            // if downleft.x < downright.x {
            flat_bottom(frame, color, up, v1, x4);
            flat_bottom(frame, color, Pos2D::new(v1.x, v2.y), v1, x4);
            // Renderer::render_line(frame, color * Color::gray(2.0), 2.0, Pos2D::new(downleft.x, downright.y), downleft);
            // Renderer::render_line(frame, color * Color::gray(2.0), 2.0, Pos2D::new(downleft.x, downright.y), x4);
            // Renderer::render_line(frame, color * Color::gray(2.0), 2.0, downleft, x4);

            // } else {s
            //     flat_bottom(frame, color, up, x4, downleft);
            // }
        } else {
            let x4 = Pos2D::new(v1.x, v2.y);

            // if downleft.x < downright.x {
            flat_bottom(frame, color, up, v2, x4);
            flat_bottom(frame, Color::gray(0.5), Pos2D::new(v2.x, v1.y), v2, x4);
            // Renderer::render_line(frame, color * Color::gray(2.0), 2.0, Pos2D::new(downleft.x, downright.y), downleft);
            // Renderer::render_line(frame, color * Color::gray(2.0), 2.0, Pos2D::new(downleft.x, downright.y), x4);
            // Renderer::render_line(frame, color * Color::gray(2.0), 2.0, downleft, x4);

            // } else {s
            //     flat_bottom(frame, color, up, x4, downleft);
            // }
        }
    }

    fn render_square(frame: &mut Surface, color: Color, pos: Pos2D, size: Size) {
        let (r, g, b) = (
            (color.x * 255.0).clamp(0.0, 255.0) as u32,
            (color.y * 255.0).clamp(0.0, 255.0) as u32,
            (color.z * 255.0).clamp(0.0, 255.0) as u32,
        );

        if (size.x - 1.0).abs() < f32::EPSILON && (size.y - 1.0).abs() < f32::EPSILON {
            if let Some(pix) = frame.get_mut((pos.x as u32, pos.y as u32)) {
                *pix = r + (g << 8) + (b << 16) + (0xff << 24);
            }

            return
        }

        let (size_x, size_y) = (size.x as u32, size.y as u32);

        for y in 0..size_y {
            let pos_y = pos.y + y as f32;

            if pos.x >= frame.width as f32 {
                break
            }

            frame.fill(
                (pos.x as u32, pos_y as u32),
                (pos.x as u32 + size_x, pos_y as u32),
                r + (g << 8) + (b << 16) + (0xff << 24)
            );
        }
    }
}

use std::{mem, ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign}};

macro_rules! make_vec_type {
    (
        $name:ident => $( $coord:ident ),*
    ) => {
        #[derive(Copy, Clone, Debug, Default)]
        pub struct $name<T> {
            $( $coord: T ),*
        }

        impl<T> $name<T> where T: Copy {
            pub fn new($( $coord: T ),*) -> Self {
                Self {
                    $(
                        $coord,
                    )*
                }
            }

            #[allow(unused)]
            pub fn gray(val: T) -> Self {
                Self {
                    $(
                        $coord: val,
                    )*
                }
            }
        }

        impl<T> Add for $name<T> where T: Add<Output=T> {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    $(
                        $coord: self.$coord + rhs.$coord,
                    )*
                }
            }
        }

        impl<T> std::ops::Sub for $name<T> where T: Sub<Output=T> {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    $(
                        $coord: self.$coord - rhs.$coord,
                    )*
                }
            }
        }

        impl<T> std::ops::Mul for $name<T> where T: Mul<Output=T> {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                Self {
                    $(
                        $coord: self.$coord * rhs.$coord,
                    )*
                }
            }
        }

        impl<T> std::ops::AddAssign for $name<T> where T: AddAssign {
            fn add_assign(&mut self, rhs: Self) {
                $(
                    self.$coord += rhs.$coord;
                )*
            }
        }

        impl<T> std::ops::SubAssign for $name<T> where T: SubAssign {
            fn sub_assign(&mut self, rhs: Self) {
                $(
                    self.$coord -= rhs.$coord;
                )*
            }
        }

        impl<T> std::ops::MulAssign for $name<T> where T: MulAssign {
            fn mul_assign(&mut self, rhs: Self) {
                $(
                    self.$coord *= rhs.$coord;
                )*
            }
        }
    };
}

make_vec_type!(Pos2DTyped => x, y);
make_vec_type!(Pos3DTyped => x, y, z);

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

pub type Pos2D = Pos2DTyped<f32>;
pub type Pos3D = Pos3DTyped<f32>;

pub type Size = Pos2D;
pub type Color = Pos3D;

#[derive(Default, Debug, Copy, Clone)]
pub struct WorldPos {
    begin: Pos2D,
}

#[derive(Debug, Clone)]
pub enum ObjectVertices {
    Line {
        begin: Pos2D, end: Pos2D,
    },
    Square {
        pos: Pos2D, size: Size,
    },
    Triangle {
        up: Pos2D,
        left: Pos2D,
        right: Pos2D,
    },
}

impl std::ops::AddAssign<WorldPos> for ObjectVertices {
    fn add_assign(&mut self, rhs: WorldPos) {
        match self {
            ObjectVertices::Line { begin, end } => {
                *begin += rhs.begin;
                *end   += rhs.begin;
            }
            ObjectVertices::Square { pos, size: _ } => {
                *pos += rhs.begin;
            }
            ObjectVertices::Triangle { up, left, right } => {
                *up += rhs.begin;
                *left += rhs.begin;
                *right += rhs.begin;
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

    pub fn triangle(&mut self, color: Color, up: Pos2D, left: Pos2D, right: Pos2D) {
        self.objects.push(
            Object {
                local_pos: self.world_pos,
                color,
                vertices: ObjectVertices::Triangle { up, left, right }
            }
        );
    }
    pub fn wf_triangle(&mut self, color: Color, up: Pos2D, left: Pos2D, right: Pos2D) {
        self.line(color * Pos3D::gray(0.3), up, left);
        self.line(color * Pos3D::gray(0.6), up, right);
        self.line(color * Pos3D::gray(0.9), left, right);
    }

    pub fn line(&mut self, color: Color, begin: Pos2D, end: Pos2D) {
        self.objects.push(
            Object {
                local_pos: self.world_pos,
                color,
                vertices: ObjectVertices::Line { begin, end }
            }
        );
    }

    pub fn square(&mut self, color: Color, pos: Pos2D, size: Size) {
        self.objects.push(
            Object {
                local_pos: self.world_pos,
                color,
                vertices: ObjectVertices::Square { pos, size }
            }
        );
    }
}

#[derive(Debug)]
pub struct Surface<'a> {
    pub frame: &'a mut [u32],
    pub width: u32,
    pub height: u32,
}

impl<'a> Surface<'a> {
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

    fn fill(&mut self, color: u32) {
        self.frame.fill(color);
    }
}

#[derive(Debug)]
pub struct Renderer();

impl Renderer {
    pub fn render(frame: &mut Surface, objects: Vec<Object>) {
        for Object { local_pos, color, mut vertices } in objects {
            vertices += local_pos;

            match vertices {
                ObjectVertices::Line { begin, end } => {
                    Self::render_line(frame, color, begin, end);
                }
                ObjectVertices::Square { pos, size } => {
                    Self::render_square(frame, color, pos, size);
                }
                ObjectVertices::Triangle { up, left, right } => {
                    Self::render_triangle(frame, color, up, left, right);
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

        frame.fill(r + (g << 8) + (b << 16) + (0xff << 24));
    }

    fn render_line(frame: &mut Surface, color: Color, start: Pos2D, end: Pos2D) {
        let mut pos = start;

        let d = end - pos;

        let sx = (d.x / d.y).abs();

        let sx = if sx.is_nan() {
            0.0
        } else {
            sx
        };

        let sy = (d.y / d.x).abs();

        let sy = if sy.is_nan() {
            0.0
        } else {
            sy
        };

        loop {
            let d = end - pos;

            // println!("d = {d:?}");

            let (dx, dy) = if sx < sy {
                (
                    d.x.clamp(-sx, sx),
                    d.y.clamp(-1.0, 1.0)
                )
            } else {
                (
                    d.x.clamp(-1.0, 1.0),
                    d.y.clamp(-sy, sy)
                )
            };

            if dx == 0.0 && dy == 0.0 {
                break
            }

            Self::render_square(frame, color, pos, Pos2D::new(5.0_f32, 5.0_f32));

            pos.x += dx; pos.y += dy;
        }
    }

    fn render_triangle(frame: &mut Surface, color: Color, up: Pos2D, left: Pos2D, right: Pos2D) {
        fn flat_bottom(frame: &mut Surface, color: Color, up: Pos2D, left: Pos2D, right: Pos2D) {
            let (r, g, b) = (
                (color.x * 255.0).clamp(0.0, 255.0) as u32,
                (color.y * 255.0).clamp(0.0, 255.0) as u32,
                (color.z * 255.0).clamp(0.0, 255.0) as u32,
            );

            let y_height = right.y - up.y;

            for i in 0..(y_height as u32) {
                let y_norm = (i as f32) / y_height;

                let stx = (up.x * (1.0 - y_norm) + left.x * y_norm) as u32;
                let end_x = (up.x * (1.0 - y_norm) + right.x * y_norm) as u32;

                for x in stx..end_x {
                    if let Some(pix) = frame.get_mut((x, i + up.y as u32)) {
                        *pix = r + (g << 8) + (b << 16) + (0xff << 24);
                    }
                }
            }
        }

        flat_bottom(frame, color, up, left, right);
    }

    fn render_square(frame: &mut Surface, color: Color, pos: Pos2D, size: Size) {
        let (r, g, b) = (
            (color.x * 255.0).clamp(0.0, 255.0) as u32,
            (color.y * 255.0).clamp(0.0, 255.0) as u32,
            (color.z * 255.0).clamp(0.0, 255.0) as u32,
        );

        let (size_x, size_y) = (size.x as u32, size.y as u32);

        for y in 0..size_y {
            for x in 0..size_x {
                let pos_x = pos.x as u32 + x;
                let pos_y = pos.y as u32 + y;

                if pos_x >= frame.width {
                    break
                }

                if let Some(pix) = frame.get_mut((pos_x, pos_y)) {
                    *pix = r + (g << 8) + (b << 16) + (0xff << 24);
                }
            }
        }
    }
}

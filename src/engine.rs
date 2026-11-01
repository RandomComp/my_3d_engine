use std::{mem, ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign}};

use crate::cast_u32_to_u8;

macro_rules! make_vec_type {
    (
        $name:ident<$t:ty> => $( $coord:ident ),*
    ) => {
        #[derive(Copy, Clone, Debug, Default)]
        pub struct $name {
            $( pub $coord: $t ),*
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
            pub fn point(val: $t) -> Self {
                Self {
                    $(
                        $coord: val,
                    )*
                }
            }
        }
    };
}

macro_rules! impl_ops_for_vec_type {
    (
        $name:ident<$t:ty> => $( $coord:ident ),*
    ) => {
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

        impl Mul<$t> for $name {
            type Output = Self;

            fn mul(self, rhs: $t) -> Self::Output {
                Self {
                    $(
                        $coord: self.$coord * rhs,
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

make_vec_type!(ScreenPos<i32> => x, y);
impl_ops_for_vec_type!(ScreenPos<i32> => x, y);

pub type ScreenSize = ScreenPos;

make_vec_type!(Pos2D<f32> => x, y);
impl_ops_for_vec_type!(Pos2D<f32> => x, y);

impl Pos2D {
    #[allow(unused)]
    pub fn distance_to(self, rhs: Self) -> f32 {
        (
            (rhs.x - self.x).powi(2) +
            (rhs.y - self.y).powi(2)
        ).sqrt()
    }
}

make_vec_type!(Pos3D<f32> => x, y, z);
impl_ops_for_vec_type!(Pos3D<f32> => x, y, z);

make_vec_type!(BoxCollider2D<Pos2D> => v0, v1);

impl BoxCollider2D {
    pub fn bounds_with(&self, rhs: Self) -> bool {
        rhs.v0.x >= self.v0.x && rhs.v0.y >= self.v0.y &&
        rhs.v1.x <= self.v1.x && rhs.v1.y <= self.v1.y
    }

    pub fn contains(&self, rhs: Self) -> bool {
        rhs.v0.x >= self.v0.x && rhs.v0.y >= self.v0.y &&
        rhs.v1.x <= self.v1.x && rhs.v1.y <= self.v1.y
    }
}

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
pub struct Matrix2x2 {
    m11: f32, m12: f32,
    m21: f32, m22: f32,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct WorldPos {
    begin: Pos2D
}

impl Mul for Matrix2x2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ObjectVertices {
    Line {
        begin: Pos2D, end: Pos2D, width: f32,
    },
    Triangle {
        v0: Pos2D, v1: Pos2D, v2: Pos2D,
    },
}

impl ObjectVertices {
    pub fn bounding_box(&self) -> BoxCollider2D {
        match *self {
            ObjectVertices::Line { begin, end, width } => {
                BoxCollider2D {
                    v0: begin,
                    v1: Pos2D::new(end.x + width, end.y + width),
                }
            }
            ObjectVertices::Triangle { v0, v1, v2 } => {
                let (leftx, lefty) = (v0.x.min(v1.x).min(v2.x), v0.y.min(v1.y).min(v2.y));
                let (rightx, righty) = (v0.x.max(v1.x).max(v2.x), v0.y.max(v1.y).max(v2.y));

                BoxCollider2D {
                    v0: Pos2D::new(leftx, lefty),
                    v1: Pos2D::new(rightx, righty),
                }
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
    pub fn bounding_box(&self) -> BoxCollider2D {
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
    #[must_use = "if you not use taked objects, objects will be dropped"]
    pub fn take_objects(&mut self) -> Vec<Object> {
        mem::take(&mut self.objects)
    }

    #[must_use = "if you not use taked matrix, mtrix will be dropped"]
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
        self.wf_triangle(color * 0.3, downleft, upleft, upright);
        self.wf_triangle(color * 0.8, upright, downleft, downright);
    }

    pub fn square(&mut self, color: Color, upleft: Pos2D, upright: Pos2D, downleft: Pos2D, downright: Pos2D) {
        self.triangle(color * 0.3, downleft, upleft, upright);
        self.triangle(color * 0.8, upright, downleft, downright);
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
        let (r, g, b) = ((color & 0xff), ((color >> 8) & 0xff), ((color >> 16) & 0xff));

        if  (color & 0x00ffffff) == 0x00000000 ||
            (color & 0x00ffffff) == 0x00ffffff {
            let frame = cast_u32_to_u8(self.frame);

            let gray = (r + g + b) / 3;

            frame.fill(gray as u8);
        } else {
            self.frame.fill(color);
        }
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
        let frame_box = BoxCollider2D { v0: Pos2D::point(0.0), v1: Pos2D::new(frame.width as f32, frame.height as f32) };

        for Object { local_pos, color, mut vertices } in objects {
            vertices += local_pos;

            if vertices.bounding_box().bounds_with(frame_box) {
                return
            }

            match vertices {
                ObjectVertices::Line { begin, end, width } => {
                    // Self::render_line_bresenham(frame, color, width as i32,
                    //     ScreenPos::new(begin.x as i32, begin.y as i32),
                    //     ScreenPos::new(end.x as i32, end.y as i32)
                    // );

                    Self::render_line_dda(frame, color, width as i32,
                        begin,
                        end
                    );
                }
                ObjectVertices::Triangle { v0, v1, v2 } => {
                    Self::render_triangle_barycentric(frame, color, v0, v1, v2);
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

    fn render_line_dda(frame: &mut Surface, color: Color, width: i32, start: Pos2D, end: Pos2D) {
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

            if dx.abs() < 0.01 && dy.abs() < 0.01 {
                break
            }

            Self::render_square(frame, color, ScreenPos::new(pos.x as i32, pos.y as i32), ScreenPos::new(width, width));

            pos.x += dx; pos.y += dy;
        }
    }

    fn render_line_bresenham(frame: &mut Surface, color: Color, width: i32, start: ScreenPos, end: ScreenPos) {
        let (r, g, b) = (
            (color.x * 255.0).clamp(0.0, 255.0) as u32,
            (color.y * 255.0).clamp(0.0, 255.0) as u32,
            (color.z * 255.0).clamp(0.0, 255.0) as u32,
        );

        // let (start, end) = (end, start);

        // let (start, end) = if start.x >= end.x && start.y >= end.y {
        //     (end, start)
        // } else {
        //     (start, end)
        // };

        // println!("({start:?}, {end:?})");

        let delta_x = (end.x - start.x).abs();
        let delta_y = (end.y - start.y).abs();

        let (sx, sy) = (
            if end.x < start.x { -1 } else { 1 },
            if end.y < start.y { -1 } else { 1 }
        );

        // println!("({sx}, {sy})");

        if delta_y <= delta_x {
            let mut pos_y = start.y;

            // println!("start = {start:?}, end = {end:?}");

            let mut error = 2 * delta_y - delta_x;

            for x in 0..delta_x {
                if error < 0 {
                    error += 2 * delta_y;
                } else {
                    pos_y += sy;
                    error += 2 * delta_y - 2 * delta_x;
                }

                if let Some(pix) = frame.get_mut(((start.x + x) as u32, pos_y as u32)) {
                    *pix = r + (g << 8) + (b << 16) + (0xff << 24);
                }
            }
        } else {
            let mut pos_x = start.x;

            let mut error = 2 * delta_x - delta_y;

            for y in 0..delta_y {
                if error < 0 {
                    error += 2 * delta_x;
                } else {
                    pos_x += sx;
                    error += 2 * delta_x - 2 * delta_y;
                }

                if let Some(pix) = frame.get_mut((pos_x as u32, (start.y + y) as u32)) {
                    *pix = r + (g << 8) + (b << 16) + (0xff << 24);
                }
            }
        }
    }

    fn triangle_flat_bottom(frame: &mut Surface, color: Color, v0: Pos2D, v1: Pos2D, v2: Pos2D) {
        if (v0.y - v2.y).abs() < f32::EPSILON {
            Self::render_line_dda(frame, color, 1, v0, v2);

            return
        }

        let y_height = (v2.y - v0.y).ceil() as i32;

        let (v0, v1, v2) = (
            ScreenPos::new(v0.x as i32, v0.y as i32),
            ScreenPos::new(v1.x as i32, v1.y as i32),
            ScreenPos::new(v2.x as i32, v2.y as i32),
        );

        let mut i = if v0.y < 0 {-v0.y} else { 0 };

        let (r, g, b) = (
            (color.x * 255.0).clamp(0.0, 255.0) as u32,
            (color.y * 255.0).clamp(0.0, 255.0) as u32,
            (color.z * 255.0).clamp(0.0, 255.0) as u32,
        );

        let color = r + (g << 8) + (b << 16) + (0xff << 24);

        while i <= y_height {
            if (i + v0.y) >= frame.height as i32 {
                i += 1;

                continue;
            }

            let stx = (v0.x * (y_height - i) + v1.x * i) / y_height;
            let end_x = (v0.x * (y_height - i) + v2.x * i) / y_height;

            let y = (i + v0.y) as u32;

            frame.fill((stx as u32, y), (end_x as u32, y), color);

            i += 1;
        }
    }

    fn triangle_flat_top(frame: &mut Surface, color: Color, v0: Pos2D, v1: Pos2D, v2: Pos2D) {
        if (v0.y - v2.y).abs() < f32::EPSILON {
            Self::render_line_dda(frame, color, 1, v0, v2);

            return
        }

        let y_height = (v2.y - v0.y).ceil() as i32;

        let (r, g, b) = (
            (color.x * 255.0).clamp(0.0, 255.0) as u32,
            (color.y * 255.0).clamp(0.0, 255.0) as u32,
            (color.z * 255.0).clamp(0.0, 255.0) as u32,
        );

        let color = r + (g << 8) + (b << 16) + (0xff << 24);

        let mut i = if v0.y < 0.0 {-v0.y as i32} else { 0 };

        let (v0, v1, v2) = (
            ScreenPos::new(v0.x as i32, v0.y as i32),
            ScreenPos::new(v1.x as i32, v1.y as i32),
            ScreenPos::new(v2.x as i32, v2.y as i32),
        );

        // println!("(v0, v1, v2) = ({v0:?}, {v1:?}, {v2:?})");

        while i <= y_height {
            if (i + v0.y) >= frame.height as i32 {
                i += 1;

                continue;
            }

            let stx = (v1.x * (y_height - i) + v2.x * i) / y_height;
            let end_x = (v0.x * (y_height - i) + v2.x * i) / y_height;

            let y = (i + v0.y) as u32;

            frame.fill((stx as u32, y), (end_x as u32, y), color);

            i += 1;
        }
    }

    fn render_triangle(frame: &mut Surface, color: Color, mut v0: Pos2D, v1: Pos2D, v2: Pos2D) {
        let mut sorted = [v0, v1, v2];
        sorted.sort_by(|&a, &b|
            a.y.total_cmp(&b.y)
        );

        let [v0, v1, v2] = sorted;

        if (v0.y - v2.y).abs() < f32::EPSILON {
            let mut sorted = [v0, v2];
            sorted.sort_by(|&a, &b|
                a.x.total_cmp(&b.x)
            );

            let [v0, v2] = sorted;

            Self::render_line_dda(frame, color, 1, v0, v2);

            return
        }

        if v0.y != v1.y && v1.y != v2.y {
            let v3 = Pos2D::new(v0.x + ((v1.y - v0.y) * (v2.x - v0.x)) / (v2.y - v0.y), v1.y);

            Self::triangle_flat_bottom(frame, color, v0, v1, v3);
            Self::triangle_flat_top(frame, color, v1, v3, v2);
        } else if (v1.y - v2.y).abs() < f32::EPSILON {
            Self::triangle_flat_bottom(frame, color, v0, v1, v2);
        } else {
            Self::triangle_flat_top(frame, color, v0, v1, v2);
        }
    }

    fn render_triangle_barycentric(frame: &mut Surface, color: Color, v1: Pos2D, v2: Pos2D, v3: Pos2D) {
        let (r, g, b) = (
            (color.x * 255.0).clamp(0.0, 255.0) as u32,
            (color.y * 255.0).clamp(0.0, 255.0) as u32,
            (color.z * 255.0).clamp(0.0, 255.0) as u32,
        );

        let bounding_box = ObjectVertices::Triangle { v0: v1, v1: v2, v2: v3 }.bounding_box();

        let (stx, sty, endx, endy) = (
            bounding_box.v0.x.round() as u32, bounding_box.v0.y.round() as u32,
            bounding_box.v1.x.round() as u32, bounding_box.v1.y.round() as u32,
        );

        let d = (v2.y - v3.y) * (v1.x - v3.x) + (v3.x - v2.x) * (v1.y - v3.y);

        for it_y in sty..endy {
            let y = it_y as f32;

            for it_x in stx..endx {
                let x = it_x as f32;

                let h1 = ((v2.y - v3.y) * (x - v3.x) + (v3.x - v2.x) * (y - v3.y)) / d;
                let h2 = ((v3.y - v1.y) * (x - v3.x) + (v1.x - v3.x) * (y - v3.y)) / d;
                let h3 = 1.0 - h1 - h2;

                if h1 >= 0.0 && h2 >= 0.0 && h3 >= 0.0 {
                    if let Some(pix) = frame.get_mut((it_x, it_y)) {
                        *pix = r + (g << 8) + (b << 16) + (0xff << 24);
                    }
                }
            }
        }
    }

    fn render_square(frame: &mut Surface, color: Color, pos: ScreenPos, size: ScreenSize) {
        let (r, g, b) = (
            (color.x * 255.0).clamp(0.0, 255.0) as u32,
            (color.y * 255.0).clamp(0.0, 255.0) as u32,
            (color.z * 255.0).clamp(0.0, 255.0) as u32,
        );

        if size.x == 1 && size.y == 1 {
            if let Some(pix) = frame.get_mut((pos.x as u32, pos.y as u32)) {
                *pix = r + (g << 8) + (b << 16) + (0xff << 24);
            }

            return
        }

        for y in 0..(size.y as u32) {
            let pos_y = pos.y as u32 + y;

            if pos.x as u32 >= frame.width {
                break
            }

            frame.fill(
                (pos.x as u32, pos_y),
                (pos.x as u32 + size.x as u32, pos_y),
                r + (g << 8) + (b << 16) + (0xff << 24)
            );
        }
    }
}

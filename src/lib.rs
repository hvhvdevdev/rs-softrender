pub mod rasterizer;
pub mod serializer;

pub struct PixelRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

pub struct Image {
    pub width: u16,
    pub height: u16,
    pub data: Vec<PixelRGB>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

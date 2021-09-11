pub mod rasterizer;
pub mod serializer;

pub struct Pixel {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub struct Image {
    pub width: u16,
    pub height: u16,
    pub data: Vec<Pixel>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

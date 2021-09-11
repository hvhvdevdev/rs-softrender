fn ppm(image: crate::Image) -> Vec<u8> {
    let mut data = format!("P3\n{} {}\n", image.width, image.height);

    for pixel in image.data {
        data = format!("{}\n{} {} {}", data, pixel.r, pixel.g, pixel.b)
    }

    data.into_bytes()
}

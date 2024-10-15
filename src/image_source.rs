pub enum ImgFmt {
    GRAYSCALE,
    RGB,
    BGR,
}

pub struct Image<T> {
    height: u32,
    width: u32,
    stride: u32,
    data: Vec<T>,
    fmt: ImgFmt,
}

impl<T> Image<T> {
    pub fn get_dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn get_stride(&self) -> u32 {
        self.stride
    }

    pub fn get_format(&self) -> &ImgFmt {
        &self.fmt
    }
}

pub trait ImageSrc<T> {
    fn get_image() -> Image<T>;
}

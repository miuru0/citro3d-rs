use crate::{Point, Size, sprite::SpriteSheet};

pub trait C2DImage {
    fn draw(&self, position: Point, scale: Size);
}


pub struct SheetImage {
    pub(crate) raw_image: citro2d_sys::C2D_Image,
    pub(crate) _sheet: SpriteSheet,
}

impl C2DImage for SheetImage {
    fn draw(&self, position: Point, scale: Size) {
        unsafe {citro2d_sys::C2D_DrawImageAt(self.raw_image, position.x, position.y, position.z, 0 as *const _, scale.width, scale.height)};
    }
}
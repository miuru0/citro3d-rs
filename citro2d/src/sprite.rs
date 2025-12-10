use std::marker::PhantomData;

use ctru::services::romfs::RomFS;

use crate::base::SheetImage;


pub struct SpriteSheet {
    pub raw_sprite_sheet: citro2d_sys::C2D_SpriteSheet,
}

impl SpriteSheet {
    pub fn load(filename: &str) -> Result<Self, crate::Error> {
        let cstr = std::ffi::CString::new(filename).unwrap();
        let raw_sprite_sheet = unsafe {citro2d_sys::C2D_SpriteSheetLoad(cstr.as_ptr()) };
        if raw_sprite_sheet.is_null() {
            return Err(crate::Error::ErrorLoading)
        }
        Ok(Self {
            raw_sprite_sheet
        })
    }

    pub fn get(&self, index: usize) -> Sprite<'_> {
        let mut raw_sprite = unsafe { std::mem::zeroed::<citro2d_sys::C2D_Sprite>() };
        unsafe { citro2d_sys::C2D_SpriteFromSheet(&mut raw_sprite as *mut _, self.raw_sprite_sheet, index) };
        Sprite { raw_sprite, _marker: PhantomData }
    }

    

    pub fn get_image(&self, index: usize) -> SheetImage<'_> {
        let raw_image = unsafe { citro2d_sys::C2D_SpriteSheetGetImage(self.raw_sprite_sheet, index) };
        SheetImage {
            raw_image,
            _marker: PhantomData,
        }
    }
}

impl Drop for SpriteSheet {
    fn drop(&mut self) {
        unsafe { citro2d_sys::C2D_SpriteSheetFree(self.raw_sprite_sheet) };
    }
}


pub struct Sprite<'sheet> {
    raw_sprite: citro2d_sys::C2D_Sprite,
    _marker: std::marker::PhantomData<&'sheet SpriteSheet>,
}

impl<'sheet> Sprite<'sheet> {
    pub fn render(&self) {
        unsafe { citro2d_sys::C2D_DrawSprite(&self.raw_sprite as *const _) };
    }

    pub fn rotate(&mut self, radians: f32) {
        self.raw_sprite.params.angle += radians;
    }
    
    pub fn translate(&mut self, x: f32, y: f32) {
        self.raw_sprite.params.pos.x += x;
        self.raw_sprite.params.pos.y += y;
    }

    pub fn scale(&mut self, x: f32, y: f32) {
        self.raw_sprite.params.pos.w *= x;
        self.raw_sprite.params.pos.h *= y;
        self.raw_sprite.params.center.x *= x;
        self.raw_sprite.params.center.y *= y;
    }

    pub fn set_scale(&mut self, x: f32, y: f32) {
        unsafe { citro2d_sys::C2D_SpriteSetScale(&mut self.raw_sprite as *mut _, x, y) };
    }

    pub fn set_rotation(&mut self, radians: f32) {
        self.raw_sprite.params.angle = radians;
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.raw_sprite.params.pos.x = x;
        self.raw_sprite.params.pos.y = y;
    }

    pub fn set_depth(&mut self, depth: f32) {
        self.raw_sprite.params.depth = depth;
    }

}
use std::{marker::PhantomData, rc::Rc};

use crate::base::SheetImage;


pub struct SpriteSheetData {
    pub raw_sprite_sheet: citro2d_sys::C2D_SpriteSheet,
    count: usize,
}

impl Drop for SpriteSheetData {
    fn drop(&mut self) {
        unsafe { citro2d_sys::C2D_SpriteSheetFree(self.raw_sprite_sheet) };
    }
}


#[derive(Clone)]
pub struct SpriteSheet {
    data: Rc<SpriteSheetData>,
}

impl SpriteSheet {
    pub fn load(filename: &str) -> Result<Self, crate::Error> {
        let cstr = std::ffi::CString::new(filename).unwrap();
        let raw_sprite_sheet = unsafe {citro2d_sys::C2D_SpriteSheetLoad(cstr.as_ptr()) };
        if raw_sprite_sheet.is_null() {
            return Err(crate::Error::ErrorLoading)
        }

        let count = unsafe { citro2d_sys::C2D_SpriteSheetCount(raw_sprite_sheet) };
        Ok(Self { data: Rc::new(SpriteSheetData {
            raw_sprite_sheet,
            count,
        })})
    }

    pub fn len(&self) -> usize {
        self.data.count
    }

    pub fn get(&self, index: usize) -> Option<Sprite> {
        if index >= self.data.count {
            return None
        }

        let mut raw_sprite = std::mem::MaybeUninit::<citro2d_sys::C2D_Sprite>::uninit();
        unsafe { 
            citro2d_sys::C2D_SpriteFromSheet(raw_sprite.as_mut_ptr(), self.data.raw_sprite_sheet, index);
            Some(Sprite { 
                raw_sprite: raw_sprite.assume_init(), 
                _sheet: self.clone() 
            })
        }
    }
    
    pub fn get_image(&self, index: usize) -> SheetImage {
        let raw_image = unsafe { citro2d_sys::C2D_SpriteSheetGetImage(self.data.raw_sprite_sheet, index) };
       SheetImage {
            raw_image,
            _sheet: self.clone(),
        }
    }
}


#[derive(Clone)]
pub struct Sprite {
    raw_sprite: citro2d_sys::C2D_Sprite,
    _sheet: SpriteSheet,
}

impl Sprite {
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
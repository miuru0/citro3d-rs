use crate::{Point, render::Color};
use bitflags::bitflags;


pub struct C2DText {
    raw_text: citro2d_sys::C2D_Text,
    raw_text_buf: citro2d_sys::C2D_TextBuf,
    max_glyphs: usize,
    pub flags: C2DTextFlags,
    pub position: Point,
    pub scale_x: f32,
    pub scale_y: f32,
    pub color: Option<Color>,
}




bitflags! {
    pub struct C2DTextFlags: u32 {
        const AT_BASELINE      = 1 << 0; // BIT(0)
        const WITH_COLOR       = 1 << 1; // BIT(1)

        const ALIGN_LEFT       = 0 << 2; // 0
        const ALIGN_RIGHT      = 1 << 2; // 4
        const ALIGN_CENTER     = 2 << 2; // 8
        const ALIGN_JUSTIFIED  = 3 << 2; // 12
        const ALIGN_MASK       = 3 << 2; // 12

        const WORD_WRAP        = 1 << 4; // BIT(4)
    }
}

impl C2DText {
    pub fn new(max_glyphs: usize, flags: C2DTextFlags, position: Point, scale_x: f32, scale_y: f32, color: Option<Color>) -> Result<Self, crate::Error> {
        let mut raw_text = std::mem::MaybeUninit::<citro2d_sys::C2D_Text>::uninit();
        unsafe {
            let raw_text_buf = citro2d_sys::C2D_TextBufNew(max_glyphs);
        
            let result = citro2d_sys::C2D_TextParse(
                raw_text.as_mut_ptr(),
                raw_text_buf,
                b"\0".as_ptr()
            );
            if result.is_null() {
                return Err(crate::Error::FailedToParse)
            }

            citro2d_sys::C2D_TextOptimize(raw_text.as_mut_ptr());
        
            Ok(Self { 
                raw_text: raw_text.assume_init(), 
                raw_text_buf, 
                max_glyphs,
                position,
                flags,
                scale_x,
                scale_y,
                color,
            })
        }
    }

    pub fn clear(&mut self) {
        unsafe {citro2d_sys::C2D_TextBufClear(self.raw_text_buf)}
    }

    pub fn resize(&mut self, max_glyphs: usize) {
        unsafe {
            citro2d_sys::C2D_TextBufResize(self.raw_text_buf, max_glyphs);
        }
        self.max_glyphs = max_glyphs
    }

    pub fn get_num_glyphs(&self) -> usize {
        unsafe {
            citro2d_sys::C2D_TextBufGetNumGlyphs(self.raw_text_buf)
        }
    }

    pub fn append_text(&mut self, s: &str) -> Result<(), crate::Error> {
        let cstr = std::ffi::CString::new(s).unwrap();
        unsafe {
            let result = citro2d_sys::C2D_TextParse(
                &mut self.raw_text,
                self.raw_text_buf,
                cstr.as_ptr()
            );

            if result.is_null() {
                return Err(crate::Error::FailedToParse)
            } else {
                if *result != 0 {
                    return Err(crate::Error::TooManyGlyphs)
                }
            }
            citro2d_sys::C2D_TextOptimize(&self.raw_text);
        }
        Ok(())
    }

    pub fn set_text(&mut self, s: &str) -> Result<(), crate::Error> {
        self.clear();
        self.append_text(s)
    }

    pub(crate) fn render(&self) {
        unsafe {
            if let Some(color) = self.color {
                citro2d_sys::C2D_DrawText(
                    &self.raw_text, 
                    self.flags.bits(), 
                    self.position.x, 
                    self.position.y, 
                    self.position.z, 
                    self.scale_x, 
                    self.scale_y,
                    color
                )
            } else {
                citro2d_sys::C2D_DrawText(
                    &self.raw_text, 
                    self.flags.bits(), 
                    self.position.x, 
                    self.position.y, 
                    self.position.z, 
                    self.scale_x, 
                    self.scale_y
                )
            }
            
        }
    }
}

impl Drop for C2DText {
    fn drop(&mut self) {
        unsafe  {
            citro2d_sys::C2D_TextBufDelete(self.raw_text_buf);
        }
    }
}
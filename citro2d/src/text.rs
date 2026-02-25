use crate::{Point, Size, render::Color};

#[derive(Debug, Clone, Copy)]
pub enum C2DTextAlignment {
    Left,
    Right,
    Center,
    Justified,
}

impl Default for C2DTextAlignment {
    fn default() -> Self {
        C2DTextAlignment::Left
    }
}

impl C2DTextAlignment {
    pub fn bits(&self) -> u32 {
        match self {
            C2DTextAlignment::Left => 0 << 2,
            C2DTextAlignment::Right => 1 << 2,
            C2DTextAlignment::Center => 2 << 2,
            C2DTextAlignment::Justified => 3 << 2,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct C2DTextFlags {
    pub baseline: bool,
    pub word_wrap: Option<f64>,
    pub alignment: C2DTextAlignment,
}

impl C2DTextFlags {
    pub fn new(baseline: bool, word_wrap: Option<f64>, alignment: C2DTextAlignment) -> Self {
        C2DTextFlags { baseline, word_wrap, alignment }
    }

    pub fn alignment(&mut self, alignment: C2DTextAlignment) -> &mut Self {
        self.alignment = alignment;
        self
    }
    
    pub fn at_baseline(&mut self, baseline: bool) -> &mut Self {
        self.baseline = baseline;
        self
    }

    pub fn word_wrap(&mut self, width: Option<f64>) -> &mut Self {
        self.word_wrap = width;
        self
    }

    pub fn bits(&self) -> u32 {
        let mut bits = 0;
        if self.baseline {
            bits |= 1 << 0;
        }
        if self.word_wrap.is_some() {
            bits |= 1 << 4;
        }
        bits |= self.alignment.bits();
        bits
    }
}


pub struct C2DText {
    raw_text: citro2d_sys::C2D_Text,
    raw_text_buf: citro2d_sys::C2D_TextBuf,
    text_string: String,
    max_glyphs: usize,
    pub flags: C2DTextFlags,
    pub position: Point,
    pub size: Size,
    pub color: Option<Color>,
}

impl C2DText {
    pub fn new(max_glyphs: usize, position: Point, size: Size, color: Option<Color>, flags: C2DTextFlags) -> Result<Self, crate::Error> {
        let mut raw_text = std::mem::MaybeUninit::<citro2d_sys::C2D_Text>::uninit();
        unsafe {
            let raw_text_buf = citro2d_sys::C2D_TextBufNew(max_glyphs);
        

            // Only way I know of initializing C2D_Text and associating it with the TextBuf
            let result = citro2d_sys::C2D_TextParse(
                raw_text.as_mut_ptr(),
                raw_text_buf,
                b"\0".as_ptr()
            );
            if result.is_null() {
                citro2d_sys::C2D_TextBufDelete(raw_text_buf);
                return Err(crate::Error::FailedToParse)
            }

            citro2d_sys::C2D_TextOptimize(raw_text.as_mut_ptr());
        
            Ok(Self { 
                raw_text: raw_text.assume_init(), 
                raw_text_buf, 
                max_glyphs,
                text_string: String::new(),
                position,
                flags,
                size,
                color,
            })
        }
    }

    pub fn clear(&mut self) {
        //self.raw_text.yScale = 0.;
        self.text_string.clear();
        unsafe {citro2d_sys::C2D_TextBufClear(self.raw_text_buf)}
    }

    pub fn resize(&mut self, max_glyphs: usize) {
        unsafe {
            self.raw_text_buf = citro2d_sys::C2D_TextBufResize(self.raw_text_buf, max_glyphs);
        }
        self.max_glyphs = max_glyphs
    }

    pub fn get_num_glyphs(&self) -> usize {
        unsafe {
            citro2d_sys::C2D_TextBufGetNumGlyphs(self.raw_text_buf)
        }
    }

    pub fn layout_text(&mut self) -> (f32, f32) {
        unsafe {
            let flags = self.flags.bits();
            let w = self.size.width;
            let h = self.size.height;

            if let Some(color) = self.color {
                if let Some(wrap_width) = self.flags.word_wrap {
                    unsafe { citro2d_sys::C2D_LayoutText(
                        &mut self.raw_text, 
                        flags | (1 << 1), // with color flag 
                        w, h,
                        color.inner,
                        wrap_width,
                    ) }
                } else {
                    unsafe { citro2d_sys::C2D_LayoutText(
                        &mut self.raw_text, 
                        flags | (1 << 1), // with color flag 
                        w, h,
                        color.inner,
                    ) }
                }
            } else {
                if let Some(wrap_width) = self.flags.word_wrap {
                    unsafe { citro2d_sys::C2D_LayoutText(
                        &mut self.raw_text, 
                        flags,
                        w, h,
                        wrap_width,
                    ) }
                } else {
                    unsafe { citro2d_sys::C2D_LayoutText(
                        &mut self.raw_text, 
                        flags,
                        w, h,
                    ) }
                }
            }
        }

        let mut width = 0.;
        let mut height = 0.;
        unsafe {
            citro2d_sys::C2D_TextGetDimensions(&self.raw_text, self.size.width, self.size.height, &mut width, &mut height);
        }
        (width, height)
    }

    pub fn append_text(&mut self, s: &str) -> Result<(), crate::Error> {
        //self.raw_text.yScale = 0.;
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

        self.text_string.push_str(s);
        Ok(())
    }

    pub fn set_text(&mut self, s: &str) -> Result<(), crate::Error> {
        self.clear();
        self.append_text(s)
    }

    pub fn get_text(&self) -> &str {
        return &self.text_string
    }

    pub(crate) fn render(&self) {
        let flags = self.flags.bits();
        let x = self.position.x;
        let y = self.position.y;
        let z = self.position.z;
        let w = self.size.width;
        let h = self.size.height;

        if let Some(color) = self.color {
            if let Some(wrap_width) = self.flags.word_wrap {
                unsafe { citro2d_sys::C2D_DrawText(
                    &self.raw_text, 
                    flags | (1 << 1), // with color flag 
                    x, y, z, w, h,
                    color.inner,
                    wrap_width,
                ) }
            } else {
                unsafe { citro2d_sys::C2D_DrawText(
                    &self.raw_text, 
                    flags | (1 << 1), // with color flag 
                    x, y, z, w, h,
                    color.inner,
                ) }
            }
        } else {
            if let Some(wrap_width) = self.flags.word_wrap {
                unsafe { citro2d_sys::C2D_DrawText(
                    &self.raw_text, 
                    flags,
                    x, y, z, w, h,
                    wrap_width,
                ) }
            } else {
                unsafe { citro2d_sys::C2D_DrawText(
                    &self.raw_text, 
                    flags,
                    x, y, z, w, h,
                ) }
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
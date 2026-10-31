// 
// AllBinary Open License Version 1 
// Copyright (c) 2011 AllBinary 
// 
// By agreeing to this license you and any business entity you represent are 
// legally bound to the AllBinary Open License Version 1 legal agreement. 
// 
// You may obtain the AllBinary Open License Version 1 legal agreement from 
// AllBinary or the root directory of AllBinary's AllBinary Platform repository. 
// 
// Created By: Travis Berthelot 
//  
// Generated Code Do Not Modify
// package org.allbinary.emulator.device
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::File::File;
// use crate::java::io::FileOutputStream::FileOutputStream;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::android::graphics::Bitmap::Bitmap;
use crate::android::graphics::Canvas::Canvas;
use crate::android::graphics::Paint::Paint;
use crate::android::graphics::Rect::Rect;
use crate::android::graphics::Typeface::Typeface;
use crate::org::allbinary::data::resource::ResourceUtil::ResourceUtil;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::displayable::CanvasStrings::CanvasStrings;
use crate::org::allbinary::image::PreResourceImageUtil::PreResourceImageUtil;
use crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::microemu::android::device::AndroidImageInterface::AndroidImageInterface;
use crate::org::microemu::android::device::AndroidImmutableImage::AndroidImmutableImage;
use super::TrueTypeFontUtilBase::TrueTypeFontUtilBase;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//TTF
pub struct TrueTypeFontUtil {
    pub base: TrueTypeFontUtilBase,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub preResourceImageUtil: Option<Rc<RefCell<PreResourceImageUtil>>>,
    pub realFontSize: i32,
    pub fontImage: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>>,
}

thread_local! { pub static TrueTypeFontUtil_instance: Option<Rc<RefCell<TrueTypeFontUtil>>> = TrueTypeFontUtil::new(); }

impl TrueTypeFontUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<TrueTypeFontUtil>>> {
        return TrueTypeFontUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: TrueTypeFontUtilBase::new_struct(1),
            logUtil: None,
            preResourceImageUtil: None,
            realFontSize: 0,
            fontImage: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.preResourceImageUtil = PreResourceImageUtil::getInstance();
        this.realFontSize = 18;
        this.fontImage = crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage_NULL_OPENGL_IMAGE.with(Clone::clone).clone();
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn saveFontAtlasAsFile(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            let mut file: Option<Rc<RefCell<File>>> = jcall!(jcall!(ResourceUtil::getInstance()).getContext()).getFilesDir();
            let mut path: String = format!("{}{}", jcall!(file).getAbsolutePath(), jcall!(CanvasStrings::getInstance()).FONT_ATLAS);
            jcall!(self.logUtil).putF(path.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone());
            let mut fos: Option<Rc<RefCell<FileOutputStream>>> = FileOutputStream::new(path.clone());
            let mut bitmap: Option<Rc<RefCell<Bitmap>>> = jcall!(((jcall!(jcall!(self.fontImage).as_OpenGLESImage_mut().openGLBitmap).getImage()).clone() /* as AndroidImageInterface */)).getBitmap();
            jcall!(bitmap).compress(jcall!(unsafe { crate::android::graphics::Bitmap::Bitmap_CompressFormat.clone() }).PNG.clone(), 100, fos.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            PreLogUtil::putOE(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).EXCEPTION.clone(), e.clone());
            }
        }
    }
    

    pub fn getFontBitmapGL(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut filename: String, mut cellSize: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Image>>> {
        if match (&self.fontImage, &crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage_NULL_OPENGL_IMAGE.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut cellsPerRow2: i32 = self.CELLS_PER_ROW * 2;
            let mut cellsPerRow3: i32 = self.CELLS_PER_ROW * 3;
            let mut cellsPerRow4: i32 = self.CELLS_PER_ROW * 4;
            let mut cellsPerRow5: i32 = self.CELLS_PER_ROW * 5;
            let mut cellsPerRow6: i32 = self.CELLS_PER_ROW * 6;
            let mut cellsPerRow7: i32 = self.CELLS_PER_ROW * 7;
            let mut typeface: Option<Rc<RefCell<Typeface>>> = unsafe { crate::android::graphics::Typeface::Typeface_DEFAULT.clone() }.clone();
            let mut textureSize: i32 = self.getAsTextureSize(self.CELLS_PER_ROW * cellSize);
            let mut bitmap: Option<Rc<RefCell<Bitmap>>> = Bitmap::createBitmap(textureSize.clone(), textureSize.clone(), jcall!(unsafe { crate::android::graphics::Bitmap::Bitmap_Config.clone() }).ARGB_8888.clone());
            let mut canvas: Option<Rc<RefCell<Canvas>>> = Canvas::new(bitmap.clone());
            let mut paint: Option<Rc<RefCell<Paint>>> = Paint::new();
            jcall!(paint).setTypeface(typeface.clone());
            jcall!(paint).setTextSize(((self.realFontSize) as f32));
            jcall!(paint).setAlpha(((jcall!(basicColor).alpha) as i32));
            jcall!(paint).setColor(jcall!(basicColor).intValue());
            let mut biggestHeight: i32 = 0;
            let mut bounds: Option<Rc<RefCell<Rect>>> = Rect::new();
            let mut x: i32 = 0;
            let mut y: i32 = 0;
            for index in (0..self.size) {
                self.characterArray[(0) as usize] = self.pattern.chars().nth((index.clone()) as usize).unwrap();
                jcall!(paint).getTextBounds(self.characterArray.clone(), 0, 1, bounds.clone());
                self._characterWidth[(index) as usize] = jcall!(bounds).right.clone();
                if jcall!(bounds).bottom - jcall!(bounds).top > biggestHeight {
                    biggestHeight = jcall!(bounds).bottom - jcall!(bounds).top;
                }
                x = (index % self.CELLS_PER_ROW) * cellSize;
                x += (cellSize >> 1);
                x -= (self._characterWidth[(index) as usize] >> 1);
                y = 0;
                if index >= self.CELLS_PER_ROW {
                    y += cellSize.clone();
                }
                if index >= cellsPerRow2 {
                    y += cellSize.clone();
                }
                if index >= cellsPerRow3 {
                    y += cellSize.clone();
                }
                if index >= cellsPerRow4 {
                    y += cellSize.clone();
                }
                if index >= cellsPerRow5 {
                    y += cellSize.clone();
                }
                if index >= cellsPerRow6 {
                    y += cellSize.clone();
                }
                if index >= cellsPerRow7 {
                    y += cellSize.clone();
                }
                y += cellSize.clone();
                y -= (cellSize >> 2);
                jcall!(canvas).drawText(self.characterArray.clone(), 0, 1, ((x) as f32) - 3.0_f32, ((y) as f32) - 6.0_f32, paint.clone());
            }
            jcall!(canvas).save();
            let mut image: Option<Rc<RefCell<Image>>> = AndroidImmutableImage::create(bitmap.clone());
            self.fontImage = (jcall!(self.preResourceImageUtil).encapsulate(image.clone())).clone() /* as OpenGLESImage */;
            return self.fontImage.clone();
        } else {
            return self.fontImage.clone();
        }
    }
    

    pub fn getFontWidths(&mut self, mut filename: String, mut fontSize: i32) -> Vec<i32> {
        let mut typeface: Option<Rc<RefCell<Typeface>>> = unsafe { crate::android::graphics::Typeface::Typeface_DEFAULT.clone() }.clone();
        let mut paint: Option<Rc<RefCell<Paint>>> = Paint::new();
        jcall!(paint).setTypeface(typeface.clone());
        jcall!(paint).setTextSize(((fontSize) as f32));
        jcall!(paint).setARGB(255, 255, 255, 255);
        let mut bounds: Option<Rc<RefCell<Rect>>> = Rect::new();
        for index in (0..self.size) {
            self.characterArray[(0) as usize] = self.pattern.chars().nth((index.clone()) as usize).unwrap();
            jcall!(paint).getTextBounds(self.characterArray.clone(), 0, 1, bounds.clone());
            if index < self.lastCapIndex {
                if self.characterArray[(0) as usize] == '1' {
                    self._characterWidth[(index) as usize] = jcall!(bounds).right + 3;
                } else if self.characterArray[(0) as usize] == 'J' || self.characterArray[(0) as usize] == 'V' || self.characterArray[(0) as usize] == '2' || self.characterArray[(0) as usize] == '9' || self.characterArray[(0) as usize] == 'I' || self.characterArray[(0) as usize] == 'N' || self.characterArray[(0) as usize] == 'U' {
                    self._characterWidth[(index) as usize] = jcall!(bounds).right + 1;
                } else if self.characterArray[(0) as usize] == '4' || self.characterArray[(0) as usize] == 'C' || self.characterArray[(0) as usize] == 'E' || self.characterArray[(0) as usize] == 'O' {
                    self._characterWidth[(index) as usize] = jcall!(bounds).right - 2;
                } else if self.characterArray[(0) as usize] == 'B' || self.characterArray[(0) as usize] == 'D' || self.characterArray[(0) as usize] == 'G' || self.characterArray[(0) as usize] == 'H' || self.characterArray[(0) as usize] == 'T' || self.characterArray[(0) as usize] == 'W' {
                    self._characterWidth[(index) as usize] = jcall!(bounds).right - 3;
                } else if self.characterArray[(0) as usize] == 'A' || self.characterArray[(0) as usize] == 'Q' || self.characterArray[(0) as usize] == 'R' {
                    self._characterWidth[(index) as usize] = jcall!(bounds).right - 5;
                } else if self.characterArray[(0) as usize] == 'M' {
                    self._characterWidth[(index) as usize] = jcall!(bounds).right - 6;
                } else if self.characterArray[(0) as usize] == 'm' {
                    self._characterWidth[(index) as usize] = jcall!(bounds).right - 8;
                } else {
                    self._characterWidth[(index) as usize] = jcall!(bounds).right.clone();
                }
            } else {
                if self.characterArray[(0) as usize] == ' ' {
                    self._characterWidth[(index) as usize] = jcall!(bounds).right + 10;
                } else if self.characterArray[(0) as usize] == 'l' || self.characterArray[(0) as usize] == 'i' || self.characterArray[(0) as usize] == 'j' || self.characterArray[(0) as usize] == '.' || self.characterArray[(0) as usize] == '!' || self.characterArray[(0) as usize] == '|' {
                    self._characterWidth[(index) as usize] = jcall!(bounds).right + 6;
                } else if self.characterArray[(0) as usize] == 'f' || self.characterArray[(0) as usize] == 't' || self.characterArray[(0) as usize] == 'u' || self.characterArray[(0) as usize] == 'v' {
                    self._characterWidth[(index) as usize] = jcall!(bounds).right + 1;
                } else if self.characterArray[(0) as usize] == 'r' {
                    self._characterWidth[(index) as usize] = jcall!(bounds).right + 2;
                } else if self.characterArray[(0) as usize] == 'a' || self.characterArray[(0) as usize] == 'b' || self.characterArray[(0) as usize] == 'g' || self.characterArray[(0) as usize] == 'u' {
                    self._characterWidth[(index) as usize] = jcall!(bounds).right - 1;
                } else if self.characterArray[(0) as usize] == 'o' || self.characterArray[(0) as usize] == 'e' {
                    self._characterWidth[(index) as usize] = jcall!(bounds).right - 2;
                } else if self.characterArray[(0) as usize] == 'm' {
                    self._characterWidth[(index) as usize] = jcall!(bounds).right - 7;
                } else {
                    self._characterWidth[(index) as usize] = jcall!(bounds).right.clone();
                }
            }
        }
        self._characterWidth[(0) as usize] = (fontSize >> 1) - 2;
        return self._characterWidth.clone();
    }
}
impl std::ops::Deref for TrueTypeFontUtil {
    type Target = TrueTypeFontUtilBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TrueTypeFontUtil {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::emulator::device::TrueTypeFontUtilBase::TrueTypeFontUtilBaseDyn for TrueTypeFontUtil {
    fn as_TrueTypeFontUtilBase(&self) -> &crate::org::allbinary::emulator::device::TrueTypeFontUtilBase::TrueTypeFontUtilBase { &self.base }
    fn as_TrueTypeFontUtilBase_mut(&mut self) -> &mut crate::org::allbinary::emulator::device::TrueTypeFontUtilBase::TrueTypeFontUtilBase { &mut self.base }
}



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
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::eclipse::swt::SWT::SWT;
use crate::org::eclipse::swt::graphics::ImageData::ImageData;
use crate::org::eclipse::swt::graphics::ImageLoader::ImageLoader;
use crate::org::microemu::device::swt::SwtMutableImage::SwtMutableImage;
use crate::org::allbinary::AvianUtil::AvianUtil;
use crate::org::allbinary::data::resource::ResourceUtil::ResourceUtil;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::displayable::CanvasStrings::CanvasStrings;
use crate::org::allbinary::graphics::opengles::OpenGLCapabilities::OpenGLCapabilities;
use crate::org::allbinary::image::PreResourceImageUtil::PreResourceImageUtil;
use crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
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
    pub preResourceImageUtil: Option<Rc<RefCell<PreResourceImageUtil>>>,
    pub widthFloatArray: Vec<f32>,
    pub fontImage: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>>,
    pub widthScale: f32,
}

thread_local! { pub static TrueTypeFontUtil_instance: Option<Rc<RefCell<TrueTypeFontUtil>>> = TrueTypeFontUtil::new(); }

impl TrueTypeFontUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<TrueTypeFontUtil>>> {
        return TrueTypeFontUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: TrueTypeFontUtilBase::new_struct(2),
            preResourceImageUtil: None,
            widthFloatArray: Vec::new(),
            fontImage: None,
            widthScale: 0.0_f32,
        };
        this.preResourceImageUtil = PreResourceImageUtil::getInstance();
        this.fontImage = crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage_NULL_OPENGL_IMAGE.with(Clone::clone).clone();
        // super(...) is in the struct literal above
        if AvianUtil::isAvian() {
            this.widthFloatArray = vec![1504.0_f32, 967.0_f32, 527.0_f32, 961.0_f32, 957.0_f32, 1087.0_f32, 920.0_f32, 954.0_f32, 973.0_f32, 961.0_f32, 955.0_f32, 1296.0_f32, 1013.0_f32, 1106.0_f32, 1167.0_f32, 815.0_f32, 815.0_f32, 1216.0_f32, 1110.0_f32, 170.0_f32, 520.0_f32, 1056.0_f32, 815.0_f32, 1448.0_f32, 1142.0_f32, 1345.0_f32, 927.0_f32, 1345.0_f32, 1030.0_f32, 920.0_f32, 1096.0_f32, 1119.0_f32, 1219.0_f32, 1841.0_f32, 1166.0_f32, 1147.0_f32, 1005.0_f32, 879.0_f32, 965.0_f32, 792.0_f32, 964.0_f32, 927.0_f32, 753.0_f32, 1034.0_f32, 916.0_f32, 196.0_f32, 469.0_f32, 877.0_f32, 166.0_f32, 1563.0_f32, 916.0_f32, 1007.0_f32, 965.0_f32, 964.0_f32, 631.0_f32, 777.0_f32, 649.0_f32, 917.0_f32, 1026.0_f32, 1548.0_f32, 993.0_f32, 1028.0_f32, 795.0_f32, 241.0_f32, 798.0_f32, 241.0_f32, 905.0_f32, 1477.0_f32, 393.0_f32, 961.0_f32, 961.0_f32, 961.0_f32, 961.0_f32, 555.0_f32, 1029.0_f32, 1010.0_f32, 1378.0_f32, 952.0_f32, 952.0_f32, 952.0_f32, 952.0_f32, 463.0_f32, 463.0_f32, 926.0_f32, 961.0_f32, 491.0_f32, 930.0_f32, 457.0_f32, 457.0_f32, 644.0_f32, 643.0_f32, 326.0_f32, 186.0_f32, 1219.0_f32, 241.0_f32, 1599.0_f32, 961.0_f32, 302.0_f32, 711.0_f32, 961.0_f32, 961.0_f32, 710.0_f32, 141.0_f32, 1504.0_f32, 1504.0_f32, 1504.0_f32, 1504.0_f32, 1504.0_f32, 1504.0_f32, 1504.0_f32, 1504.0_f32];
        } else {
            this.widthFloatArray = vec![1504.0_f32, 967.0_f32, 527.0_f32, 961.0_f32, 957.0_f32, 1087.0_f32, 920.0_f32, 954.0_f32, 973.0_f32, 961.0_f32, 955.0_f32, 1296.0_f32, 1013.0_f32, 1106.0_f32, 1167.0_f32, 815.0_f32, 815.0_f32, 1216.0_f32, 1110.0_f32, 170.0_f32, 520.0_f32, 1056.0_f32, 815.0_f32, 1448.0_f32, 1142.0_f32, 1345.0_f32, 927.0_f32, 1345.0_f32, 1030.0_f32, 920.0_f32, 1096.0_f32, 1119.0_f32, 1219.0_f32, 1841.0_f32, 1166.0_f32, 1147.0_f32, 1005.0_f32, 879.0_f32, 965.0_f32, 792.0_f32, 964.0_f32, 927.0_f32, 753.0_f32, 1034.0_f32, 916.0_f32, 196.0_f32, 469.0_f32, 877.0_f32, 166.0_f32, 1563.0_f32, 916.0_f32, 1007.0_f32, 965.0_f32, 964.0_f32, 631.0_f32, 777.0_f32, 649.0_f32, 917.0_f32, 1026.0_f32, 1548.0_f32, 993.0_f32, 1028.0_f32, 795.0_f32, 241.0_f32, 798.0_f32, 241.0_f32, 905.0_f32, 1477.0_f32, 393.0_f32, 961.0_f32, 555.0_f32, 1029.0_f32, 1010.0_f32, 1378.0_f32, 952.0_f32, 463.0_f32, 463.0_f32, 926.0_f32, 961.0_f32, 491.0_f32, 930.0_f32, 457.0_f32, 457.0_f32, 644.0_f32, 643.0_f32, 326.0_f32, 186.0_f32, 1219.0_f32, 241.0_f32, 1599.0_f32, 961.0_f32, 302.0_f32, 711.0_f32, 961.0_f32, 961.0_f32, 710.0_f32, 141.0_f32, 1504.0_f32, 1504.0_f32];
        }
        this.widthScale = (if (this.scale == 2) { 1.2_f32 } else { 1.0_f32 });
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getFontBitmapGL(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut filename: String, mut cellSize: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Image>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if match (&self.fontImage, &crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage_NULL_OPENGL_IMAGE.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut canvasStrings: Option<Rc<RefCell<CanvasStrings>>> = CanvasStrings::getInstance();
                let mut openGLCapabilities: Option<Rc<RefCell<OpenGLCapabilities>>> = OpenGLCapabilities::getInstance();
                if jcall!(openGLCapabilities).isTextureSizeValid(1024) && self.scale == 2 {
                    let mut FONT_ATLAS_1024: String = String::from("/font_1024.png");
                    let mut image: Option<Rc<RefCell<Image>>> = Image::createImage(jcall!(ResourceUtil::getInstance()).getResourceAsStream(FONT_ATLAS_1024.clone()));
                    jcall!(image).setName(jcall!(canvasStrings).FONT_ATLAS.clone());
                    self.fontImage = (jcall!(self.preResourceImageUtil).encapsulate(image.clone())).clone() /* as OpenGLESImage */;
                } else if jcall!(openGLCapabilities).isTextureSizeValid(512) {
                    let mut FONT_ATLAS_512: String = String::from("/font_512.png");
                    let mut image: Option<Rc<RefCell<Image>>> = Image::createImage(jcall!(ResourceUtil::getInstance()).getResourceAsStream(FONT_ATLAS_512.clone()));
                    jcall!(image).setName(jcall!(canvasStrings).FONT_ATLAS.clone());
                    self.fontImage = (jcall!(self.preResourceImageUtil).encapsulate(image.clone())).clone() /* as OpenGLESImage */;
                } else {
                    let mut FONT_ATLAS_512: String = String::from("/font_512.png");
                    let mut image: Option<Rc<RefCell<Image>>> = Image::createImage(jcall!(ResourceUtil::getInstance()).getResourceAsStream(FONT_ATLAS_512.clone()));
                    jcall!(image).setName(jcall!(canvasStrings).FONT_ATLAS.clone());
                    self.fontImage = (jcall!(self.preResourceImageUtil).encapsulate(image.clone())).clone() /* as OpenGLESImage */;
                }
                return self.fontImage.clone();
            } else {
                return self.fontImage.clone();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            PreLogUtil::putOE(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).EXCEPTION.clone(), e.clone());
            panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
            }
        }
    }
    

    pub fn saveFontAtlasAsFile(&mut self) {
        let mut image: Option<Rc<RefCell<Image>>> = { let java_arguments = (None, self.base.cellSize.clone(), jcall!(BasicColorFactory::getInstance()).WHITE.clone(),); self.getFontBitmap(java_arguments.0, java_arguments.1, java_arguments.2) };
        let mut imageLoader: Option<Rc<RefCell<ImageLoader>>> = ImageLoader::new();
        let mut imageData: Option<Rc<RefCell<ImageData>>> = jcall!(jcall!(((image).clone() /* as SwtMutableImage */)).image).getImageData();
        jcall!(imageData).transparentPixel = jcall!(imageData).getPixel(0, 0);
        jcall!(imageLoader).data = vec![imageData.clone()];
        jcall!(imageLoader).save(format!("{}{}", jcall!(CommonSeps::getInstance()).PERIOD, jcall!(CanvasStrings::getInstance()).FONT_ATLAS), unsafe { crate::org::eclipse::swt::SWT::SWT_IMAGE_PNG.clone() }.clone());
    }
    

    pub fn getFontBitmap(&mut self, mut filename: String, mut cellSize: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Image>>> {
        let mut cellsPerRow2: i32 = self.CELLS_PER_ROW * 2;
        let mut cellsPerRow3: i32 = self.CELLS_PER_ROW * 3;
        let mut cellsPerRow4: i32 = self.CELLS_PER_ROW * 4;
        let mut cellsPerRow5: i32 = self.CELLS_PER_ROW * 5;
        let mut cellsPerRow6: i32 = self.CELLS_PER_ROW * 6;
        let mut cellsPerRow7: i32 = self.CELLS_PER_ROW * 7;
        let mut image: Option<Rc<RefCell<Image>>> = Image::createImage(self.base.textureSize.clone(), self.base.textureSize.clone());
        let mut graphics: Option<Rc<RefCell<Graphics>>> = jcall!(image).getGraphics();
        jcall!(graphics).setColor(jcall!(basicColor).intValue());
        let mut biggestHeight: i32 = 0;
        let mut bounds: Option<Rc<RefCell<Rectangle>>> = Rectangle::new(jcall!(PointFactory::getInstance()).createXY(0, 0), cellSize.clone(), cellSize.clone());
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        for index in (0..self.base.size) {
            self.characterArray[(0) as usize] = self.base.pattern.chars().nth((index.clone()) as usize).unwrap();
            self.base._characterWidth[(index) as usize] = jcall!(bounds).getMaxX();
            if jcall!(bounds).getMaxY() - jcall!(jcall!(bounds).getPoint()).getX() > biggestHeight {
                biggestHeight = jcall!(bounds).getMaxY() - jcall!(jcall!(bounds).getPoint()).getX();
            }
            x = (index % self.CELLS_PER_ROW) * cellSize;
            x += (cellSize >> 1);
            x -= (self.base._characterWidth[(index) as usize] >> 1);
            y = -(cellSize >> 1);
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
            jcall!(graphics).drawChar(self.characterArray[(0) as usize].clone(), x.clone(), y.clone(), 0);
        }
        return image.clone();
    }
    

    pub fn getFontWidths(&mut self, mut filename: String, mut fontSize: i32) -> Vec<i32> {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        for index in (0..self.base.size) {
            self.characterArray[(0) as usize] = self.base.pattern.chars().nth((index.clone()) as usize).unwrap();
            if self.characterArray[(0) as usize] == ' ' {
                self.base._characterWidth[(index) as usize] = (fontSize / 4);
            } else {
                let mut shortPatternIndex: i32 = self.shortPattern.find(self.characterArray[(0) as usize].clone()).map(|index| index as i32).unwrap_or(-1);
                let mut w: i32 = (((self.widthFloatArray[(shortPatternIndex) as usize] / 75.0_f32)) as i32) + 19;
                self.base._characterWidth[(index) as usize] = w;
                if self.characterArray[(0) as usize] == '.' || self.characterArray[(0) as usize] == '1' {
                    self.base._characterWidth[(index) as usize] += 4;
                }
                if self.characterArray[(0) as usize] >= 'a' && self.characterArray[(0) as usize] < 'k' {
                    self.base._characterWidth[(index) as usize] -= 2;
                }
                if self.characterArray[(0) as usize] >= 'k' && self.characterArray[(0) as usize] < 'v' {
                    self.base._characterWidth[(index) as usize] -= 4;
                }
                if self.characterArray[(0) as usize] >= 'v' && self.characterArray[(0) as usize] < 'z' {
                    self.base._characterWidth[(index) as usize] -= 8;
                }
                if self.characterArray[(0) as usize] == 'D' {
                    self.base._characterWidth[(index) as usize] -= 4;
                }
                if self.characterArray[(0) as usize] >= 'A' && self.characterArray[(0) as usize] <= 'Z' {
                    self.base._characterWidth[(index) as usize] -= 2;
                }
            }
            self.base._characterWidth[(index) as usize] *= self.widthScale.clone();
        }
        return self.base._characterWidth.clone();
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



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
// package org.allbinary.media.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::Graphics2D::Graphics2D;
// use crate::java::awt::GraphicsConfiguration::GraphicsConfiguration;
// use crate::java::awt::GraphicsDevice::GraphicsDevice;
// use crate::java::awt::GraphicsEnvironment::GraphicsEnvironment;
// use crate::java::awt::Transparency::Transparency;
// use crate::java::awt::geom::AffineTransform::AffineTransform;
// use crate::java::awt::image::BufferedImage::BufferedImage;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::microemu::device::j2se::J2SEImmutableImage::J2SEImmutableImage;
use crate::org::microemu::device::j2se::J2SEMutableImage::J2SEMutableImage;
use super::awt::awt;
use super::java::java;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageUtil {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
    pub IIOIMAGE_POOL_NAME: String,
    pub CREATE_BUFFERED_IMAGE: String,
}

thread_local! { pub static ImageUtil_instance: Option<Rc<RefCell<ImageUtil>>> = ImageUtil::new(); }

impl ImageUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<ImageUtil>>> {
        return ImageUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            commonStrings: None,
            commonSeps: None,
            IIOIMAGE_POOL_NAME: String::new(),
            CREATE_BUFFERED_IMAGE: String::new(),
        };
        this.commonStrings = CommonStrings::getInstance();
        this.commonSeps = CommonSeps::getInstance();
        this.IIOIMAGE_POOL_NAME = String::from("IIOIMAGE_POOL_NAME");
        this.CREATE_BUFFERED_IMAGE = String::from("createBufferedImage");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getDefaultConfiguration(&mut self) -> Option<Rc<RefCell<GraphicsConfiguration>>> {
        let mut ge: Option<Rc<RefCell<GraphicsEnvironment>>> = GraphicsEnvironment::getLocalGraphicsEnvironment();
        let mut gd: Option<Rc<RefCell<GraphicsDevice>>> = jcall!(ge).getDefaultScreenDevice();
        return jcall!(gd).getDefaultConfiguration();
    }
    

    pub fn create(&mut self, mut width: i32, mut height: i32) -> Option<Rc<RefCell<BufferedImage>>> {
        let mut graphicsConfiguration: Option<Rc<RefCell<GraphicsConfiguration>>> = self.getDefaultConfiguration();
        return jcall!(graphicsConfiguration).createCompatibleImage(width.clone(), height.clone(), unsafe { crate::java::awt::Transparency::Transparency_TRANSLUCENT.clone() }.clone());
    }
    

    // throws Exception
    pub fn createBufferedImageForResize(&mut self, mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>>, mut percent: i32, mut scale: bool) -> Vec<Option<Rc<RefCell<BufferedImage>>>> {
        let mut size: i32 = (bufferedImageArray.len() as i32).clone();
        let mut scaledBufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = vec![None; (size) as usize];
        let mut bufferedImage: Option<Rc<RefCell<BufferedImage>>> = None;
        for index in (0..size) {
            bufferedImage = bufferedImageArray[(index) as usize].clone();
            let mut newWidth: i32 = (((jcall!(bufferedImage).getWidth() * percent / 100)) as i32);
            let mut newHeight: i32 = (((jcall!(bufferedImage).getHeight() * percent / 100)) as i32);
            scaledBufferedImageArray[(index) as usize] = self.createBufferedImage(bufferedImage.clone(), newWidth.clone(), newHeight.clone(), scale.clone());
        }
        return scaledBufferedImageArray.clone();
    }
    

    // throws Exception
    pub fn createBufferedImage2(&mut self, mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>>, mut percent: f32, mut scale: bool) -> Vec<Option<Rc<RefCell<BufferedImage>>>> {
        let mut size: i32 = (bufferedImageArray.len() as i32).clone();
        let mut scaledBufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = vec![None; (size) as usize];
        let mut bufferedImage: Option<Rc<RefCell<BufferedImage>>> = None;
        for index in (0..size) {
            bufferedImage = bufferedImageArray[(index) as usize].clone();
            let mut newWidth: i32 = (((jcall!(bufferedImage).getWidth() * percent)) as i32);
            let mut newHeight: i32 = (((jcall!(bufferedImage).getHeight() * percent)) as i32);
            scaledBufferedImageArray[(index) as usize] = self.createBufferedImage(bufferedImage.clone(), newWidth.clone(), newHeight.clone(), scale.clone());
        }
        return scaledBufferedImageArray.clone();
    }
    

    // throws Exception
    pub fn createBufferedImage3(&mut self, mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>>, mut width: i32, mut height: i32, mut scale: bool) -> Vec<Option<Rc<RefCell<BufferedImage>>>> {
        let mut size: i32 = (bufferedImageArray.len() as i32).clone();
        let mut scaledBufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = vec![None; (size) as usize];
        for index in (0..size) {
            scaledBufferedImageArray[(index) as usize] = self.createBufferedImage(bufferedImageArray[(index) as usize].clone(), width.clone(), height.clone(), scale.clone());
        }
        return scaledBufferedImageArray.clone();
    }
    

    // throws Exception
    pub fn createBufferedImageForSave(&mut self, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut newWidth: i32, mut newHeight: i32) -> Option<Rc<RefCell<BufferedImage>>> {
        return self.createBufferedImage(bufferedImage.clone(), newWidth.clone(), newHeight.clone(), true);
    }
    

    // throws Exception
    pub fn createBufferedImage(&mut self, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut newWidth: i32, mut newHeight: i32, mut scale: bool) -> Option<Rc<RefCell<BufferedImage>>> {
        return self.createBufferedImageAllowTranslate(bufferedImage.clone(), newWidth.clone(), newHeight.clone(), scale.clone(), false);
    }
    

    // throws Exception
    pub fn createBufferedImageAllowTranslate(&mut self, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut newWidth: i32, mut newHeight: i32, mut scale: bool, mut allowTranslate: bool) -> Option<Rc<RefCell<BufferedImage>>> {
        let mut width: f64 = jcall!(bufferedImage).getWidth();
        let mut height: f64 = jcall!(bufferedImage).getHeight();
        let mut d_newWidth: f64 = ((newWidth) as f64).clone();
        let mut d_newHeight: f64 = ((newHeight) as f64).clone();
        let mut widthRatio: f64 = d_newWidth / width;
        let mut heightRatio: f64 = d_newHeight / height;
        let mut ratioX: f64 = 1.0;
        let mut ratioY: f64 = 1.0;
        if scale {
            ratioX = widthRatio.clone();
            ratioY = heightRatio.clone();
        }
        let mut affineTransform: Option<Rc<RefCell<AffineTransform>>> = AffineTransform::getScaleInstance(ratioX.clone(), ratioY.clone());
        if !scale && allowTranslate {
            let mut dx: f64 = (((newWidth) as f64) - width) / 2.0;
            let mut dy: f64 = (((newHeight) as f64) - height) / 2.0;
            jcall!(affineTransform).translate(dx.clone(), dy.clone());
        }
        let mut newBufferedImage: Option<Rc<RefCell<BufferedImage>>> = BufferedImage::new(newWidth.clone(), newHeight.clone(), unsafe { crate::java::awt::image::BufferedImage::BufferedImage_TYPE_INT_ARGB_PRE.clone() }.clone());
        let mut graphics: Option<Rc<RefCell<Graphics2D>>> = jcall!(newBufferedImage).createGraphics();
        jcall!(graphics).drawRenderedImage(bufferedImage.clone(), affineTransform.clone());
        return newBufferedImage.clone();
    }
    

    // throws Exception
    pub fn createBufferedImageWithLargerCanvas(&mut self, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut newWidth: i32, mut newHeight: i32) -> Option<Rc<RefCell<BufferedImage>>> {
        let mut width: f64 = jcall!(bufferedImage).getWidth();
        let mut height: f64 = jcall!(bufferedImage).getHeight();
        let mut d_newWidth: f64 = ((newWidth) as f64).clone();
        let mut d_newHeight: f64 = ((newHeight) as f64).clone();
        let mut widthRatio: f64 = d_newWidth / width;
        let mut heightRatio: f64 = d_newHeight / height;
        let mut dx2: i32 = (((((newWidth) as f64) - width)) as i32);
        let mut dy2: i32 = (((((newHeight) as f64) - height)) as i32);
        let mut dx: i32 = dx2 / 2;
        let mut dy: i32 = dy2 / 2;
        let mut dx4: i32 = dx2 / 4;
        let mut dy4: i32 = dy2 / 4;
        let mut newBufferedImage: Option<Rc<RefCell<BufferedImage>>> = BufferedImage::new(newWidth.clone(), newHeight.clone(), unsafe { crate::java::awt::image::BufferedImage::BufferedImage_TYPE_INT_ARGB_PRE.clone() }.clone());
        let mut graphics: Option<Rc<RefCell<Graphics2D>>> = jcall!(newBufferedImage).createGraphics();
        jcall!(graphics).drawImage(bufferedImage.clone(), dx4.clone(), dy4.clone(), None);
        return newBufferedImage.clone();
    }
    

    pub fn convertToBufferedImage(&mut self, mut toolkitImage: Option<Rc<RefCell<Image>>>) -> Option<Rc<RefCell<BufferedImage>>> {
        if toolkitImage.is_none() {
            return None;
        }
        let mut bufferedImage: Option<Rc<RefCell<BufferedImage>>> = BufferedImage::new(jcall!(toolkitImage).getWidth(None), jcall!(toolkitImage).getHeight(None), unsafe { crate::java::awt::image::BufferedImage::BufferedImage_TYPE_INT_ARGB.clone() }.clone());
        let mut g2d: Option<Rc<RefCell<Graphics2D>>> = jcall!(bufferedImage).createGraphics();
        jcall!(g2d).drawImage(toolkitImage.clone(), 0, 0, None);
        jcall!(g2d).dispose();
        return bufferedImage.clone();
    }
    

    pub fn getBufferedImage(&mut self, mut image: Option<Rc<RefCell<Image>>>) -> Option<Rc<RefCell<BufferedImage>>> {
        let mut bufferedImage: Option<Rc<RefCell<BufferedImage>>> = None;
        if jcall!(image).isMutable() {
            let mut j2seImage: Option<Rc<RefCell<J2SEMutableImage>>> = (image).clone() /* as J2SEMutableImage */;
            bufferedImage = (jcall!(j2seImage).getImage()).clone() /* as BufferedImage */;
        } else {
            let mut j2seImage: Option<Rc<RefCell<J2SEImmutableImage>>> = (image).clone() /* as J2SEImmutableImage */;
            bufferedImage = (jcall!(j2seImage).getImage()).clone() /* as BufferedImage */;
        }
        return bufferedImage.clone();
    }
    

    pub fn toString(&mut self, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>) -> String {
        let mut commonLabels: Option<Rc<RefCell<CommonLabels>>> = CommonLabels::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from(" BufferedImage -"));
        jcall!(stringBuffer).append(jcall!(commonLabels).WIDTH_LABEL.clone());
        jcall!(stringBuffer).appendint(((jcall!(bufferedImage).getWidth()) as i32));
        jcall!(stringBuffer).append(jcall!(commonLabels).HEIGHT_LABEL.clone());
        jcall!(stringBuffer).appendint(((jcall!(bufferedImage).getHeight()) as i32));
        jcall!(stringBuffer).append(String::from(" Type: "));
        jcall!(stringBuffer).appendint(((jcall!(bufferedImage).getType()) as i32));
        return jcall!(stringBuffer).toString();
    }
}


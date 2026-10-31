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
// use crate::java::awt::geom::AffineTransform::AffineTransform;
// use crate::java::awt::image::BufferedImage::BufferedImage;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::image::ImageCache::ImageCache;
use super::ImageUtil::ImageUtil;
use super::ImageJ2SEUtil::ImageJ2SEUtil;
use super::ImageCreationUtil::ImageCreationUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageScaleUtil {
    pub imageUtil: Option<Rc<RefCell<ImageUtil>>>,
    pub imageJ2SEUtil: Option<Rc<RefCell<ImageJ2SEUtil>>>,
    pub imageCreationUtil: Option<Rc<RefCell<ImageCreationUtil>>>,
}

thread_local! { pub static ImageScaleUtil_instance: Option<Rc<RefCell<ImageScaleUtil>>> = ImageScaleUtil::new(); }

impl ImageScaleUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<ImageScaleUtil>>> {
        return ImageScaleUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            imageUtil: None,
            imageJ2SEUtil: None,
            imageCreationUtil: None,
        };
        this.imageUtil = ImageUtil::getInstance();
        this.imageJ2SEUtil = ImageJ2SEUtil::getInstance();
        this.imageCreationUtil = ImageCreationUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn createImage2(&mut self, mut imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>, mut originalImage: Option<Rc<RefCell<Image>>>, mut scaleNominatorX: f32, mut scaleDenominatorX: f32, mut scaleNominatorY: f32, mut scaleDenominatorY: f32, mut cached: bool) -> Option<Rc<RefCell<Image>>> {
        return self.createImage3(imageCache.clone(), originalImage.clone(), scaleNominatorX.clone(), scaleDenominatorX.clone(), scaleNominatorY.clone(), scaleDenominatorY.clone(), cached.clone(), true);
    }
    

    // throws Exception
    pub fn createImage3(&mut self, mut imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>, mut originalImage: Option<Rc<RefCell<Image>>>, mut scaleNominatorX: f32, mut scaleDenominatorX: f32, mut scaleNominatorY: f32, mut scaleDenominatorY: f32, mut cached: bool, mut mutable: bool) -> Option<Rc<RefCell<Image>>> {
        let mut width: i32 = jcall!(originalImage).getWidth();
        let mut height: i32 = jcall!(originalImage).getHeight();
        let mut scaleX: f32 = scaleNominatorX / scaleDenominatorX;
        let mut scaleY: f32 = scaleNominatorY / scaleDenominatorY;
        let mut image: Option<Rc<RefCell<Image>>> = None;
        if cached {
            image = jcall!(imageCache).get(std::any::type_name_of_val(&self).to_string(), (((((width) as f32) * scaleX)) as i32), (((((height) as f32) * scaleY)) as i32));
        } else {
            image = jcall!(imageCache).get(String::from("createImage"), (((((width) as f32) * scaleX)) as i32), (((((height) as f32) * scaleY)) as i32));
        }
        self.scale(originalImage.clone(), image.clone(), scaleX.clone(), scaleY.clone(), true);
        return image.clone();
    }
    

    // throws Exception
    pub fn scale2(&mut self, mut originalImage: Option<Rc<RefCell<Image>>>, mut originalImageArray: Vec<Option<Rc<RefCell<Image>>>>, mut ximageToShowArray: Vec<Option<Rc<RefCell<Image>>>>, mut unused: i32, mut scaleX: f32, mut scaleY: f32, mut maxScaleX: f32, mut maxScaleY: f32) {
        if maxScaleX * jcall!(originalImage).getWidth() > jcall!(originalImageArray[(0) as usize]).getWidth() || maxScaleY * jcall!(originalImage).getHeight() > jcall!(originalImageArray[(0) as usize]).getHeight() {
            originalImageArray[(0) as usize] = { let java_receiver = self.imageCreationUtil.clone(); let java_arguments = (jcall!(originalImage).getWidth(), jcall!(originalImage).getHeight(), maxScaleX.clone(), maxScaleY.clone(),); let java_result = jcall!(java_receiver).createImage(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); java_result };
        }
        self.scale(originalImage.clone(), originalImageArray[(0) as usize].clone(), scaleX.clone(), scaleY.clone(), false);
    }
    

    pub fn scale(&mut self, mut originalImage: Option<Rc<RefCell<Image>>>, mut newMaxSizeImage: Option<Rc<RefCell<Image>>>, mut scaleX: f32, mut scaleY: f32, mut clear: bool) {
        let mut bufferedImage: Option<Rc<RefCell<BufferedImage>>> = jcall!(self.imageUtil).getBufferedImage(originalImage.clone());
        let mut newBufferedImage: Option<Rc<RefCell<BufferedImage>>> = jcall!(self.imageUtil).getBufferedImage(newMaxSizeImage.clone());
        let mut at: Option<Rc<RefCell<AffineTransform>>> = AffineTransform::getScaleInstance(scaleX.clone(), scaleY.clone());
        let mut g: Option<Rc<RefCell<Graphics2D>>> = jcall!(newBufferedImage).createGraphics();
        if clear {
            jcall!(g).setBackground(jcall!(self.imageJ2SEUtil).TRANSPARENT_COLOR.clone());
            jcall!(g).clearRect(0, 0, jcall!(newBufferedImage).getWidth(), jcall!(newBufferedImage).getHeight());
        }
        jcall!(g).drawRenderedImage(bufferedImage.clone(), at.clone());
        jcall!(g).dispose();
    }
}


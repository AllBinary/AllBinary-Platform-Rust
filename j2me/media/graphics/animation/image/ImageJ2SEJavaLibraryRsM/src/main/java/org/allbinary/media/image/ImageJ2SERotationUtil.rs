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
// use crate::java::awt::Image::Image;
// use crate::java::awt::image::BufferedImage::BufferedImage;
use super::ImageUtil::ImageUtil;
use super::ImageJ2SEUtil::ImageJ2SEUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageJ2SERotationUtil {
    pub imageUtil: Option<Rc<RefCell<ImageUtil>>>,
    pub imageJ2SEUtil: Option<Rc<RefCell<ImageJ2SEUtil>>>,
    pub TWO_PIE: f64,
}

thread_local! { pub static ImageJ2SERotationUtil_instance: Option<Rc<RefCell<ImageJ2SERotationUtil>>> = ImageJ2SERotationUtil::new(); }

impl ImageJ2SERotationUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<ImageJ2SERotationUtil>>> {
        return ImageJ2SERotationUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            imageUtil: None,
            imageJ2SEUtil: None,
            TWO_PIE: 0.0,
        };
        this.imageUtil = ImageUtil::getInstance();
        this.imageJ2SEUtil = ImageJ2SEUtil::getInstance();
        this.TWO_PIE = 2 * unsafe { Math_PI.clone() };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getRotatedImage(&mut self, mut bufferedImage: Option<Rc<RefCell<Image>>>, mut totalAngle: i32) -> Option<Rc<RefCell<BufferedImage>>> {
        return self.getRotatedImageRadians(bufferedImage.clone(), self.TWO_PIE * ((totalAngle) as f64) / 360.0);
    }
    

    pub fn getRotatedImageRadians(&mut self, mut bufferedImage: Option<Rc<RefCell<Image>>>, mut radians: f64) -> Option<Rc<RefCell<BufferedImage>>> {
        let mut newBufferedImage: Option<Rc<RefCell<BufferedImage>>> = { let java_receiver = self.imageUtil.clone(); let java_arguments = (jcall!(bufferedImage).getWidth(None), jcall!(bufferedImage).getHeight(None),); let java_result = jcall!(java_receiver).create(java_arguments.0, java_arguments.1); java_result };
        return { let java_arguments = (bufferedImage.clone(), newBufferedImage.clone(), jcall!(newBufferedImage).createGraphics(), radians.clone(),); self.drawRotatedImageRadians(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3) };
    }
    

    pub fn drawRotatedImageRadians(&mut self, mut bufferedImage: Option<Rc<RefCell<Image>>>, mut newBufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut g: Option<Rc<RefCell<Graphics2D>>>, mut radians: f64) -> Option<Rc<RefCell<BufferedImage>>> {
        jcall!(g).rotate(radians.clone(), jcall!(newBufferedImage).getWidth(None) / 2, jcall!(newBufferedImage).getHeight(None) / 2);
        jcall!(g).drawImage(bufferedImage.clone(), 0, 0, jcall!(bufferedImage).getWidth(None), jcall!(bufferedImage).getHeight(None), None);
        jcall!(g).dispose();
        return newBufferedImage.clone();
    }
    

    pub fn rotateImage(&mut self, mut bufferedImage: Option<Rc<RefCell<Image>>>, mut newBufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut totalAngle: i32) -> Option<Rc<RefCell<BufferedImage>>> {
        let mut g: Option<Rc<RefCell<Graphics2D>>> = jcall!(newBufferedImage).createGraphics();
        jcall!(g).setBackground(jcall!(self.imageJ2SEUtil).TRANSPARENT_COLOR.clone());
        jcall!(g).clearRect(0, 0, jcall!(newBufferedImage).getWidth(), jcall!(newBufferedImage).getHeight());
        return self.drawRotatedImageRadians(bufferedImage.clone(), newBufferedImage.clone(), g.clone(), self.TWO_PIE * ((totalAngle) as f64) / 360.0);
    }
    

    pub fn createRotatedImage(&mut self, mut bufferedImage: Option<Rc<RefCell<Image>>>, mut newBufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut totalAngle: i32) -> Option<Rc<RefCell<BufferedImage>>> {
        let mut g: Option<Rc<RefCell<Graphics2D>>> = jcall!(newBufferedImage).createGraphics();
        return self.drawRotatedImageRadians(bufferedImage.clone(), newBufferedImage.clone(), g.clone(), self.TWO_PIE * ((totalAngle) as f64) / 360.0);
    }
    

    pub fn getRotatedImages(&mut self, mut bufferedImage: Option<Rc<RefCell<Image>>>, mut numberOfFrames: i32, mut totalAngle: i32) -> Vec<Option<Rc<RefCell<BufferedImage>>>> {
        let mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = vec![None; (numberOfFrames) as usize];
        let mut arc: f64 = (self.TWO_PIE) * ((totalAngle) as f64) / 360.0;
        let mut size: i32 = (bufferedImageArray.len() as i32).clone();
        for index in (0..size) {
            let mut radians: f64 = (arc / ((size) as f64)) * ((index) as f64);
            bufferedImageArray[(index) as usize] = self.getRotatedImageRadians(bufferedImage.clone(), radians.clone());
        }
        return bufferedImageArray.clone();
    }
    

    pub fn createSpriteImage(&mut self, mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>>) -> Option<Rc<RefCell<BufferedImage>>> {
        let mut columns: i32 = 9;
        let mut rows: i32 = 0;
        let mut size: i32 = (bufferedImageArray.len() as i32).clone();
        if size < columns {
            columns = size.clone();
        }
        rows = (size / columns);
        if size % columns != 0 {
            rows += 1;
        }
        let mut firstBufferedImage: Option<Rc<RefCell<BufferedImage>>> = bufferedImageArray[(0) as usize].clone();
        let mut bufferedImage: Option<Rc<RefCell<BufferedImage>>> = { let java_receiver = self.imageUtil.clone(); let java_arguments = (jcall!(firstBufferedImage).getWidth(None) * columns, jcall!(firstBufferedImage).getHeight(None) * rows,); let java_result = jcall!(java_receiver).create(java_arguments.0, java_arguments.1); java_result };
        let mut g: Option<Rc<RefCell<Graphics2D>>> = jcall!(bufferedImage).createGraphics();
        let mut columnIndex: i32 = 0;
        let mut rowIndex: i32 = 0;
        let mut nextBufferedImage: Option<Rc<RefCell<BufferedImage>>> = None;
        for index in (0..size) {
            if index / 9 != 0 && index % 9 == 0 {
                rowIndex += 1;
                columnIndex = 0;
            }
            nextBufferedImage = bufferedImageArray[(index) as usize].clone();
            jcall!(g).drawImage(nextBufferedImage.clone(), jcall!(nextBufferedImage).getWidth(None) * columnIndex, jcall!(nextBufferedImage).getHeight(None) * rowIndex, jcall!(nextBufferedImage).getWidth(None), jcall!(nextBufferedImage).getHeight(None), None);
            columnIndex += 1;
        }
        jcall!(g).dispose();
        return bufferedImage.clone();
    }
}


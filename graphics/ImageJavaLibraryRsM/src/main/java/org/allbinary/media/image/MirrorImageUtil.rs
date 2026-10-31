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
// use crate::java::awt::*;
// use crate::java::awt::geom::AffineTransform::AffineTransform;
// use crate::java::awt::image::AffineTransformOp::AffineTransformOp;
// use crate::java::awt::image::BufferedImage::BufferedImage;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MirrorImageUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub imageUtil: Option<Rc<RefCell<ImageUtil>>>,
}

thread_local! { pub static MirrorImageUtil_instance: Option<Rc<RefCell<MirrorImageUtil>>> = MirrorImageUtil::new(); }

impl MirrorImageUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<MirrorImageUtil>>> {
        return MirrorImageUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            imageUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.imageUtil = ImageUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getImage(&mut self, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut verticle: bool, mut horizontal: bool) -> Option<Rc<RefCell<BufferedImage>>> {
        jcall!(self.logUtil).putF(String::from("Starting"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getImage"));
        let mut newBufferedImage: Option<Rc<RefCell<BufferedImage>>> = { let java_receiver = self.imageUtil.clone(); let java_arguments = (jcall!(bufferedImage).getWidth(None), jcall!(bufferedImage).getHeight(None),); let java_result = jcall!(java_receiver).create(java_arguments.0, java_arguments.1); java_result };
        let mut g: Option<Rc<RefCell<Graphics2D>>> = jcall!(newBufferedImage).createGraphics();
        if verticle {
            let mut tx: Option<Rc<RefCell<AffineTransform>>> = AffineTransform::getScaleInstance(1, -1);
            jcall!(tx).translate(0, -jcall!(bufferedImage).getHeight(None));
            let mut op: Option<Rc<RefCell<AffineTransformOp>>> = AffineTransformOp::new(tx.clone(), unsafe { crate::java::awt::image::AffineTransformOp::AffineTransformOp_TYPE_NEAREST_NEIGHBOR.clone() }.clone());
            bufferedImage = jcall!(op).filter(bufferedImage.clone(), None);
        }
        if horizontal {
            let mut tx: Option<Rc<RefCell<AffineTransform>>> = AffineTransform::getScaleInstance(-1, 1);
            jcall!(tx).translate(-jcall!(bufferedImage).getWidth(None), 0);
            let mut op: Option<Rc<RefCell<AffineTransformOp>>> = AffineTransformOp::new(tx.clone(), unsafe { crate::java::awt::image::AffineTransformOp::AffineTransformOp_TYPE_NEAREST_NEIGHBOR.clone() }.clone());
            bufferedImage = jcall!(op).filter(bufferedImage.clone(), None);
        }
        jcall!(g).drawImage(bufferedImage.clone(), 0, 0, jcall!(bufferedImage).getWidth(None), jcall!(bufferedImage).getHeight(None), None);
        jcall!(g).dispose();
        return newBufferedImage.clone();
    }
    

    pub fn getImages(&mut self, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut verticle: bool, mut horizontal: bool) -> Vec<Option<Rc<RefCell<BufferedImage>>>> {
        let mut width: i32 = jcall!(bufferedImage).getWidth();
        let mut height: i32 = jcall!(bufferedImage).getHeight();
        let mut cellHeight: i32 = height.clone();
        let mut cellWidth: i32 = height.clone();
        let mut numberOfFramesPerOrientation: i32 = width / cellWidth;
        let mut numberOfFrames: i32 = numberOfFramesPerOrientation.clone();
        if verticle {
            numberOfFrames *= 2;
        }
        if horizontal {
            numberOfFrames *= 2;
        }
        jcall!(self.logUtil).putF(format!("{}{}", format!("{}{}", format!("{}{}", String::from("numberOfFramesPerOrientation: "), numberOfFramesPerOrientation), String::from(" numberOfFrames: ")), numberOfFrames), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getImages"));
        let mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = vec![None; (numberOfFrames) as usize];
        let mut y: i32 = 0;
        for index in (0..numberOfFramesPerOrientation) {
            let mut x: i32 = index * cellWidth;
            bufferedImageArray[(index) as usize] = jcall!(bufferedImage).getSubimage(x.clone(), y.clone(), cellWidth.clone(), cellHeight.clone());
        }
        for index in (0..numberOfFramesPerOrientation) {
            bufferedImageArray[(index + numberOfFramesPerOrientation) as usize] = self.getImage(bufferedImageArray[(index) as usize].clone(), verticle.clone(), horizontal.clone());
        }
        return bufferedImageArray.clone();
    }
}


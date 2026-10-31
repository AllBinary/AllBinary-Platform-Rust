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
// use crate::java::awt::image::BufferedImage::BufferedImage;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImagesRatioUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub imageUtil: Option<Rc<RefCell<ImageUtil>>>,
}

thread_local! { pub static ImagesRatioUtil_instance: Option<Rc<RefCell<ImagesRatioUtil>>> = ImagesRatioUtil::new(); }

impl ImagesRatioUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<ImagesRatioUtil>>> {
        return ImagesRatioUtil_instance.with(Clone::clone).clone();
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
    

    pub fn isEqual(&mut self, mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>>, mut totalImages: i32) -> bool {
        let mut end: i32 = totalImages.clone();
        if (bufferedImageArray.len() as i32) < totalImages {
            end = (bufferedImageArray.len() as i32).clone();
        }
        let mut ratio: f64 = (((jcall!(bufferedImageArray[(0) as usize]).getWidth()) as f64) / jcall!(bufferedImageArray[(0) as usize]).getHeight());
        for index in (1..end) {
            if ratio != (((jcall!(bufferedImageArray[(index) as usize]).getWidth()) as f64) / jcall!(bufferedImageArray[(index) as usize]).getHeight()) {
                return false;
            }
        }
        return true;
    }
    

    pub fn getAverage(&mut self, mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>>, mut totalImages: i32) -> f64 {
        let mut ratio: f64 = 0.0;
        let mut end: i32 = totalImages.clone();
        if (bufferedImageArray.len() as i32) < totalImages {
            end = (bufferedImageArray.len() as i32).clone();
        }
        for index in (0..end) {
            ratio += (((jcall!(bufferedImageArray[(index) as usize]).getWidth()) as f64) / jcall!(bufferedImageArray[(index) as usize]).getHeight());
        }
        return (ratio / ((end) as f64));
    }
    

    // throws Exception
    pub fn fudge(&mut self, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut ratio: f64) -> Option<Rc<RefCell<BufferedImage>>> {
        let mut imageRatio: f64 = (((jcall!(bufferedImage).getWidth()) as f64) / jcall!(bufferedImage).getHeight());
        let mut point: Option<Rc<RefCell<Point>>> = Point::new();
        let mut newHeight: i32 = jcall!(bufferedImage).getHeight();
        let mut newWidth: i32 = jcall!(bufferedImage).getWidth();
        if ratio > imageRatio {
            newWidth = (((((jcall!(bufferedImage).getHeight()) as f64) * ratio)) as i32);
            jcall!(point).x = (newWidth - jcall!(bufferedImage).getWidth()) / 2;
        } else {
            newHeight = (((((jcall!(bufferedImage).getWidth()) as f64) / ratio)) as i32);
            jcall!(point).y = (newHeight - jcall!(bufferedImage).getHeight()) / 2;
        }
        let mut commonLabels: Option<Rc<RefCell<CommonLabels>>> = CommonLabels::getInstance();
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", jcall!(commonLabels).WIDTH_LABEL, jcall!(bufferedImage).getWidth()), String::from(" newWidth: ")), newWidth), jcall!(commonLabels).HEIGHT_LABEL), jcall!(bufferedImage).getHeight()), String::from(" newHeight: ")), newHeight), String::from(" needed ratio: ")), (((newWidth) as f64) / ((newHeight) as f64))), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("fudge"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut newBufferedImage: Option<Rc<RefCell<BufferedImage>>> = jcall!(self.imageUtil).create(newWidth.clone(), newHeight.clone());
        let mut g: Option<Rc<RefCell<Graphics2D>>> = jcall!(newBufferedImage).createGraphics();
        if jcall!(point).x > 0 {
            let mut firstColumnBufferedImage: Option<Rc<RefCell<BufferedImage>>> = jcall!(bufferedImage).getSubimage(0, 0, 1, jcall!(bufferedImage).getHeight());
            let mut lastColumnBufferedImage: Option<Rc<RefCell<BufferedImage>>> = jcall!(bufferedImage).getSubimage(jcall!(bufferedImage).getWidth() - 1, 0, 1, jcall!(bufferedImage).getHeight());
            jcall!(self.logUtil).putF(String::from("Draw some columns to fill in gap"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("fudge"));
            for index in (0..jcall!(point).x) {
                jcall!(g).drawImage(firstColumnBufferedImage.clone(), index.clone(), 0, None);
                jcall!(g).drawImage(lastColumnBufferedImage.clone(), newWidth - index, 0, None);
            }
        }
        if jcall!(point).y > 0 {
            let mut firstRowBufferedImage: Option<Rc<RefCell<BufferedImage>>> = jcall!(bufferedImage).getSubimage(0, 0, jcall!(bufferedImage).getWidth(), 1);
            let mut lastRowBufferedImage: Option<Rc<RefCell<BufferedImage>>> = jcall!(bufferedImage).getSubimage(0, jcall!(bufferedImage).getHeight() - 1, jcall!(bufferedImage).getWidth(), 1);
            jcall!(self.logUtil).putF(String::from("Draw some rows to fill in gap"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("fudge"));
            for index in (0..jcall!(point).y) {
                jcall!(g).drawImage(firstRowBufferedImage.clone(), 0, index.clone(), None);
                jcall!(g).drawImage(lastRowBufferedImage.clone(), 0, newHeight - index, None);
            }
        }
        jcall!(g).drawImage(bufferedImage.clone(), jcall!(point).x.clone(), jcall!(point).y.clone(), jcall!(bufferedImage).getWidth(), jcall!(bufferedImage).getHeight(), None);
        return newBufferedImage.clone();
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn fudge3(&mut self, mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>>, mut totalImages: i32, mut ratio: f64) -> Vec<Option<Rc<RefCell<BufferedImage>>>> {
        let mut end: i32 = totalImages.clone();
        if (bufferedImageArray.len() as i32) < totalImages {
            end = (bufferedImageArray.len() as i32).clone();
        }
        let mut fudgedBufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = vec![None; (end) as usize];
        for index in (0..end) {
            fudgedBufferedImageArray[(index) as usize] = self.fudge(bufferedImageArray[(index) as usize].clone(), ratio.clone());
        }
        return fudgedBufferedImageArray.clone();
    }
}


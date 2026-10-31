// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// use crate::java::awt::image::BufferedImage::BufferedImage;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::ImageUtil::ImageUtil;
use super::ImageProcessorInput::ImageProcessorInput;
use super::ImageProcessedVisitor::ImageProcessedVisitor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CanvasExpandUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub stringUtil: Option<Rc<RefCell<StringUtil>>>,
}

thread_local! { pub static CanvasExpandUtil_instance: Option<Rc<RefCell<CanvasExpandUtil>>> = CanvasExpandUtil::new(); }

impl CanvasExpandUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            stringUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.stringUtil = StringUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<CanvasExpandUtil>>> {
        return CanvasExpandUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn process(&mut self, mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>, mut increaseX: i32, mut increaseY: i32, mut visitor: Option<Rc<RefCell<dyn ImageProcessedVisitor>>>) {
        let mut imageUtil: Option<Rc<RefCell<ImageUtil>>> = ImageUtil::getInstance();
        let mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = jcall!(imageProcessorInput).getBufferedImageArray();
        let mut subBufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = vec![None; ((bufferedImageArray.len() as i32)) as usize];
        let mut size: i32 = (bufferedImageArray.len() as i32).clone();
        let mut bufferedImage: Option<Rc<RefCell<BufferedImage>>> = None;
        for index in (0..size) {
            bufferedImage = bufferedImageArray[(index) as usize].clone();
            subBufferedImageArray[(index) as usize] = jcall!(imageUtil).createBufferedImage(bufferedImage.clone(), jcall!(bufferedImage).getWidth() + increaseX, jcall!(bufferedImage).getHeight() + increaseY, false);
            jcall!(visitor).visit(subBufferedImageArray[(index) as usize].clone(), jcall!(self.stringUtil).EMPTY_STRING.clone(), index.clone());
        }
    }
}


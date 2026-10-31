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
// use crate::java::io::IOException::IOException;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::ImageProcessorInput::ImageProcessorInput;
use super::ImageProcessedVisitor::ImageProcessedVisitor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CanvasTrimUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub stringUtil: Option<Rc<RefCell<StringUtil>>>,
}

thread_local! { pub static CanvasTrimUtil_instance: Option<Rc<RefCell<CanvasTrimUtil>>> = CanvasTrimUtil::new(); }

impl CanvasTrimUtil {

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
    

    pub fn getInstance() -> Option<Rc<RefCell<CanvasTrimUtil>>> {
        return CanvasTrimUtil_instance.with(Clone::clone).clone();
    }
    

    // throws IOException
    pub fn process(&mut self, mut imageProcessorInput: Option<Rc<RefCell<ImageProcessorInput>>>, mut cutCanvasFromLeftReduction: i32, mut cutCanvasFromTopReduction: i32, mut cutCanvasFromRightReduction: i32, mut cutCanvasFromBottomReduction: i32, mut visitor: Option<Rc<RefCell<dyn ImageProcessedVisitor>>>) {
        let mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = jcall!(imageProcessorInput).getBufferedImageArray();
        let mut subBufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = vec![None; ((bufferedImageArray.len() as i32)) as usize];
        let mut size: i32 = (bufferedImageArray.len() as i32).clone();
        let mut bufferedImage: Option<Rc<RefCell<BufferedImage>>> = None;
        for index in (0..size) {
            bufferedImage = bufferedImageArray[(index) as usize].clone();
            subBufferedImageArray[(index) as usize] = jcall!(bufferedImage).getSubimage(cutCanvasFromLeftReduction.clone(), cutCanvasFromTopReduction.clone(), jcall!(bufferedImage).getWidth() - cutCanvasFromLeftReduction - cutCanvasFromRightReduction, jcall!(bufferedImage).getHeight() - cutCanvasFromTopReduction - cutCanvasFromBottomReduction);
            jcall!(visitor).visit(subBufferedImageArray[(index) as usize].clone(), jcall!(self.stringUtil).EMPTY_STRING.clone(), index.clone());
        }
    }
}


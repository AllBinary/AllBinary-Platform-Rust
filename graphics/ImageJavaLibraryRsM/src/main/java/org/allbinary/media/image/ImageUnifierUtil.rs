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

pub struct ImageUnifierUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub imageUtil: Option<Rc<RefCell<ImageUtil>>>,
}

thread_local! { pub static ImageUnifierUtil_instance: Option<Rc<RefCell<ImageUnifierUtil>>> = ImageUnifierUtil::new(); }

impl ImageUnifierUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<ImageUnifierUtil>>> {
        return ImageUnifierUtil_instance.with(Clone::clone).clone();
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
    

    pub fn getDefaultConfiguration(&mut self) -> Option<Rc<RefCell<GraphicsConfiguration>>> {
        let mut ge: Option<Rc<RefCell<GraphicsEnvironment>>> = GraphicsEnvironment::getLocalGraphicsEnvironment();
        let mut gd: Option<Rc<RefCell<GraphicsDevice>>> = jcall!(ge).getDefaultScreenDevice();
        return jcall!(gd).getDefaultConfiguration();
    }
    

    pub fn getImage(&mut self, mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>>, mut imageUnifierProperties: Option<Rc<RefCell<ImageUnifierProperties>>>) -> Option<Rc<RefCell<BufferedImage>>> {
        let mut newBufferedImage: Option<Rc<RefCell<BufferedImage>>> = { let java_receiver = self.imageUtil.clone(); let java_arguments = (jcall!(imageUnifierProperties).getWidth(), jcall!(imageUnifierProperties).getHeight(),); let java_result = jcall!(java_receiver).create(java_arguments.0, java_arguments.1); java_result };
        let mut commonLabels: Option<Rc<RefCell<CommonLabels>>> = CommonLabels::getInstance();
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("Setting Image - "), jcall!(commonLabels).WIDTH_LABEL), jcall!(newBufferedImage).getWidth()), jcall!(commonLabels).HEIGHT_LABEL), jcall!(newBufferedImage).getHeight()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getImage"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut g: Option<Rc<RefCell<Graphics2D>>> = jcall!(newBufferedImage).createGraphics();
        let mut columnIndex: i32 = 0;
        let mut rowIndex: i32 = 0;
        for index in (0..(bufferedImageArray.len() as i32)) {
            let mut x: i32 = ((jcall!(jcall!(imageUnifierProperties).getImageUnifierCell()).getWidth()).unwrap() as i32) * { let value = columnIndex; columnIndex += 1; value };
            let mut y: i32 = ((jcall!(jcall!(imageUnifierProperties).getImageUnifierCell()).getHeight()).unwrap() as i32) * rowIndex;
            jcall!(self.logUtil).putF(format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("Adding Image: "), index), String::from(" x: ")), x), String::from(" y: ")), y), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getImage"));
            jcall!(g).drawImage(bufferedImageArray[(index) as usize].clone(), x.clone(), y.clone(), ((jcall!(jcall!(imageUnifierProperties).getImageUnifierCell()).getWidth()).unwrap() as i32), ((jcall!(jcall!(imageUnifierProperties).getImageUnifierCell()).getHeight()).unwrap() as i32), None);
            let mut totalColumnsMinusOne: i32 = (((jcall!(imageUnifierProperties).getColumns()).unwrap() as i32) - 1);
            if columnIndex > totalColumnsMinusOne {
                rowIndex += 1;
                columnIndex = 0;
            }
        }
        jcall!(g).dispose();
        return newBufferedImage.clone();
    }
}


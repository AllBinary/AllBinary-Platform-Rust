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
// package org.allbinary.input.media.image.capture
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
use crate::org::allbinary::input::automation::ImageOutputData::ImageOutputData;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::java::number::LongUtil::LongUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::util::cache::AutomaticCacheInterface::AutomaticCacheInterface;
use crate::org::allbinary::logic::visual::media::MediaDataFactory::MediaDataFactory;
use crate::org::allbinary::media::image::ImagePersistanceUtil::ImagePersistanceUtil;
use crate::org::allbinary::media::image::ImageUtil::ImageUtil;
use crate::org::allbinary::media::image::cache::BufferedImageFrameCacheable::BufferedImageFrameCacheable;
use crate::org::allbinary::media::image::io::ImageIOInterface::ImageIOInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CapturedImageInputOutput {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

impl ImageIOInterface for CapturedImageInputOutput {
    fn save(&mut self, mut frame: Option<i64>) { Self::save(self, frame) }
}

impl CapturedImageInputOutput {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn save(&mut self, mut frame: Option<i64>) {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        let mut capturedBufferedImageCacheable: Option<Rc<RefCell<BufferedImageFrameCacheable>>> = ((jcall!(((CapturedBufferedImagesCacheSingleton::CapturedBufferedImagesCacheSingleton::getInstance()).clone() /* as AutomaticCacheInterface */)).get(frame.clone())).clone() /* as BufferedImageFrameCacheable */);
        { let java_receiver = self.logUtil.clone(); let java_arguments = ((format!("{}{}", String::from("Saving: "), jcall!(capturedBufferedImageCacheable).toString())).clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).SAVE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        { let java_arguments = (jcall!(capturedBufferedImageCacheable).getBufferedImage(), frame.clone(),); self.save2(java_arguments.0, java_arguments.1) };
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn save2(&mut self, mut bufferedImage: Option<Rc<RefCell<BufferedImage>>>, mut frame: Option<i64>) {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        let mut imageUtil: Option<Rc<RefCell<ImageUtil>>> = ImageUtil::getInstance();
        let mut filePathStringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(filePathStringBuffer).append(crate::org::allbinary::input::automation::ImageOutputData::ImageOutputData_SAVE_PATH.with(Clone::clone).clone());
        jcall!(filePathStringBuffer).append(LongUtil::fillIn((frame).expect("NullPointerException").to_string()));
        jcall!(filePathStringBuffer).append(jcall!(jcall!(MediaDataFactory::getInstance()).JPG).getExtension());
        let mut filePath: String = jcall!(filePathStringBuffer).toString();
        { let java_receiver = self.logUtil.clone(); let java_arguments = ((format!("{}{}", format!("{}{}", String::from("Image File Path: "), filePath), jcall!(imageUtil).toString(bufferedImage.clone()))).clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).SAVE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut imagePersistanceUtil: Option<Rc<RefCell<ImagePersistanceUtil>>> = ImagePersistanceUtil::getInstance();
        jcall!(imagePersistanceUtil).saveWithImageIO(filePath.clone(), bufferedImage.clone());
    }
}


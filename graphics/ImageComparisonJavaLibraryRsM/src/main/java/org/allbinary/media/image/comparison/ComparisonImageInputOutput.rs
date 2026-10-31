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
// package org.allbinary.media.image.comparison
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
use crate::org::allbinary::media::image::cache::BufferedImageCacheable::BufferedImageCacheable;
use crate::org::allbinary::media::image::io::ImageIOInterface::ImageIOInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::ImageComparisonResultFrameCacheable::ImageComparisonResultFrameCacheable;
use super::ImageComparisonResult::ImageComparisonResult;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ComparisonImageInputOutput {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static ComparisonImageInputOutput_ROOT_NAME: String = String::from("_Changed_"); }
impl ImageIOInterface for ComparisonImageInputOutput {
    fn save(&mut self, mut frame: Option<i64>) { Self::save(self, frame) }
}

impl ComparisonImageInputOutput {
    

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
        if frame > 0 {
            let mut imageComparisonResultFrameCacheable: Option<Rc<RefCell<ImageComparisonResultFrameCacheable>>> = (jcall!(((ImageComparisonResultCacheSingleton::ImageComparisonResultCacheSingleton::getInstance()).clone() /* as AutomaticCacheInterface */)).get(frame.clone())).clone() /* as ImageComparisonResultFrameCacheable */;
            if imageComparisonResultFrameCacheable.is_some() {
                let mut imageComparisonResult: Option<Rc<RefCell<ImageComparisonResult>>> = jcall!(imageComparisonResultFrameCacheable).getImageComparisonResult();
                { let java_arguments = (imageComparisonResult.clone(), jcall!(imageComparisonResultFrameCacheable).getFrame(),); self.save2(java_arguments.0, java_arguments.1) };
            } else {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Comparison Results Not Available for Output: "), frame), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).SAVE.clone());
            }
        } else {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("No Comparison Results: for first frame: "), frame), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).SAVE.clone());
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn save2(&mut self, mut imageComparisonResult: Option<Rc<RefCell<ImageComparisonResult>>>, mut frame: Option<i64>) {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        let mut filePathStringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(filePathStringBuffer).append(crate::org::allbinary::input::automation::ImageOutputData::ImageOutputData_SAVE_PATH.with(Clone::clone).clone());
        jcall!(filePathStringBuffer).append(LongUtil::fillIn((frame).expect("NullPointerException").to_string()));
        jcall!(filePathStringBuffer).append(ComparisonImageInputOutput_ROOT_NAME.with(Clone::clone).clone());
        let mut filePathStringBuffer1: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(filePathStringBuffer1).append(jcall!(filePathStringBuffer).toString());
        jcall!(filePathStringBuffer1).append(String::from("_1"));
        jcall!(filePathStringBuffer1).append(jcall!(jcall!(MediaDataFactory::getInstance()).JPG).getExtension());
        let mut filePath1: String = jcall!(filePathStringBuffer1).toString();
        let mut filePathStringBuffer2: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(filePathStringBuffer2).append(jcall!(filePathStringBuffer).toString());
        jcall!(filePathStringBuffer2).append(String::from("_2"));
        jcall!(filePathStringBuffer2).append(jcall!(jcall!(MediaDataFactory::getInstance()).JPG).getExtension());
        let mut filePath2: String = jcall!(filePathStringBuffer2).toString();
        let mut bufferedImageCacheables: Vec<Option<Rc<RefCell<BufferedImageCacheable>>>> = ChangedPixelsUtil::ChangedPixelsUtil::generateBufferedImageChacheables(imageComparisonResult.clone());
        jcall!(self.logUtil).putF(format!("{}{}", String::from("Comparison Image File Path 1: "), filePath1), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).SAVE.clone());
        jcall!(self.logUtil).putF(format!("{}{}", String::from("Comparison Image File Path 2: "), filePath2), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).SAVE.clone());
        let mut bufferedImageArray: Vec<Option<Rc<RefCell<BufferedImage>>>> = vec![None; (2) as usize];
        bufferedImageArray[(0) as usize] = jcall!(bufferedImageCacheables[(0) as usize]).getBufferedImage();
        bufferedImageArray[(1) as usize] = jcall!(bufferedImageCacheables[(1) as usize]).getBufferedImage();
        let mut imagePersistanceUtil: Option<Rc<RefCell<ImagePersistanceUtil>>> = ImagePersistanceUtil::getInstance();
        jcall!(imagePersistanceUtil).saveWithImageIO(filePath1.clone(), bufferedImageArray[(0) as usize].clone());
        jcall!(imagePersistanceUtil).saveWithImageIO(filePath2.clone(), bufferedImageArray[(1) as usize].clone());
    }
}


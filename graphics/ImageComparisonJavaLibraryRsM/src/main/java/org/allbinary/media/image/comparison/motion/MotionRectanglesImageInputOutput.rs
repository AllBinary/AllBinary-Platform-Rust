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
// package org.allbinary.media.image.comparison.motion
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::*;
// use crate::java::awt::image::BufferedImage::BufferedImage;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::input::automation::ImageOutputData::ImageOutputData;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::java::number::LongUtil::LongUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::util::cache::AutomaticCacheInterface::AutomaticCacheInterface;
use crate::org::allbinary::logic::util::cache::PoolInterface::PoolInterface;
use crate::org::allbinary::logic::visual::media::MediaDataFactory::MediaDataFactory;
use crate::org::allbinary::media::image::ImagePersistanceUtil::ImagePersistanceUtil;
use crate::org::allbinary::media::image::cache::BufferedImageCacheable::BufferedImageCacheable;
use crate::org::allbinary::media::image::cache::BufferedImageInfo::BufferedImageInfo;
use crate::org::allbinary::media::image::cache::BufferedImageInfoFactory::BufferedImageInfoFactory;
use crate::org::allbinary::media::image::cache::BufferedImagePoolSingleton::BufferedImagePoolSingleton;
use crate::org::allbinary::media::image::comparison::ImageComparisonResult::ImageComparisonResult;
use crate::org::allbinary::media::image::io::ImageIOInterface::ImageIOInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MotionRectanglesImageInputOutput {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub NUMBER_OF_IMAGES: i32,
}

impl ImageIOInterface for MotionRectanglesImageInputOutput {
    fn save(&mut self, mut frame: Option<i64>) { Self::save(self, frame) }
}

impl MotionRectanglesImageInputOutput {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            NUMBER_OF_IMAGES: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.NUMBER_OF_IMAGES = 1;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn save(&mut self, mut frame: Option<i64>) {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        let mut motionRectanglesResultsFrameCacheable: Option<Rc<RefCell<MotionRectanglesResultsFrameCacheable>>> = (jcall!(((AllMotionRectanglesResultsCacheSingleton::AllMotionRectanglesResultsCacheSingleton::getInstance()).clone() /* as AutomaticCacheInterface */)).get(frame.clone())).clone() /* as MotionRectanglesResultsFrameCacheable */;
        if motionRectanglesResultsFrameCacheable.is_some() {
            let mut motionRectangles: Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>> = jcall!(motionRectanglesResultsFrameCacheable).getMotionRectangles();
            self.save2(motionRectangles.clone(), frame.clone());
            motionRectanglesResultsFrameCacheable = (jcall!(((ConsolidatedMotionRectanglesResultsCacheSingleton::ConsolidatedMotionRectanglesResultsCacheSingleton::getInstance()).clone() /* as AutomaticCacheInterface */)).get(frame.clone())).clone() /* as MotionRectanglesResultsFrameCacheable */;
            motionRectangles = jcall!(motionRectanglesResultsFrameCacheable).getMotionRectangles();
            self.save2(motionRectangles.clone(), frame.clone());
            motionRectanglesResultsFrameCacheable = (jcall!(((ConstrainedMotionRectanglesResultsCacheSingleton::ConstrainedMotionRectanglesResultsCacheSingleton::getInstance()).clone() /* as AutomaticCacheInterface */)).get(frame.clone())).clone() /* as MotionRectanglesResultsFrameCacheable */;
            motionRectangles = jcall!(motionRectanglesResultsFrameCacheable).getMotionRectangles();
            self.save2(motionRectangles.clone(), frame.clone());
        } else {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("No Motion Rectangle Results for Frame: "), frame), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).SAVE.clone());
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn save2(&mut self, mut motionRectangles: Option<Rc<RefCell<dyn crate::org::allbinary::media::image::comparison::motion::MotionRectangles::MotionRectanglesDyn>>>, mut frame: Option<i64>) {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        let mut filePathStringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(filePathStringBuffer).append(crate::org::allbinary::input::automation::ImageOutputData::ImageOutputData_SAVE_PATH.with(Clone::clone).clone());
        jcall!(filePathStringBuffer).append(LongUtil::fillIn((frame).expect("NullPointerException").to_string()));
        jcall!(filePathStringBuffer).append(String::from("_"));
        jcall!(filePathStringBuffer).append(jcall!(motionRectangles).getName());
        let mut filePathStringBuffer1: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(filePathStringBuffer1).append(jcall!(filePathStringBuffer).toString());
        jcall!(filePathStringBuffer1).append(String::from("_1"));
        jcall!(filePathStringBuffer1).append(jcall!(jcall!(MediaDataFactory::getInstance()).JPG).getExtension());
        let mut filePathStringBuffer2: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(filePathStringBuffer2).append(jcall!(filePathStringBuffer).toString());
        jcall!(filePathStringBuffer2).append(String::from("_2"));
        jcall!(filePathStringBuffer2).append(jcall!(jcall!(MediaDataFactory::getInstance()).JPG).getExtension());
        let mut filePathStringBuffer3: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(filePathStringBuffer3).append(jcall!(filePathStringBuffer).toString());
        jcall!(filePathStringBuffer3).append(jcall!(jcall!(MediaDataFactory::getInstance()).JPG).getExtension());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Motion Image File Path 1: "), jcall!(filePathStringBuffer1).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).SAVE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Motion Image File Path 2: "), jcall!(filePathStringBuffer2).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).SAVE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Motion Image File Path 2: "), jcall!(filePathStringBuffer3).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).SAVE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut bufferedImageCacheables: Vec<Option<Rc<RefCell<BufferedImageCacheable>>>> = vec![None; (self.NUMBER_OF_IMAGES) as usize];
        let mut imageComparisonInfo: Option<Rc<RefCell<ImageComparisonResult>>> = jcall!(motionRectangles).getImageComparisonResult();
        let mut originalBufferedImage: Option<Rc<RefCell<BufferedImage>>> = jcall!(imageComparisonInfo).getBufferedImages()[(0) as usize].clone();
        let mut bufferedImageInfo: Option<Rc<RefCell<BufferedImageInfo>>> = BufferedImageInfoFactory::getInstance(jcall!(imageComparisonInfo).imageWidth.clone(), jcall!(imageComparisonInfo).imageHeight.clone(), jcall!(originalBufferedImage).getType());
        let mut poolInterface: Option<Rc<RefCell<dyn PoolInterface>>> = BufferedImagePoolSingleton::getInstance().map(|reference| reference as Rc<RefCell<dyn PoolInterface>>);
        for index in (0..self.NUMBER_OF_IMAGES) {
            bufferedImageCacheables[(index) as usize] = (jcall!(poolInterface).remove(crate::java_into_object(bufferedImageInfo.clone()))).clone() /* as BufferedImageCacheable */;
        }
        jcall!(jcall!(bufferedImageCacheables[(0) as usize]).getBufferedImage()).setData(jcall!(jcall!(imageComparisonInfo).getBufferedImages()[(1) as usize]).getData());
        if self.NUMBER_OF_IMAGES > 1 {
            jcall!(jcall!(bufferedImageCacheables[(1) as usize]).getBufferedImage()).setData(jcall!(jcall!(imageComparisonInfo).getBufferedImages()[(1) as usize]).getData());
        }
        let mut bufferedImageGraphicsArray: Vec<Option<Rc<RefCell<Graphics>>>> = Vec::new();
        bufferedImageGraphicsArray = vec![None; (self.NUMBER_OF_IMAGES) as usize];
        for index in (0..self.NUMBER_OF_IMAGES) {
            bufferedImageGraphicsArray[(index) as usize] = jcall!(jcall!(bufferedImageCacheables[(index) as usize]).getBufferedImage()).getGraphics();
        }
        let mut rectangelList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(motionRectangles).getVector();
        let mut size: i32 = jcall!(rectangelList).size();
        for index2 in (0..size) {
            let mut motionRectangle: Option<Rc<RefCell<MotionRectangle>>> = { let java_object = (jcall!(rectangelList).get(index2.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<MotionRectangle>>>>().expect("ClassCastException").clone()) };
            let mut rectangle: Option<Rc<RefCell<Rectangle>>> = jcall!(motionRectangle).getRectangle();
            for index in (0..self.NUMBER_OF_IMAGES) {
                jcall!(bufferedImageGraphicsArray[(index) as usize]).setColor(unsafe { Color_ORANGE.clone() }.clone());
                jcall!(bufferedImageGraphicsArray[(index) as usize]).drawRect(jcall!(rectangle).x.clone(), jcall!(rectangle).y.clone(), jcall!(rectangle).width.clone(), jcall!(rectangle).height.clone());
            }
        }
        let mut imagePersistanceUtil: Option<Rc<RefCell<ImagePersistanceUtil>>> = ImagePersistanceUtil::getInstance();
        if self.NUMBER_OF_IMAGES > 1 {
            jcall!(imagePersistanceUtil).saveWithImageIO(jcall!(filePathStringBuffer1).toString(), jcall!(bufferedImageCacheables[(1) as usize]).getBufferedImage());
        }
        jcall!(imagePersistanceUtil).saveWithImageIO(jcall!(filePathStringBuffer2).toString(), jcall!(bufferedImageCacheables[(0) as usize]).getBufferedImage());
        if self.NUMBER_OF_IMAGES > 2 {
            jcall!(imagePersistanceUtil).saveWithImageIO(jcall!(filePathStringBuffer3).toString(), jcall!(bufferedImageCacheables[(2) as usize]).getBufferedImage());
        }
    }
}


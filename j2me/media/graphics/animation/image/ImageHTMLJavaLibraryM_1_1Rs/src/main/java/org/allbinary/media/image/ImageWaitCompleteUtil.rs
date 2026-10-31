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
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::image::GameFeatureImageCacheFactory::GameFeatureImageCacheFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::HashtableUtil::HashtableUtil;
use crate::org::microemu::device::playn::PlaynImage::PlaynImage;
use super::ImageCompleteUtil::ImageCompleteUtil;
use super::core::core;
use super::playn::playn;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageWaitCompleteUtil {
    pub base: ImageCompleteUtil,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub timeDelayHelper: Option<Rc<RefCell<TimeDelayHelper>>>,
    pub allTimeDelayHelper: Option<Rc<RefCell<TimeDelayHelper>>>,
}


impl ImageWaitCompleteUtil {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ImageCompleteUtil::new_struct(),
            logUtil: None,
            timeDelayHelper: None,
            allTimeDelayHelper: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.timeDelayHelper = TimeDelayHelper::new(18000);
        this.allTimeDelayHelper = TimeDelayHelper::new(120000);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn waitForImage(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut name: String) {
        jcall!(self.timeDelayHelper).setStartTimeTNT();
        self.waitForATime(image.clone(), name.clone(), self.timeDelayHelper.clone());
    }
    

    // throws Exception
    pub fn handleTimeout(&mut self, mut name: String) {
        if jcall!(self.timeDelayHelper).isTimeTNT() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("isReady: Timeout Waiting or GameHtmlHasLoadedResourcesProcessor does not have this Image: "), name))));
        }
    }
    

    // throws Exception
    pub fn waitForATime(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut name: String, mut timeDelayHelper: Option<Rc<RefCell<TimeDelayHelper>>>) {
        let mut playnImage: Option<Rc<RefCell<PlaynImage>>> = (image).clone() /* as PlaynImage */;
        let mut playnCoreImage: Option<Rc<RefCell<Image>>> = (jcall!(playnImage).getImage()).clone() /* as Image */;
        while !jcall!(playnCoreImage).isReady() || jcall!(playnCoreImage).width() + jcall!(playnCoreImage).height() <= 0 {
            if jcall!(timeDelayHelper).isTimeTNT() {
                panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("waitFor: Timeout Waiting or GameHtmlHasLoadedResourcesProcessor does not have this Image: "), name))));
            }
        }
        if !jcall!(image).isReady() {
            jcall!(image).init(jcall!(image).getImage());
        }
    }
    

    // throws Exception
    pub fn waitForAll(&mut self) {
        jcall!(self.allTimeDelayHelper).setStartTimeTNT();
        let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = jcall!(GameFeatureImageCacheFactory::getInstance()).getHashtableP();
        let mut objectArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(HashtableUtil::getInstance()).getKeysAsArray(hashtable.clone());
        let mut size: i32 = (objectArray.len() as i32).clone();
        PreLogUtil::put(format!("{}{}", String::from("Image Total: "), size), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("waitForAll"));
        for index in (0..size) {
            { let java_arguments = ((jcall!(hashtable).get(objectArray[(index) as usize].clone())).clone() /* as Image */, (objectArray[(index) as usize]).clone() /* as String */, self.allTimeDelayHelper.clone(),); self.waitForATime(java_arguments.0, java_arguments.1, java_arguments.2) };
        }
    }
}
impl std::ops::Deref for ImageWaitCompleteUtil {
    type Target = ImageCompleteUtil;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageWaitCompleteUtil {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::image::ImageCompleteUtil::ImageCompleteUtilDyn for ImageWaitCompleteUtil {
    fn as_ImageCompleteUtil(&self) -> &crate::org::allbinary::media::image::ImageCompleteUtil::ImageCompleteUtil { &self.base }
    fn as_ImageCompleteUtil_mut(&mut self) -> &mut crate::org::allbinary::media::image::ImageCompleteUtil::ImageCompleteUtil { &mut self.base }
    fn waitForImage(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut name: String) { Self::waitForImage(self, image, name) }
    fn handleTimeout(&mut self, mut name: String) { Self::handleTimeout(self, name) }
    fn waitForAll(&mut self) { Self::waitForAll(self) }
}



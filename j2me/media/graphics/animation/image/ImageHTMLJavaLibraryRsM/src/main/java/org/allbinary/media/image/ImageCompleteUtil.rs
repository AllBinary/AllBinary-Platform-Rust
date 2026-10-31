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
use crate::Hashtable;
use super::*;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::image::GameFeatureImageCacheFactory::GameFeatureImageCacheFactory;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::allbinary::util::HashtableUtil::HashtableUtil;
use crate::org::microemu::device::playn::PlaynImage::PlaynImage;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageCompleteUtil {
    pub timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub allTimeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
}

thread_local! { pub static ImageCompleteUtil_instance: Option<Rc<RefCell<dyn ImageCompleteUtilDyn>>> = ImageCompleteUtil::new().map(|reference| reference as Rc<RefCell<dyn ImageCompleteUtilDyn>>); }

impl ImageCompleteUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            timeDelayHelper: None,
            allTimeDelayHelper: None,
        };
        this.timeDelayHelper = TimeDelayHelper::new(18000).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.allTimeDelayHelper = TimeDelayHelper::new(120000).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn ImageCompleteUtilDyn>>> {
        return ImageCompleteUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn waitForImage(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut name: String) {
        jcall!(self.timeDelayHelper).as_TimeDelayHelper_mut().setStartTime();
        self.waitForATime(image.clone(), name.clone(), self.timeDelayHelper.clone());
    }
    

    // throws Exception
    pub fn isReady(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut name: String, mut timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>) -> bool {
        let mut playnImage: Option<Rc<RefCell<PlaynImage>>> = (image).clone() /* as PlaynImage */;
        if !jcall!(jcall!(playnImage).getImage()).isReady() && jcall!(jcall!(playnImage).getImage()).width() + jcall!(jcall!(playnImage).getImage()).height() == 0 {
            if jcall!(timeDelayHelper).as_TimeDelayHelper_mut().isTime() {
                panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("isReady: Timeout Waiting or GameHtmlHasLoadedResourcesProcessor does not have this Image: "), name))));
            }
            return false;
        }
        return true;
    }
    

    // throws Exception
    pub fn waitForATime(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut name: String, mut timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>) {
        let mut playnImage: Option<Rc<RefCell<PlaynImage>>> = (image).clone() /* as PlaynImage */;
        while !jcall!(jcall!(playnImage).getImage()).isReady() && jcall!(jcall!(playnImage).getImage()).width() + jcall!(jcall!(playnImage).getImage()).height() == 0 {
            if jcall!(timeDelayHelper).as_TimeDelayHelper_mut().isTime() {
                panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("waitFor: Timeout Waiting or GameHtmlHasLoadedResourcesProcessor does not have this Image: "), name))));
            }
        }
    }
    

    // throws Exception
    pub fn waitForAll(&mut self) {
        jcall!(self.allTimeDelayHelper).as_TimeDelayHelper_mut().setStartTime();
        let mut hashtable: Option<Rc<RefCell<Hashtable>>> = jcall!(GameFeatureImageCacheFactory::getInstance()).getHashtable();
        let mut objectArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(HashtableUtil::getInstance()).getKeysAsArray(hashtable.clone());
        let mut size: i32 = (objectArray.len() as i32).clone();
        PreLogUtil::put(format!("{}{}", String::from("Total: "), size), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("waitForAll"));
        for index in (0..size) {
            { let java_arguments = ((jcall!(hashtable).get(objectArray[(index) as usize].clone())).clone() /* as Image */, (objectArray[(index) as usize]).clone() /* as String */, self.allTimeDelayHelper.clone(),); self.waitForATime(java_arguments.0, java_arguments.1, java_arguments.2) };
        }
    }
}
pub trait ImageCompleteUtilDyn {
    fn as_ImageCompleteUtil(&self) -> &ImageCompleteUtil;
    fn as_ImageCompleteUtil_mut(&mut self) -> &mut ImageCompleteUtil;
    fn waitForImage(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut name: String) { ImageCompleteUtil::waitForImage(self.as_ImageCompleteUtil_mut(), image, name) }
    fn isReady(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut name: String, mut timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>) -> bool { ImageCompleteUtil::isReady(self.as_ImageCompleteUtil_mut(), image, name, timeDelayHelper) }
    fn waitForATime(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut name: String, mut timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>) { ImageCompleteUtil::waitForATime(self.as_ImageCompleteUtil_mut(), image, name, timeDelayHelper) }
    fn waitForAll(&mut self) { ImageCompleteUtil::waitForAll(self.as_ImageCompleteUtil_mut()) }
}
impl ImageCompleteUtilDyn for ImageCompleteUtil {
    fn as_ImageCompleteUtil(&self) -> &ImageCompleteUtil { self }
    fn as_ImageCompleteUtil_mut(&mut self) -> &mut ImageCompleteUtil { self }
}



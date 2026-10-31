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
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::microemu::device::playn::PlaynImage::PlaynImage;
use crate::playn::core::CoreImage::CoreImage;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageCompleteUtil {
    pub ISREADY: String,
    pub COMPLETE: String,
}

thread_local! { pub static ImageCompleteUtil_instance: Option<Rc<RefCell<dyn ImageCompleteUtilDyn>>> = ImageCompleteUtil::new().map(|reference| reference as Rc<RefCell<dyn ImageCompleteUtilDyn>>); }

impl ImageCompleteUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            ISREADY: String::new(),
            COMPLETE: String::new(),
        };
        this.ISREADY = String::from("isReady");
        this.COMPLETE = String::from("Image loading complete for: ");
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
    }
    

    // throws Exception
    pub fn isReady(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut name: String, mut timeDelayHelper: Option<Rc<RefCell<TimeDelayHelper>>>) -> bool {
        let mut playnImage: Option<Rc<RefCell<PlaynImage>>> = (image).clone() /* as PlaynImage */;
        let mut playnCoreImage: Option<Rc<RefCell<CoreImage>>> = (jcall!(playnImage).getImage()).clone() /* as CoreImage */;
        if !jcall!(playnCoreImage).isReady() || jcall!(playnCoreImage).width() + jcall!(playnCoreImage).height() <= 0 {
            self.handleTimeout(name.clone());
            return false;
        }
        if !jcall!(image).isReady() {
            jcall!(image).init(jcall!(image).getImage());
            if !jcall!(image).isReady() {
                PreLogUtil::put(format!("{}{}", String::from("Waiting for Not Ready Image: "), jcall!(image).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.ISREADY.clone());
            }
        }
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        PreLogUtil::put(format!("{}{}", self.COMPLETE, jcall!(image).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).SUCCESS.clone());
        return true;
    }
    

    // throws Exception
    pub fn handleTimeout(&mut self, mut name: String) {
    }
    

    // throws Exception
    pub fn waitForAll(&mut self) {
    }
}
pub trait ImageCompleteUtilDyn {
    fn as_ImageCompleteUtil(&self) -> &ImageCompleteUtil;
    fn as_ImageCompleteUtil_mut(&mut self) -> &mut ImageCompleteUtil;
    fn waitForImage(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut name: String) { ImageCompleteUtil::waitForImage(self.as_ImageCompleteUtil_mut(), image, name) }
    fn isReady(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut name: String, mut timeDelayHelper: Option<Rc<RefCell<TimeDelayHelper>>>) -> bool { ImageCompleteUtil::isReady(self.as_ImageCompleteUtil_mut(), image, name, timeDelayHelper) }
    fn handleTimeout(&mut self, mut name: String) { ImageCompleteUtil::handleTimeout(self.as_ImageCompleteUtil_mut(), name) }
    fn waitForAll(&mut self) { ImageCompleteUtil::waitForAll(self.as_ImageCompleteUtil_mut()) }
}
impl ImageCompleteUtilDyn for ImageCompleteUtil {
    fn as_ImageCompleteUtil(&self) -> &ImageCompleteUtil { self }
    fn as_ImageCompleteUtil_mut(&mut self) -> &mut ImageCompleteUtil { self }
}



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
// package org.allbinary.graphics.displayable
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Displayable::Displayable;
use crate::org::allbinary::AndroidUtil::AndroidUtil;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::graphics::displayable::event::DisplayChangeEvent::DisplayChangeEvent;
use crate::org::allbinary::graphics::displayable::event::DisplayChangeEventHandler::DisplayChangeEventHandler;
use crate::org::allbinary::graphics::displayable::event::LastDisplayChangeEventHandler::LastDisplayChangeEventHandler;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureFactory::OpenGLFeatureFactory;
use crate::org::allbinary::graphics::threed::SWTJOGLProcessor::SWTJOGLProcessor;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem;
use crate::org::allbinary::logic::system::os::OperatingSystemFactory::OperatingSystemFactory;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::BaseScalable::BaseScalable;
use super::ScalableListener::ScalableListener;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DisplayInfoSingleton {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonLabels: Option<Rc<RefCell<CommonLabels>>>,
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub ORIENTATION: String,
    pub ADJUSTING_FOR_SCALING_IN_PORTRAIT: String,
    pub ADJUSTING_FOR_SCALING_IN_LANDSCAPE: String,
    pub last: Vec<i32>,
    pub lastHalf: Vec<i32>,
    pub full: Vec<i32>,
    pub top: i32,
    pub left: i32,
    pub xOffset: i32,
    pub yOffset: i32,
    pub scaleLargestTo: f32,
    pub WIDTH: i32,
    pub HEIGHT: i32,
    pub CUSTOM_WIDTH: i32,
    pub CUSTOM_HEIGHT: i32,
    pub scalableListener: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::BaseScalable::BaseScalableDyn>>>,
    pub displayRatio: f32,
    pub ratio: f32,
    pub SET_LAST_SIZE_METHOD_NAME: String,
    pub FULL_WIDTH: String,
    pub FULL_HEIGHT: String,
    pub LAST_WIDTH: String,
    pub LAST_HEIGHT: String,
    pub PORTRAIT: String,
    pub LANDSCAPE: String,
    pub displayChangeEvent: Option<Rc<RefCell<DisplayChangeEvent>>>,
    pub FIRE_METHOD_NAME: String,
    pub REASON: String,
    pub UPDATE_FROM_ORIENTATION_CHANGE: String,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub DISPLAY_INFO: String,
    pub FULL: String,
    pub LAST: String,
    pub LAST_HALF: String,
}

thread_local! { pub static DisplayInfoSingleton_SINGLETON: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::new(); }

impl DisplayInfoSingleton {
    

    pub fn getInstance() -> Option<Rc<RefCell<DisplayInfoSingleton>>> {
        return DisplayInfoSingleton_SINGLETON.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonLabels: None,
            commonSeps: None,
            commonStrings: None,
            ORIENTATION: String::new(),
            ADJUSTING_FOR_SCALING_IN_PORTRAIT: String::new(),
            ADJUSTING_FOR_SCALING_IN_LANDSCAPE: String::new(),
            last: Vec::new(),
            lastHalf: Vec::new(),
            full: Vec::new(),
            top: 0,
            left: 0,
            xOffset: 0,
            yOffset: 0,
            scaleLargestTo: 0.0_f32,
            WIDTH: 0,
            HEIGHT: 0,
            CUSTOM_WIDTH: 0,
            CUSTOM_HEIGHT: 0,
            scalableListener: None,
            displayRatio: 0.0_f32,
            ratio: 0.0_f32,
            SET_LAST_SIZE_METHOD_NAME: String::new(),
            FULL_WIDTH: String::new(),
            FULL_HEIGHT: String::new(),
            LAST_WIDTH: String::new(),
            LAST_HEIGHT: String::new(),
            PORTRAIT: String::new(),
            LANDSCAPE: String::new(),
            displayChangeEvent: None,
            FIRE_METHOD_NAME: String::new(),
            REASON: String::new(),
            UPDATE_FROM_ORIENTATION_CHANGE: String::new(),
            list: None,
            DISPLAY_INFO: String::new(),
            FULL: String::new(),
            LAST: String::new(),
            LAST_HALF: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonLabels = CommonLabels::getInstance();
        this.commonSeps = CommonSeps::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.ORIENTATION = String::from("ORIENTATION");
        this.ADJUSTING_FOR_SCALING_IN_PORTRAIT = String::from("Adjusting for Scaling in portrait display ratio: ");
        this.ADJUSTING_FOR_SCALING_IN_LANDSCAPE = String::from("Adjusting for Scaling in landscape display ratio: ");
        this.last = vec![0; (4) as usize];
        this.lastHalf = vec![0; (4) as usize];
        this.full = vec![0; (4) as usize];
        this.WIDTH = 0;
        this.HEIGHT = 1;
        this.CUSTOM_WIDTH = 2;
        this.CUSTOM_HEIGHT = 3;
        this.scalableListener = BaseScalable::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::BaseScalable::BaseScalableDyn>>);
        this.ratio = 1.0_f32;
        this.SET_LAST_SIZE_METHOD_NAME = String::from("setLastSize");
        this.FULL_WIDTH = String::from(" FullWidth: ");
        this.FULL_HEIGHT = String::from(" FullHeight: ");
        this.LAST_WIDTH = String::from("LastWidth: ");
        this.LAST_HEIGHT = String::from(" LastHeight: ");
        this.PORTRAIT = String::from("Portrait");
        this.LANDSCAPE = String::from("Landscape");
        this.displayChangeEvent = DisplayChangeEvent::new(this);
        this.FIRE_METHOD_NAME = String::from("fire");
        this.REASON = String::from("reason: ");
        this.UPDATE_FROM_ORIENTATION_CHANGE = String::from("Updating from Orientation Change ");
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.DISPLAY_INFO = String::from("Display Info: ");
        this.FULL = String::from("full");
        this.LAST = String::from("last");
        this.LAST_HALF = String::from("lastHalf");
        let mut size: i32 = (this.last.len() as i32).clone();
        for index in (0..size) {
            this.last[(index) as usize] = 0;
            this.lastHalf[(index) as usize] = 0;
            this.full[(index) as usize] = 0;
        }
        if AndroidUtil::isAndroid() {
            this.scaleLargestTo = 640.0_f32;
        } else {
            this.scaleLargestTo = 1080.0_f32;
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getLastHalf(&mut self) -> Vec<i32> {
        return self.lastHalf.clone();
    }
    

    pub fn getLast(&mut self) -> Vec<i32> {
        return self.last.clone();
    }
    

    pub fn getFull(&mut self) -> Vec<i32> {
        return self.full.clone();
    }
    

    pub fn getScaleLargestTo(&mut self) -> f32 {
        return self.scaleLargestTo.clone();
    }
    

    pub fn setScaleLargestTo(&mut self, mut scaleLargestTo: i32) {
        self.scaleLargestTo = ((scaleLargestTo) as f32);
    }
    

    pub fn setLastSize(&mut self, mut aLastWidth: i32, mut aLastHeight: i32, mut reason: String) {
        if self.full[(self.WIDTH) as usize] != aLastWidth || self.full[(self.HEIGHT) as usize] != aLastHeight {
            self.setLastSizeForce(aLastWidth.clone(), aLastHeight.clone(), reason.clone());
        }
    }
    

    pub fn setLastSizeForce(&mut self, mut aLastWidth: i32, mut aLastHeight: i32, mut reason: String) {
        let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringMaker).append(jcall!(CommonLabels::getInstance()).START_LABEL.clone()).append(reason.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.SET_LAST_SIZE_METHOD_NAME.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut aFullWidth: i32 = aLastWidth.clone();
        let mut aFullHeight: i32 = aLastHeight.clone();
        jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringMaker).append(self.FULL_WIDTH.clone()).appendint(aFullWidth.clone()).append(self.FULL_HEIGHT.clone()).appendint(aFullHeight.clone()).append(self.toString()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.SET_LAST_SIZE_METHOD_NAME.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut operatingSystemInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>> = jcall!(OperatingSystemFactory::getInstance()).getOperatingSystemInstance();
        if jcall!(operatingSystemInterface).isOverScan() {
            aLastWidth = aLastWidth * jcall!(operatingSystemInterface).getOverScanXPercent() / 100;
            aLastHeight = aLastHeight * jcall!(operatingSystemInterface).getOverScanYPercent() / 100;
        }
        if jcall!(operatingSystemInterface).isScalable() {
            if self.isPortraitWH(aLastWidth.clone(), aLastHeight.clone()) {
                if ((aLastHeight) as f32) > self.scaleLargestTo {
                    self.displayRatio = self.scaleLargestTo / ((aLastHeight) as f32);
                    self.ratio = ((aLastHeight) as f32) / self.scaleLargestTo;
                    jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringMaker).append(self.ADJUSTING_FOR_SCALING_IN_PORTRAIT.clone()).appendfloat(self.displayRatio.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.SET_LAST_SIZE_METHOD_NAME.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    aLastWidth = (((((aLastWidth) as f32) * self.displayRatio)) as i32);
                    aLastHeight = (((((aLastHeight) as f32) * self.displayRatio)) as i32);
                    jcall!(self.scalableListener).scale(self.ratio);
                } else {
                    self.ratio = 1.0_f32;
                    jcall!(self.scalableListener).scale(self.ratio);
                }
            } else {
                if ((aLastWidth) as f32) > self.scaleLargestTo {
                    self.displayRatio = self.scaleLargestTo / ((aLastWidth) as f32);
                    self.ratio = ((aLastWidth) as f32) / self.scaleLargestTo;
                    jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringMaker).append(self.ADJUSTING_FOR_SCALING_IN_LANDSCAPE.clone()).appendfloat(self.displayRatio.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.SET_LAST_SIZE_METHOD_NAME.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    aLastWidth = (((((aLastWidth) as f32) * self.displayRatio)) as i32);
                    aLastHeight = (((((aLastHeight) as f32) * self.displayRatio)) as i32);
                    jcall!(self.scalableListener).scale(self.ratio);
                } else {
                    self.ratio = 1.0_f32;
                    jcall!(self.scalableListener).scale(self.ratio);
                }
            }
        }
        jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringMaker).append(self.LAST_WIDTH.clone()).appendint(aLastWidth.clone()).append(self.LAST_HEIGHT.clone()).appendint(aLastHeight.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.SET_LAST_SIZE_METHOD_NAME.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        self.xOffset = aFullWidth - aLastWidth;
        self.yOffset = aFullHeight - aLastHeight;
        self.left = jcall!(self.scalableListener).getLeft(self.xOffset.clone());
        self.top = jcall!(self.scalableListener).getTop(self.yOffset.clone());
        self.full[(self.WIDTH) as usize] = aFullWidth.clone();
        self.full[(self.HEIGHT) as usize] = aFullHeight.clone();
        self.last[(self.WIDTH) as usize] = aLastWidth.clone();
        self.lastHalf[(self.WIDTH) as usize] = (self.last[(self.WIDTH) as usize] >> 1);
        self.last[(self.HEIGHT) as usize] = aLastHeight.clone();
        self.lastHalf[(self.HEIGHT) as usize] = (self.last[(self.HEIGHT) as usize] >> 1);
        jcall!(SWTJOGLProcessor::getInstance()).setCustom(aLastWidth.clone(), aLastHeight.clone(), self.ratio.clone());
        self.add(self.SET_LAST_SIZE_METHOD_NAME.clone());
    }
    

    pub fn isPortraitWH(&mut self, mut lastWidth: i32, mut lastHeight: i32) -> bool {
        if lastHeight > lastWidth {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn isPortrait(&mut self) -> bool {
        return self.isPortraitWH(self.last[(self.WIDTH) as usize].clone(), self.last[(self.HEIGHT) as usize].clone());
    }
    

    pub fn add(&mut self, mut reason: String) {
        let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringMaker).append(self.REASON.clone()).append(reason.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.FIRE_METHOD_NAME.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (self.toStringAppend(stringMaker.clone()), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.FIRE_METHOD_NAME.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        jcall!(self.list).add(crate::java_into_object(reason.clone()));
    }
    

    pub fn process(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(self.list).size() > 0 {
                self.processForced();
            }
            jcall!(self.list).clear();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            PreLogUtil::putOE(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.FIRE_METHOD_NAME.clone(), e.clone());
            }
        }
    }
    

    pub fn processForced(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut swtJOGLProcessor: Option<Rc<RefCell<SWTJOGLProcessor>>> = SWTJOGLProcessor::getInstance();
            jcall!(swtJOGLProcessor).clear();
            jcall!(DisplayChangeEventHandler::getInstance()).fireEvent(self.displayChangeEvent.clone());
            jcall!(LastDisplayChangeEventHandler::getInstance()).fireEvent(self.displayChangeEvent.clone());
            jcall!(swtJOGLProcessor).onSurfaceChanged();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            PreLogUtil::putOE(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.FIRE_METHOD_NAME.clone(), e.clone());
            }
        }
    }
    

    pub fn update(&mut self, mut displayable: Option<Rc<RefCell<Displayable>>>, mut reason: String) {
        let mut aLastWidth: i32 = jcall!(displayable).getWidth();
        let mut aLastHeight: i32 = jcall!(displayable).getHeight();
        let mut aFullWidth: i32 = aLastWidth.clone();
        let mut aFullHeight: i32 = aLastHeight.clone();
        let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringMaker).append(jcall!(CommonLabels::getInstance()).START_LABEL.clone()).append(reason.clone()).append(self.LAST_WIDTH.clone()).appendint(aLastWidth.clone()).append(self.LAST_HEIGHT.clone()).appendint(aLastHeight.clone()).append(jcall!(self.commonSeps).SPACE.clone()).append(self.toString()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).UPDATE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        if aLastWidth > 0 && aLastHeight > 0 {
            let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
            let mut openGLFeatureFactory: Option<Rc<RefCell<OpenGLFeatureFactory>>> = OpenGLFeatureFactory::getInstance();
            if !jcall!(features).isDefault(jcall!(openGLFeatureFactory).OPENGL.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) || self.last[(self.WIDTH) as usize] != aLastWidth || self.last[(self.HEIGHT) as usize] != aLastHeight {
                jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringMaker).append(self.UPDATE_FROM_ORIENTATION_CHANGE.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).UPDATE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                let mut operatingSystemInterface: Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>> = jcall!(OperatingSystemFactory::getInstance()).getOperatingSystemInstance();
                if jcall!(operatingSystemInterface).isOverScan() {
                    aLastWidth = aLastWidth * jcall!(operatingSystemInterface).getOverScanXPercent() / 100;
                    aLastHeight = aLastHeight * jcall!(operatingSystemInterface).getOverScanYPercent() / 100;
                }
                if jcall!(operatingSystemInterface).isScalable() {
                    if self.isPortraitWH(aLastWidth.clone(), aLastHeight.clone()) {
                        if ((aLastHeight) as f32) > self.scaleLargestTo {
                            self.displayRatio = self.scaleLargestTo / ((aLastHeight) as f32);
                            self.ratio = ((aLastHeight) as f32) / self.scaleLargestTo;
                            jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringMaker).append(self.ADJUSTING_FOR_SCALING_IN_PORTRAIT.clone()).appendfloat(self.displayRatio.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).UPDATE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                            aLastWidth = (((((aLastWidth) as f32) * self.displayRatio)) as i32);
                            aLastHeight = (((((aLastHeight) as f32) * self.displayRatio)) as i32);
                            jcall!(self.scalableListener).scale(self.ratio);
                        } else {
                            self.ratio = 1.0_f32;
                            jcall!(self.scalableListener).scale(self.ratio);
                        }
                    } else {
                        if ((aLastWidth) as f32) > self.scaleLargestTo {
                            self.displayRatio = self.scaleLargestTo / ((aLastWidth) as f32);
                            self.ratio = ((aLastWidth) as f32) / self.scaleLargestTo;
                            jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringMaker).append(self.ADJUSTING_FOR_SCALING_IN_LANDSCAPE.clone()).appendfloat(self.displayRatio.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).UPDATE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                            aLastWidth = (((((aLastWidth) as f32) * self.displayRatio)) as i32);
                            aLastHeight = (((((aLastHeight) as f32) * self.displayRatio)) as i32);
                            jcall!(self.scalableListener).scale(self.ratio);
                        } else {
                            self.ratio = 1.0_f32;
                            jcall!(self.scalableListener).scale(self.ratio);
                        }
                    }
                }
                jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringMaker).append(self.UPDATE_FROM_ORIENTATION_CHANGE.clone()).append(self.LAST_WIDTH.clone()).appendint(aLastWidth.clone()).append(self.LAST_HEIGHT.clone()).appendint(aLastHeight.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).UPDATE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                self.xOffset = aFullWidth - aLastWidth;
                self.yOffset = aFullHeight - aLastHeight;
                self.left = jcall!(self.scalableListener).getLeft(self.xOffset.clone());
                self.top = jcall!(self.scalableListener).getTop(self.yOffset.clone());
                self.full[(self.WIDTH) as usize] = aFullWidth.clone();
                self.full[(self.HEIGHT) as usize] = aFullHeight.clone();
                self.last[(self.WIDTH) as usize] = aLastWidth.clone();
                self.lastHalf[(self.WIDTH) as usize] = (self.last[(self.WIDTH) as usize] >> 1);
                self.last[(self.HEIGHT) as usize] = aLastHeight.clone();
                self.lastHalf[(self.HEIGHT) as usize] = (self.last[(self.HEIGHT) as usize] >> 1);
                jcall!(SWTJOGLProcessor::getInstance()).setCustom(aLastWidth.clone(), aLastHeight.clone(), self.ratio.clone());
                self.add(jcall!(self.commonStrings).UPDATE.clone());
            }
        }
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        return self.toStringAppend(stringBuffer.clone());
    }
    

    pub fn toStringAppend(&mut self, mut stringBuffer: Option<Rc<RefCell<StringMaker>>>) -> String {
        jcall!(stringBuffer).append(self.DISPLAY_INFO.clone());
        jcall!(stringBuffer).append(self.FULL.clone());
        jcall!(stringBuffer).append(jcall!(self.commonLabels).WIDTH_LABEL.clone());
        jcall!(stringBuffer).appendint(self.full[(self.WIDTH) as usize].clone());
        jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(self.FULL.clone());
        jcall!(stringBuffer).append(jcall!(self.commonLabels).HEIGHT_LABEL.clone());
        jcall!(stringBuffer).appendint(self.full[(self.HEIGHT) as usize].clone());
        jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(self.LAST.clone());
        jcall!(stringBuffer).append(jcall!(self.commonLabels).WIDTH_LABEL.clone());
        jcall!(stringBuffer).appendint(self.last[(self.WIDTH) as usize].clone());
        jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(self.LAST.clone());
        jcall!(stringBuffer).append(jcall!(self.commonLabels).HEIGHT_LABEL.clone());
        jcall!(stringBuffer).appendint(self.last[(self.HEIGHT) as usize].clone());
        jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(self.LAST_HALF.clone());
        jcall!(stringBuffer).append(jcall!(self.commonLabels).WIDTH_LABEL.clone());
        jcall!(stringBuffer).appendint(self.lastHalf[(self.WIDTH) as usize].clone());
        jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(self.LAST_HALF.clone());
        jcall!(stringBuffer).append(jcall!(self.commonLabels).HEIGHT_LABEL.clone());
        jcall!(stringBuffer).appendint(self.lastHalf[(self.HEIGHT) as usize].clone());
        jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
        jcall!(stringBuffer).append((if self.isPortrait() { self.PORTRAIT } else { self.LANDSCAPE }));
        return jcall!(stringBuffer).toString();
    }
    

    pub fn getTop(&mut self) -> i32 {
        return self.top.clone();
    }
    

    pub fn getLeft(&mut self) -> i32 {
        return self.left.clone();
    }
    

    pub fn getLastWidth(&mut self) -> i32 {
        return self.last[(self.WIDTH) as usize].clone();
    }
    

    pub fn getLastHeight(&mut self) -> i32 {
        return self.last[(self.HEIGHT) as usize].clone();
    }
    

    pub fn getLastHalfWidth(&mut self) -> i32 {
        return self.lastHalf[(self.WIDTH) as usize].clone();
    }
    

    pub fn getLastHalfHeight(&mut self) -> i32 {
        return self.lastHalf[(self.HEIGHT) as usize].clone();
    }
    

    pub fn getCustomLastWidth(&mut self) -> i32 {
        return self.last[(self.CUSTOM_WIDTH) as usize].clone();
    }
    

    pub fn getCustomLastHeight(&mut self) -> i32 {
        return self.last[(self.CUSTOM_HEIGHT) as usize].clone();
    }
    

    pub fn getRawLastWidth(&mut self) -> i32 {
        return self.last[(self.WIDTH) as usize].clone();
    }
    

    pub fn getRawLastHeight(&mut self) -> i32 {
        return self.last[(self.HEIGHT) as usize].clone();
    }
    

    pub fn setCustom(&mut self, mut width: i32, mut height: i32) {
        self.last[(self.CUSTOM_WIDTH) as usize] = width.clone();
        self.lastHalf[(self.CUSTOM_WIDTH) as usize] = (self.last[(self.CUSTOM_WIDTH) as usize] >> 1);
        self.last[(self.CUSTOM_HEIGHT) as usize] = height.clone();
        self.lastHalf[(self.CUSTOM_HEIGHT) as usize] = (self.last[(self.CUSTOM_HEIGHT) as usize] >> 1);
    }
    

    pub fn setScalableListener(&mut self, mut scalableListener: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::BaseScalable::BaseScalableDyn>>>) {
        self.scalableListener = scalableListener.clone();
    }
    

    pub fn getScalableListener(&mut self) -> Option<Rc<RefCell<dyn ScalableListener>>> {
        return self.scalableListener.clone().map(|reference| reference as Rc<RefCell<dyn ScalableListener>>);
    }
    

    pub fn getRatio(&mut self) -> f32 {
        return self.ratio.clone();
    }
    

    pub fn getDisplayRatio(&mut self) -> f32 {
        return self.displayRatio.clone();
    }
    

    pub fn setOffset(&mut self, mut left: i32, mut top: i32) {
        self.left = left.clone();
    }
    

    pub fn getxOffset(&mut self) -> i32 {
        return self.xOffset.clone();
    }
    

    pub fn getyOffset(&mut self) -> i32 {
        return self.yOffset.clone();
    }
}


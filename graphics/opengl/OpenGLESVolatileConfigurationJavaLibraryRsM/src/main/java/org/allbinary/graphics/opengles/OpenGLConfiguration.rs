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
// package org.allbinary.graphics.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::game::configuration::event::ChangedGameFeatureListener::ChangedGameFeatureListener;
use crate::org::allbinary::game::configuration::feature::Feature::Feature;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::MainFeatureFactory::MainFeatureFactory;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::OpenGLFeature::OpenGLFeature;
use super::OpenGLFeatureFactory::OpenGLFeatureFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLConfiguration {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub opengl: bool,
    pub r#type: Option<Rc<RefCell<OpenGLFeature>>>,
    pub imageColor: Option<Rc<RefCell<OpenGLFeature>>>,
    pub color: Option<Rc<RefCell<OpenGLFeature>>>,
    pub versionSelector: Option<Rc<RefCell<OpenGLFeature>>>,
}

pub static mut OpenGLConfiguration_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl OpenGLConfiguration {
    

    pub fn getInstance() -> Option<Rc<RefCell<OpenGLConfiguration>>> {
        if match (&unsafe { OpenGLConfiguration_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *OpenGLConfiguration_instance = OpenGLConfiguration::new() };
        }
        return (unsafe { OpenGLConfiguration_instance.clone() }).clone() /* as OpenGLConfiguration */;
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            opengl: false,
            r#type: None,
            imageColor: None,
            color: None,
            versionSelector: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.opengl = false;
        this.r#type = jcall!(OpenGLFeatureFactory::getInstance()).OPENGL_AS_GAME_THREAD.clone();
        this.imageColor = jcall!(OpenGLFeatureFactory::getInstance()).IMAGE_COLOR_DEPTH_4444.clone();
        this.color = jcall!(OpenGLFeatureFactory::getInstance()).IMAGE_COLOR_DEPTH_4444.clone();
        this.versionSelector = jcall!(OpenGLFeatureFactory::getInstance()).OPENGL_AUTO_SELECT.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn write(&mut self) {
    }
    

    // throws Exception
    pub fn init(&mut self) {
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        if jcall!(ChangedGameFeatureListener::getInstance()).isChangedFeature(jcall!(MainFeatureFactory::getInstance()).STATIC.clone()) {
            if self.isOpenGL() {
                if !jcall!(features).isDefault(jcall!(OpenGLFeatureFactory::getInstance()).OPENGL.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
                    jcall!(self.logUtil).putF(String::from("Turning on OpenGL"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone());
                    jcall!(features).addDefault(jcall!(OpenGLFeatureFactory::getInstance()).OPENGL.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Using OpenGL Type Feature: "), self.getType()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    jcall!(features).addDefault(self.getType().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Using OpenGL ImageColor Feature: "), self.getImageColor()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    jcall!(features).addDefault(self.getImageColor().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Using OpenGL Color Feature: "), self.getColor()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    jcall!(features).addDefault(self.getColor().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Using OpenGL Version Selector Feature: "), self.getVersionSelector()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    jcall!(features).addDefault(self.getVersionSelector().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>));
                }
            } else {
                jcall!(self.logUtil).putF(String::from("OpenGL is Off"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone());
            }
        } else {
            if self.isOpenGL() && !jcall!(features).isDefault(jcall!(OpenGLFeatureFactory::getInstance()).OPENGL.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
                PreLogUtil::put(String::from("OpenGL is set but not Enabled since Statics where not cleared (Reboot or Complete exit is Required)"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone());
            }
        }
        PreLogUtil::put(self.toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone());
    }
    

    // throws Exception
    pub fn update(&mut self, mut gameFeature: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>, mut colorLocked: bool) {
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        let mut openGLFeatureFactory: Option<Rc<RefCell<OpenGLFeatureFactory>>> = OpenGLFeatureFactory::getInstance();
        let mut modified: bool = false;
        if match (&gameFeature, &jcall!(openGLFeatureFactory).OPENGL) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if jcall!(features).isFeature(gameFeature.clone()) {
                if !self.isOpenGL() {
                    self.setOpenGL(true);
                    modified = true;
                }
            } else {
                if self.isOpenGL() {
                    self.setOpenGL(false);
                    modified = true;
                }
            }
        }
        if match (&gameFeature, &jcall!(openGLFeatureFactory).OPENGL_AND_GAME_HAVE_DIFFERENT_THREADS) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&gameFeature, &jcall!(openGLFeatureFactory).OPENGL_AS_GAME_THREAD) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if jcall!(features).isFeature(gameFeature.clone()) {
                if !match (&gameFeature, &self.getType()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    self.setType((gameFeature).clone() /* as OpenGLFeature */);
                    modified = true;
                }
            }
        }
        if match (&gameFeature, &jcall!(openGLFeatureFactory).IMAGE_COLOR_DEPTH_4444) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&gameFeature, &jcall!(openGLFeatureFactory).IMAGE_COLOR_DEPTH_565) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&gameFeature, &jcall!(openGLFeatureFactory).IMAGE_COLOR_DEPTH_8888) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if jcall!(features).isFeature(gameFeature.clone()) {
                if !match (&self.getImageColor(), &gameFeature) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    self.setImageColor((gameFeature).clone() /* as OpenGLFeature */);
                    if colorLocked {
                        if match (&gameFeature, &jcall!(openGLFeatureFactory).IMAGE_COLOR_DEPTH_4444) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                            self.setColor(jcall!(openGLFeatureFactory).OPENGL_COLOR_DEPTH_4444.clone());
                        } else if match (&gameFeature, &jcall!(openGLFeatureFactory).IMAGE_COLOR_DEPTH_565) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                            self.setColor(jcall!(openGLFeatureFactory).OPENGL_COLOR_DEPTH_565.clone());
                        } else if match (&gameFeature, &jcall!(openGLFeatureFactory).IMAGE_COLOR_DEPTH_8888) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                            self.setColor(jcall!(openGLFeatureFactory).OPENGL_COLOR_DEPTH_8888.clone());
                        }
                    }
                    modified = true;
                }
            }
        }
        if match (&gameFeature, &jcall!(openGLFeatureFactory).OPENGL_COLOR_DEPTH_4444) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&gameFeature, &jcall!(openGLFeatureFactory).OPENGL_COLOR_DEPTH_565) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&gameFeature, &jcall!(openGLFeatureFactory).OPENGL_COLOR_DEPTH_8888) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if jcall!(features).isFeature(gameFeature.clone()) {
                if !match (&self.getColor(), &gameFeature) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    self.setColor((gameFeature).clone() /* as OpenGLFeature */);
                    modified = true;
                }
            }
        }
        if match (&gameFeature, &jcall!(openGLFeatureFactory).OPENGL_AUTO_SELECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&gameFeature, &jcall!(openGLFeatureFactory).OPENGL_MINIMUM) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if jcall!(features).isFeature(gameFeature.clone()) {
                self.setVersionSelector((gameFeature).clone() /* as OpenGLFeature */);
            }
            modified = true;
        }
    }
    

    pub fn setOpenGL(&mut self, mut opengl: bool) {
        self.opengl = opengl.clone();
    }
    

    pub fn isOpenGL(&mut self) -> bool {
        return self.opengl.clone();
    }
    

    pub fn setImageColor(&mut self, mut imageColor: Option<Rc<RefCell<OpenGLFeature>>>) {
        self.imageColor = imageColor.clone();
    }
    

    pub fn getImageColor(&mut self) -> Option<Rc<RefCell<OpenGLFeature>>> {
        return self.imageColor.clone();
    }
    

    pub fn setColor(&mut self, mut color: Option<Rc<RefCell<OpenGLFeature>>>) {
        self.color = color.clone();
    }
    

    pub fn getColor(&mut self) -> Option<Rc<RefCell<OpenGLFeature>>> {
        return self.color.clone();
    }
    

    pub fn setVersionSelector(&mut self, mut versionSelector: Option<Rc<RefCell<OpenGLFeature>>>) {
        self.versionSelector = versionSelector.clone();
    }
    

    pub fn getVersionSelector(&mut self) -> Option<Rc<RefCell<OpenGLFeature>>> {
        return self.versionSelector.clone();
    }
    

    pub fn setType(&mut self, mut r#type: Option<Rc<RefCell<OpenGLFeature>>>) {
        self.r#type = r#type.clone();
    }
    

    pub fn getType(&mut self) -> Option<Rc<RefCell<OpenGLFeature>>> {
        return self.r#type.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        jcall!(stringBuffer).append(String::from(" isOpenGL: "));
        jcall!(stringBuffer).appendboolean(self.isOpenGL());
        jcall!(stringBuffer).append(String::from(" VersionSelector: "));
        jcall!(stringBuffer).append(jcall!(stringUtil).toString(crate::java_into_object(self.getVersionSelector())));
        jcall!(stringBuffer).append(String::from(" Type: "));
        jcall!(stringBuffer).append(jcall!(stringUtil).toString(crate::java_into_object(self.getType())));
        jcall!(stringBuffer).append(String::from(" Image Color: "));
        jcall!(stringBuffer).append(jcall!(stringUtil).toString(crate::java_into_object(self.getImageColor())));
        jcall!(stringBuffer).append(String::from(" Color: "));
        jcall!(stringBuffer).append(jcall!(stringUtil).toString(crate::java_into_object(self.getColor())));
        return jcall!(stringBuffer).toString();
    }
}


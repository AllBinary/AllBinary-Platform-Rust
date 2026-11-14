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
// package org.allbinary.input.motion.button
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureFactory::OpenGLFeatureFactory;
use crate::org::allbinary::image::opengles::OpenGLImageCacheFactory::OpenGLImageCacheFactory;
use crate::java::lang::Exception::*;
use super::TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory::TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TouchButtonResourceOpenGLESAnimationInterfaceFactoryInterfaceFactory {
    pub base: TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory,
}

impl crate::org::allbinary::game::configuration::feature::GameFeatureControlledInterface::GameFeatureControlledInterface for TouchButtonResourceOpenGLESAnimationInterfaceFactoryInterfaceFactory {
    fn init(&mut self, mut level: i32) { Self::init(self, level) }
    fn isFeature(&mut self) -> bool { Self::isFeature(self) }
    fn isLoadingLevel(&mut self, p1: i32) -> bool { crate::org::allbinary::input::motion::button::TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory::TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory::isLoadingLevel(crate::org::allbinary::input::motion::button::TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory::TouchButtonResourceAnimationInterfaceFactoryInterfaceFactoryDyn::as_TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory_mut(self), p1) }
}
impl crate::org::allbinary::animation::resource::ResourceAnimationInterfaceFactoryInterface::ResourceAnimationInterfaceFactoryInterface for TouchButtonResourceOpenGLESAnimationInterfaceFactoryInterfaceFactory {
    fn getBasicAnimationInterfaceFactoryInstance(&mut self, p1: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface>>> { crate::org::allbinary::animation::resource::BaseResourceAnimationInterfaceFactoryInterfaceFactory::BaseResourceAnimationInterfaceFactoryInterfaceFactory::getBasicAnimationInterfaceFactoryInstance(crate::org::allbinary::animation::resource::BaseResourceAnimationInterfaceFactoryInterfaceFactory::BaseResourceAnimationInterfaceFactoryInterfaceFactoryDyn::as_BaseResourceAnimationInterfaceFactoryInterfaceFactory_mut(self), p1) }
}

impl TouchButtonResourceOpenGLESAnimationInterfaceFactoryInterfaceFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory::new_struct(String::from("OpenGL TouchButton Animations")),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn init(&mut self, mut level: i32) {
        { let java_arguments = (OpenGLImageCacheFactory::getInstance(), level.clone(),); self.base.initImageCache(java_arguments.0, java_arguments.1) };
    }
    

    pub fn isFeature(&mut self) -> bool {
        if jcall!(Features::getInstance()).isDefault({ let java_field = jcall!(OpenGLFeatureFactory::getInstance()).OPENGL.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            return true;
        } else {
            return false;
        }
    }
}
impl std::ops::Deref for TouchButtonResourceOpenGLESAnimationInterfaceFactoryInterfaceFactory {
    type Target = TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TouchButtonResourceOpenGLESAnimationInterfaceFactoryInterfaceFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::motion::button::TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory::TouchButtonResourceAnimationInterfaceFactoryInterfaceFactoryDyn for TouchButtonResourceOpenGLESAnimationInterfaceFactoryInterfaceFactory {
    fn as_TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory(&self) -> &crate::org::allbinary::input::motion::button::TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory::TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory { &self.base }
    fn as_TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory_mut(&mut self) -> &mut crate::org::allbinary::input::motion::button::TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory::TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory { &mut self.base }
}
impl crate::org::allbinary::animation::resource::BaseResourceAnimationInterfaceFactoryInterfaceFactory::BaseResourceAnimationInterfaceFactoryInterfaceFactoryDyn for TouchButtonResourceOpenGLESAnimationInterfaceFactoryInterfaceFactory {
    fn as_BaseResourceAnimationInterfaceFactoryInterfaceFactory(&self) -> &crate::org::allbinary::animation::resource::BaseResourceAnimationInterfaceFactoryInterfaceFactory::BaseResourceAnimationInterfaceFactoryInterfaceFactory { &self.base.base }
    fn as_BaseResourceAnimationInterfaceFactoryInterfaceFactory_mut(&mut self) -> &mut crate::org::allbinary::animation::resource::BaseResourceAnimationInterfaceFactoryInterfaceFactory::BaseResourceAnimationInterfaceFactoryInterfaceFactory { &mut self.base.base }
    fn init(&mut self, mut level: i32) { Self::init(self, level) }
    fn initImageCache(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>, p2: i32) { crate::org::allbinary::input::motion::button::TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory::TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory::initImageCache(crate::org::allbinary::input::motion::button::TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory::TouchButtonResourceAnimationInterfaceFactoryInterfaceFactoryDyn::as_TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory_mut(self), p1, p2) }
    fn isFeature(&mut self) -> bool { Self::isFeature(self) }
    fn isLoadingLevel(&mut self, p1: i32) -> bool { crate::org::allbinary::input::motion::button::TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory::TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory::isLoadingLevel(crate::org::allbinary::input::motion::button::TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory::TouchButtonResourceAnimationInterfaceFactoryInterfaceFactoryDyn::as_TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory_mut(self), p1) }
}



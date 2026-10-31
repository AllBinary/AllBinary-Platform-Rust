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
use crate::org::allbinary::animation::resource::BaseResourceAnimationInterfaceFactoryInterfaceFactory::BaseResourceAnimationInterfaceFactoryInterfaceFactory;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::resource::ResourceLoadingLevelFactory::ResourceLoadingLevelFactory;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureFactory::OpenGLFeatureFactory;
use crate::org::allbinary::image::ImageCache::ImageCache;
use crate::org::allbinary::image::ImageCacheFactory::ImageCacheFactory;
use crate::org::allbinary::logic::StdUtil::StdUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory {
    pub base: BaseResourceAnimationInterfaceFactoryInterfaceFactory,
}

impl crate::org::allbinary::game::configuration::feature::GameFeatureControlledInterface::GameFeatureControlledInterface for TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory {
    fn init(&mut self, mut level: i32) { Self::init(self, level) }
    fn isFeature(&mut self) -> bool { Self::isFeature(self) }
    fn isLoadingLevel(&mut self, mut level: i32) -> bool { Self::isLoadingLevel(self, level) }
}

impl TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory {
    

    pub fn createFactory() -> Option<Rc<RefCell<dyn TouchButtonResourceAnimationInterfaceFactoryInterfaceFactoryDyn>>> {
        return TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory::new(String::from("TouchButton Animations")).map(|reference| reference as Rc<RefCell<dyn TouchButtonResourceAnimationInterfaceFactoryInterfaceFactoryDyn>>);
    }
    

    pub fn new_struct(mut name: String) -> Self {
        let mut this = Self {
            base: BaseResourceAnimationInterfaceFactoryInterfaceFactory::new_struct(name.clone(), jcall!(StdUtil::getInstance()).createHashtable(), jcall!(StdUtil::getInstance()).createHashtable(), jcall!(StdUtil::getInstance()).createHashtable()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name))))
    }
    

    // throws Exception
    pub fn init(&mut self, mut level: i32) {
        { let java_arguments = (ImageCacheFactory::getInstance(), level.clone(),); self.initImageCache(java_arguments.0, java_arguments.1) };
    }
    

    // throws Exception
    pub fn initImageCache(&mut self, mut imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>, mut level: i32) {
        if self.isInitialized() {
            return;
        }
        self.base.init(level.clone());
    }
    

    pub fn isLoadingLevel(&mut self, mut level: i32) -> bool {
        let mut resourceLoadingLevelFactory: Option<Rc<RefCell<ResourceLoadingLevelFactory>>> = ResourceLoadingLevelFactory::getInstance();
        if level == jcall!(jcall!(resourceLoadingLevelFactory).LOAD_TOUCH).getLevel() {
            return true;
        } else {
            return self.base.isLoadingLevel(level.clone());
        }
    }
    

    pub fn isFeature(&mut self) -> bool {
        if !jcall!(Features::getInstance()).isDefault(jcall!(OpenGLFeatureFactory::getInstance()).OPENGL.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            return true;
        } else {
            return false;
        }
    }
}
impl std::ops::Deref for TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory {
    type Target = BaseResourceAnimationInterfaceFactoryInterfaceFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait TouchButtonResourceAnimationInterfaceFactoryInterfaceFactoryDyn: crate::org::allbinary::animation::resource::BaseResourceAnimationInterfaceFactoryInterfaceFactory::BaseResourceAnimationInterfaceFactoryInterfaceFactoryDyn {
    fn as_TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory(&self) -> &TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory;
    fn as_TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory_mut(&mut self) -> &mut TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory;
}
impl TouchButtonResourceAnimationInterfaceFactoryInterfaceFactoryDyn for TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory {
    fn as_TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory(&self) -> &TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory { self }
    fn as_TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory_mut(&mut self) -> &mut TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory { self }
}

impl crate::org::allbinary::animation::resource::BaseResourceAnimationInterfaceFactoryInterfaceFactory::BaseResourceAnimationInterfaceFactoryInterfaceFactoryDyn for TouchButtonResourceAnimationInterfaceFactoryInterfaceFactory {
    fn as_BaseResourceAnimationInterfaceFactoryInterfaceFactory(&self) -> &crate::org::allbinary::animation::resource::BaseResourceAnimationInterfaceFactoryInterfaceFactory::BaseResourceAnimationInterfaceFactoryInterfaceFactory { &self.base }
    fn as_BaseResourceAnimationInterfaceFactoryInterfaceFactory_mut(&mut self) -> &mut crate::org::allbinary::animation::resource::BaseResourceAnimationInterfaceFactoryInterfaceFactory::BaseResourceAnimationInterfaceFactoryInterfaceFactory { &mut self.base }
    fn init(&mut self, mut level: i32) { Self::init(self, level) }
    fn initImageCache(&mut self, mut imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>, mut level: i32) { Self::initImageCache(self, imageCache, level) }
    fn isFeature(&mut self) -> bool { Self::isFeature(self) }
    fn isLoadingLevel(&mut self, mut level: i32) -> bool { Self::isLoadingLevel(self, level) }
}



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
// package org.allbinary.animation.resource
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface;
use crate::org::allbinary::animation::NullAnimationFactory::NullAnimationFactory;
use crate::org::allbinary::game::resource::ResourceLoadingLevelFactory::ResourceLoadingLevelFactory;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::RectangleFactory::RectangleFactory;
use crate::org::allbinary::image::ImageCache::ImageCache;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::FeatureResourceAnimationInterfaceFactoryInterface::FeatureResourceAnimationInterfaceFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BaseResourceAnimationInterfaceFactoryInterfaceFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
    pub rectangleHashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
    pub rectangleArrayOfArraysHashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
    pub name: String,
    pub initialized: bool,
}

impl FeatureResourceAnimationInterfaceFactoryInterface for BaseResourceAnimationInterfaceFactoryInterfaceFactory {
    fn getHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { Self::getHashtable(self) }
}
impl crate::org::allbinary::animation::resource::ResourceAnimationInterfaceFactoryInterface::ResourceAnimationInterfaceFactoryInterface for BaseResourceAnimationInterfaceFactoryInterfaceFactory {
    fn getBasicAnimationInterfaceFactoryInstance(&mut self, mut resource: String) -> Option<Rc<RefCell<dyn BasicAnimationInterfaceFactoryInterface>>> { Self::getBasicAnimationInterfaceFactoryInstance(self, resource) }
}
impl crate::org::allbinary::game::configuration::feature::GameFeatureControlledInterface::GameFeatureControlledInterface for BaseResourceAnimationInterfaceFactoryInterfaceFactory {
    fn init(&mut self, mut level: i32) { Self::init(self, level) }
    fn isFeature(&mut self) -> bool { Self::isFeature(self) }
    fn isLoadingLevel(&mut self, mut level: i32) -> bool { Self::isLoadingLevel(self, level) }
}
impl crate::org::allbinary::game::resource::RectangleResourceFactoryInterface::RectangleResourceFactoryInterface for BaseResourceAnimationInterfaceFactoryInterfaceFactory {
    fn getRectangle(&mut self, mut resource: String) -> Option<Rc<RefCell<Rectangle>>> { Self::getRectangle(self, resource) }
    fn getRectangleArrayOfArrays(&mut self, mut resource: String) -> Vec<Vec<Option<Rc<RefCell<Rectangle>>>>> { Self::getRectangleArrayOfArrays(self, resource) }
}

impl BaseResourceAnimationInterfaceFactoryInterfaceFactory {
    

    pub fn new_struct(mut name: String, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut rectangleHashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut rectangleArrayHashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            hashtable: None,
            rectangleHashtable: None,
            rectangleArrayOfArraysHashtable: None,
            name: String::new(),
            initialized: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.hashtable = hashtable.clone();
        this.rectangleHashtable = rectangleHashtable.clone();
        this.rectangleArrayOfArraysHashtable = rectangleArrayHashtable.clone();
        this.name = name.clone();
        this
    }

    pub fn new(mut name: String, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut rectangleHashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut rectangleArrayHashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, hashtable, rectangleHashtable, rectangleArrayHashtable))))
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    // throws Exception
    pub fn init(&mut self, mut level: i32) {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Available List of Animations: ")).append(jcall!(self.hashtable).toString()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).INIT.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        self.setInitialized(true);
    }
    

    // throws Exception
    pub fn initImageCache(&mut self, mut imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>, mut level: i32) {
    }
    

    // throws Exception
    pub fn add(&mut self, mut resource: String, mut animationInterfaceFactoryInterface: Option<Rc<RefCell<dyn BasicAnimationInterfaceFactoryInterface>>>) {
        if jcall!(self.hashtable).containsKey(crate::java_into_object((resource).clone() /* as Any */)) {
            panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(StringMaker::new()).append(String::from("Resource Already Created: ")).append(resource.clone()).toString())));
        }
        jcall!(self.hashtable).put(crate::java_into_object(resource.clone()), crate::java_into_object(animationInterfaceFactoryInterface.clone()));
    }
    

    // throws Exception
    pub fn getBasicAnimationInterfaceFactoryInstance(&mut self, mut resource: String) -> Option<Rc<RefCell<dyn BasicAnimationInterfaceFactoryInterface>>> {
        let mut basicAnimationInterfaceFactoryInterfaceCanBeNull: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.hashtable).get(crate::java_into_object(resource.clone()));
        if basicAnimationInterfaceFactoryInterfaceCanBeNull.is_none() {
            return unsafe { crate::org::allbinary::animation::NullAnimationFactory::NullAnimationFactory_NULL_NOT_FOR_USE_ANIMATION_FACTORY.clone() }.clone();
        }
        return (basicAnimationInterfaceFactoryInterfaceCanBeNull).clone() /* as BasicAnimationInterfaceFactoryInterface */;
    }
    

    // throws Exception
    pub fn getRectangle(&mut self, mut resource: String) -> Option<Rc<RefCell<Rectangle>>> {
        let mut rectangleCanBeNull: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.rectangleHashtable).get(crate::java_into_object(resource.clone()));
        if rectangleCanBeNull.is_none() {
            return crate::org::allbinary::graphics::RectangleFactory::RectangleFactory_SINGLETON.with(Clone::clone).clone();
        }
        return (rectangleCanBeNull).clone() /* as Rectangle */;
    }
    

    // throws Exception
    pub fn addRectangle(&mut self, mut resource: String, mut rectangle: Option<Rc<RefCell<Rectangle>>>) {
        jcall!(self.rectangleHashtable).put(crate::java_into_object(resource.clone()), crate::java_into_object(rectangle.clone()));
    }
    

    // throws Exception
    pub fn getRectangleArrayOfArrays(&mut self, mut resource: String) -> Vec<Vec<Option<Rc<RefCell<Rectangle>>>>> {
        return (jcall!(self.rectangleArrayOfArraysHashtable).get(crate::java_into_object(resource.clone()))).clone() /* as  */;
    }
    

    // throws Exception
    pub fn addRectangleArrayOfArrays(&mut self, mut resource: String, mut rectangleArrayOfArrays: Vec<Vec<Option<Rc<RefCell<Rectangle>>>>>) {
        jcall!(self.rectangleArrayOfArraysHashtable).put(crate::java_into_object(resource.clone()), crate::java_into_object(rectangleArrayOfArrays.clone()));
    }
    

    pub fn isFeature(&mut self) -> bool {
        return false;
    }
    

    pub fn isLoadingLevel(&mut self, mut level: i32) -> bool {
        let mut resourceLoadingLevelFactory: Option<Rc<RefCell<ResourceLoadingLevelFactory>>> = ResourceLoadingLevelFactory::getInstance();
        if level == jcall!(jcall!(resourceLoadingLevelFactory).LOAD_ALL).getLevel() {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(StringMaker::new()).append(std::any::type_name_of_val(&self).to_string()).append(jcall!(CommonSeps::getInstance()).SEMICOLON.clone()).append(jcall!(CommonSeps::getInstance()).SPACE.clone()).append(self.name.clone()).toString();
    }
    

    pub fn getHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return self.hashtable.clone();
    }
    

    pub fn getRectangleHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return self.rectangleHashtable.clone();
    }
    

    pub fn getRectangleArrayOfArraysHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return self.rectangleArrayOfArraysHashtable.clone();
    }
    

    pub fn setInitialized(&mut self, mut initialized: bool) {
        self.initialized = initialized.clone();
    }
    

    pub fn isInitialized(&mut self) -> bool {
        return self.initialized.clone();
    }
}
pub trait BaseResourceAnimationInterfaceFactoryInterfaceFactoryDyn {
    fn as_BaseResourceAnimationInterfaceFactoryInterfaceFactory(&self) -> &BaseResourceAnimationInterfaceFactoryInterfaceFactory;
    fn as_BaseResourceAnimationInterfaceFactoryInterfaceFactory_mut(&mut self) -> &mut BaseResourceAnimationInterfaceFactoryInterfaceFactory;
    fn getName(&mut self) -> String { BaseResourceAnimationInterfaceFactoryInterfaceFactory::getName(self.as_BaseResourceAnimationInterfaceFactoryInterfaceFactory_mut()) }
    fn init(&mut self, mut level: i32) { BaseResourceAnimationInterfaceFactoryInterfaceFactory::init(self.as_BaseResourceAnimationInterfaceFactoryInterfaceFactory_mut(), level) }
    fn initImageCache(&mut self, mut imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>, mut level: i32) { BaseResourceAnimationInterfaceFactoryInterfaceFactory::initImageCache(self.as_BaseResourceAnimationInterfaceFactoryInterfaceFactory_mut(), imageCache, level) }
    fn add(&mut self, mut resource: String, mut animationInterfaceFactoryInterface: Option<Rc<RefCell<dyn BasicAnimationInterfaceFactoryInterface>>>) { BaseResourceAnimationInterfaceFactoryInterfaceFactory::add(self.as_BaseResourceAnimationInterfaceFactoryInterfaceFactory_mut(), resource, animationInterfaceFactoryInterface) }
    fn getBasicAnimationInterfaceFactoryInstance(&mut self, mut resource: String) -> Option<Rc<RefCell<dyn BasicAnimationInterfaceFactoryInterface>>> { BaseResourceAnimationInterfaceFactoryInterfaceFactory::getBasicAnimationInterfaceFactoryInstance(self.as_BaseResourceAnimationInterfaceFactoryInterfaceFactory_mut(), resource) }
    fn getRectangle(&mut self, mut resource: String) -> Option<Rc<RefCell<Rectangle>>> { BaseResourceAnimationInterfaceFactoryInterfaceFactory::getRectangle(self.as_BaseResourceAnimationInterfaceFactoryInterfaceFactory_mut(), resource) }
    fn addRectangle(&mut self, mut resource: String, mut rectangle: Option<Rc<RefCell<Rectangle>>>) { BaseResourceAnimationInterfaceFactoryInterfaceFactory::addRectangle(self.as_BaseResourceAnimationInterfaceFactoryInterfaceFactory_mut(), resource, rectangle) }
    fn getRectangleArrayOfArrays(&mut self, mut resource: String) -> Vec<Vec<Option<Rc<RefCell<Rectangle>>>>> { BaseResourceAnimationInterfaceFactoryInterfaceFactory::getRectangleArrayOfArrays(self.as_BaseResourceAnimationInterfaceFactoryInterfaceFactory_mut(), resource) }
    fn addRectangleArrayOfArrays(&mut self, mut resource: String, mut rectangleArrayOfArrays: Vec<Vec<Option<Rc<RefCell<Rectangle>>>>>) { BaseResourceAnimationInterfaceFactoryInterfaceFactory::addRectangleArrayOfArrays(self.as_BaseResourceAnimationInterfaceFactoryInterfaceFactory_mut(), resource, rectangleArrayOfArrays) }
    fn isFeature(&mut self) -> bool { BaseResourceAnimationInterfaceFactoryInterfaceFactory::isFeature(self.as_BaseResourceAnimationInterfaceFactoryInterfaceFactory_mut()) }
    fn isLoadingLevel(&mut self, mut level: i32) -> bool { BaseResourceAnimationInterfaceFactoryInterfaceFactory::isLoadingLevel(self.as_BaseResourceAnimationInterfaceFactoryInterfaceFactory_mut(), level) }
    fn toString(&mut self) -> String { BaseResourceAnimationInterfaceFactoryInterfaceFactory::toString(self.as_BaseResourceAnimationInterfaceFactoryInterfaceFactory_mut()) }
    fn getHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { BaseResourceAnimationInterfaceFactoryInterfaceFactory::getHashtable(self.as_BaseResourceAnimationInterfaceFactoryInterfaceFactory_mut()) }
    fn getRectangleHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { BaseResourceAnimationInterfaceFactoryInterfaceFactory::getRectangleHashtable(self.as_BaseResourceAnimationInterfaceFactoryInterfaceFactory_mut()) }
    fn getRectangleArrayOfArraysHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { BaseResourceAnimationInterfaceFactoryInterfaceFactory::getRectangleArrayOfArraysHashtable(self.as_BaseResourceAnimationInterfaceFactoryInterfaceFactory_mut()) }
    fn setInitialized(&mut self, mut initialized: bool) { BaseResourceAnimationInterfaceFactoryInterfaceFactory::setInitialized(self.as_BaseResourceAnimationInterfaceFactoryInterfaceFactory_mut(), initialized) }
    fn isInitialized(&mut self) -> bool { BaseResourceAnimationInterfaceFactoryInterfaceFactory::isInitialized(self.as_BaseResourceAnimationInterfaceFactoryInterfaceFactory_mut()) }
}
impl BaseResourceAnimationInterfaceFactoryInterfaceFactoryDyn for BaseResourceAnimationInterfaceFactoryInterfaceFactory {
    fn as_BaseResourceAnimationInterfaceFactoryInterfaceFactory(&self) -> &BaseResourceAnimationInterfaceFactoryInterfaceFactory { self }
    fn as_BaseResourceAnimationInterfaceFactoryInterfaceFactory_mut(&mut self) -> &mut BaseResourceAnimationInterfaceFactoryInterfaceFactory { self }
}



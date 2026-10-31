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
// package org.allbinary.animation
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::animation::resource::FeatureResourceAnimationInterfaceFactoryInterface::FeatureResourceAnimationInterfaceFactoryInterface;
use crate::org::allbinary::game::configuration::GameConfigurationCentral::GameConfigurationCentral;
use crate::org::allbinary::game::resource::FeaturedResourceFactory::FeaturedResourceFactory;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::RectangleFactory::RectangleFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::ProceduralAnimationInterfaceFactoryInterface::ProceduralAnimationInterfaceFactoryInterface;
use super::AnimationInterfaceFactoryInterface::AnimationInterfaceFactoryInterface;
use super::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FeaturedAnimationInterfaceFactoryInterfaceFactory {
    pub base: FeaturedResourceFactory,
}

pub static mut FeaturedAnimationInterfaceFactoryInterfaceFactory_INSTANCE: std::cell::LazyCell<Option<Rc<RefCell<FeaturedAnimationInterfaceFactoryInterfaceFactory>>>> = std::cell::LazyCell::new(|| FeaturedAnimationInterfaceFactoryInterfaceFactory::new());

impl FeaturedAnimationInterfaceFactoryInterfaceFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: FeaturedResourceFactory::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<FeaturedAnimationInterfaceFactoryInterfaceFactory>>> {
        return unsafe { FeaturedAnimationInterfaceFactoryInterfaceFactory_INSTANCE.clone() }.clone();
    }
    

    // throws Exception
    pub fn getRectangle(&mut self, mut resource: String) -> Option<Rc<RefCell<Rectangle>>> {
        return self.getRectangleXY(resource.clone(), 0, 0);
    }
    

    // throws Exception
    pub fn getRectangleXY(&mut self, mut resource: String, mut x: i32, mut y: i32) -> Option<Rc<RefCell<Rectangle>>> {
        let mut pointFactory: Option<Rc<RefCell<PointFactory>>> = PointFactory::getInstance();
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getList();
        let mut scale: i32 = ((jcall!(jcall!(GameConfigurationCentral::getInstance()).SCALE).getValue()).unwrap() as i32);
        let mut size: i32 = jcall!(self.getList()).size();
        for index in (0..size) {
            let mut featureInterface: Option<Rc<RefCell<dyn FeatureResourceAnimationInterfaceFactoryInterface>>> = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn FeatureResourceAnimationInterfaceFactoryInterface>>>>().expect("ClassCastException").clone()) };
            if jcall!(featureInterface).isFeature() {
                let mut rectangle: Option<Rc<RefCell<Rectangle>>> = jcall!(featureInterface).getRectangle(resource.clone());
                if !match (&rectangle, &crate::org::allbinary::graphics::RectangleFactory::RectangleFactory_SINGLETON.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    return Rectangle::new(jcall!(pointFactory).createXY(x.clone(), y.clone()), ((jcall!(rectangle).getWidth() * scale) >> 1).clone(), ((jcall!(rectangle).getHeight() * scale) >> 1).clone());
                }
            }
        }
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(StringMaker::new()).append(String::from("No rectangle available for current feature selection or Resource: ")).append(resource.clone()).toString())));
    }
    

    // throws Exception
    pub fn getProcedural(&mut self, mut resource: String) -> Option<Rc<RefCell<dyn ProceduralAnimationInterfaceFactoryInterface>>> {
        return (self.getBasicAnimationInterfaceFactoryInstance(resource.clone())).clone() /* as ProceduralAnimationInterfaceFactoryInterface */;
    }
    

    // throws Exception
    pub fn get(&mut self, mut resource: String) -> Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>> {
        return (self.getBasicAnimationInterfaceFactoryInstance(resource.clone())).clone() /* as AnimationInterfaceFactoryInterface */;
    }
    

    // throws Exception
    pub fn getBasicAnimationInterfaceFactoryInstance(&mut self, mut resource: String) -> Option<Rc<RefCell<dyn BasicAnimationInterfaceFactoryInterface>>> {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getList();
        let mut resourceTypeAvailableList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut size: i32 = jcall!(self.getList()).size();
        let mut featureInterface: Option<Rc<RefCell<dyn FeatureResourceAnimationInterfaceFactoryInterface>>> = None;
        let mut animationInterfaceFactoryInterface: Option<Rc<RefCell<dyn BasicAnimationInterfaceFactoryInterface>>> = None;
        for index in (0..size) {
            featureInterface = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn FeatureResourceAnimationInterfaceFactoryInterface>>>>().expect("ClassCastException").clone()) };
            if jcall!(featureInterface).isFeature() {
                jcall!(resourceTypeAvailableList).add(crate::java_into_object(featureInterface.clone()));
                animationInterfaceFactoryInterface = jcall!(featureInterface).getBasicAnimationInterfaceFactoryInstance(resource.clone());
                if !match (&animationInterfaceFactoryInterface, &unsafe { NullAnimationFactory_NULL_NOT_FOR_USE_ANIMATION_FACTORY.clone() }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    return animationInterfaceFactoryInterface.clone();
                }
            }
        }
        if jcall!(resourceTypeAvailableList).size() > 0 {
            let mut NO_ANIMATION_AVAILABLE_FROM: String = String::from("No animation available from: ");
            let mut FOR_FACTORIES: String = String::from(" factories: ");
            let mut FOR_RESOURCE: String = String::from(" for Resource: ");
            let mut HAS_KEY: String = String::from(" has: ");
            let mut RESOURCES_LABEL: String = String::from(" resources ");
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(NO_ANIMATION_AVAILABLE_FROM.clone());
            jcall!(stringBuffer).appendint(jcall!(resourceTypeAvailableList).size());
            jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).FORWARD_SLASH.clone());
            jcall!(stringBuffer).appendint(size.clone());
            jcall!(stringBuffer).append(FOR_FACTORIES.clone());
            let mut size2: i32 = jcall!(resourceTypeAvailableList).size();
            for index in (0..size2) {
                jcall!(stringBuffer).append(jcall!(jcall!(resourceTypeAvailableList).get(index.clone())).toString());
                jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).COMMA_SEP.clone());
            }
            jcall!(stringBuffer).append(FOR_RESOURCE.clone());
            jcall!(stringBuffer).append(resource.clone());
            for index in (0..size) {
                jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).NEW_LINE.clone());
                featureInterface = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn FeatureResourceAnimationInterfaceFactoryInterface>>>>().expect("ClassCastException").clone()) };
                if jcall!(featureInterface).isFeature() {
                    jcall!(stringBuffer).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(featureInterface.clone())));
                    jcall!(stringBuffer).append(HAS_KEY.clone());
                    jcall!(stringBuffer).appendint(jcall!(jcall!(featureInterface).getHashtable()).size());
                    jcall!(stringBuffer).append(RESOURCES_LABEL.clone());
                }
            }
            panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(stringBuffer).toString())));
        } else {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            for index in (0..size) {
                featureInterface = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn FeatureResourceAnimationInterfaceFactoryInterface>>>>().expect("ClassCastException").clone()) };
                jcall!(stringBuffer).append(jcall!(featureInterface).toString());
                jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
            }
            let mut result: String = jcall!(stringBuffer).toString();
            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
            panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(stringBuffer).append(String::from("No feature resource type available for Resource: ")).append(resource.clone()).append(String::from(" Resource Factories Available: ")).append(result.clone()).toString())));
        }
    }
}
impl std::ops::Deref for FeaturedAnimationInterfaceFactoryInterfaceFactory {
    type Target = FeaturedResourceFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for FeaturedAnimationInterfaceFactoryInterfaceFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::resource::FeaturedResourceFactory::FeaturedResourceFactoryDyn for FeaturedAnimationInterfaceFactoryInterfaceFactory {
    fn as_FeaturedResourceFactory(&self) -> &crate::org::allbinary::game::resource::FeaturedResourceFactory::FeaturedResourceFactory { &self.base }
    fn as_FeaturedResourceFactory_mut(&mut self) -> &mut crate::org::allbinary::game::resource::FeaturedResourceFactory::FeaturedResourceFactory { &mut self.base }
}



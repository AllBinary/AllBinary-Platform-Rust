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
// package org.allbinary.game.resource
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationship;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::relationship::RelativeLayerRelationship::RelativeLayerRelationship;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use super::FeaturedResourceFactory::FeaturedResourceFactory;
use super::ResourceRelativeRelationshipFactoryInterface::ResourceRelativeRelationshipFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FeaturedResourceRelativeRelationshipFactory {
    pub base: FeaturedResourceFactory,
    pub basicArrayListUtil: Option<Rc<RefCell<BasicArrayListUtil>>>,
}

pub static mut FeaturedResourceRelativeRelationshipFactory_INSTANCE: std::cell::LazyCell<Option<Rc<RefCell<FeaturedResourceRelativeRelationshipFactory>>>> = std::cell::LazyCell::new(|| FeaturedResourceRelativeRelationshipFactory::new());

impl FeaturedResourceRelativeRelationshipFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: FeaturedResourceFactory::new_struct(),
            basicArrayListUtil: None,
        };
        this.basicArrayListUtil = BasicArrayListUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<FeaturedResourceRelativeRelationshipFactory>>> {
        return unsafe { FeaturedResourceRelativeRelationshipFactory_INSTANCE.clone() }.clone();
    }
    

    // throws Exception
    pub fn getRelativeRelationshipList(&mut self, mut resource: String, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut featureReleaseList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getList();
        let mut size: i32 = jcall!(self.getList()).size();
        for index in (0..size) {
            let mut featureInterface: Option<Rc<RefCell<dyn ResourceRelativeRelationshipFactoryInterface>>> = { let java_object = (jcall!(featureReleaseList).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn ResourceRelativeRelationshipFactoryInterface>>>>().expect("ClassCastException").clone()) };
            if jcall!(featureInterface).isFeature() {
                let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(featureInterface).getResourceRelativeRelationshipList(resource.clone());
                if !match (&list, &jcall!(self.basicArrayListUtil).getImmutableInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    return self.duplicate(list.clone(), layer.clone());
                }
            }
        }
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(StringMaker::new()).append(String::from("Not available for current feature selection or Resource: ")).append(resource.clone()).toString())));
    }
    

    // throws Exception
    pub fn duplicate(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut pointFactory: Option<Rc<RefCell<PointFactory>>> = PointFactory::getInstance();
        let mut newList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut size: i32 = jcall!(list).size();
        for index in (0..size) {
            let mut relativeRelationship: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationshipDyn>>> = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationshipDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(newList).add(crate::java_into_object(RelativeLayerRelationship::new(layer.clone(), jcall!(pointFactory).createXY(jcall!(relativeRelationship).getX(), jcall!(relativeRelationship).getY()), jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::relationship::RelativeLayerRelationship::RelativeLayerRelationshipDyn>>)));
        }
        return newList.clone();
    }
}
impl std::ops::Deref for FeaturedResourceRelativeRelationshipFactory {
    type Target = FeaturedResourceFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for FeaturedResourceRelativeRelationshipFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::resource::FeaturedResourceFactory::FeaturedResourceFactoryDyn for FeaturedResourceRelativeRelationshipFactory {
    fn as_FeaturedResourceFactory(&self) -> &crate::org::allbinary::game::resource::FeaturedResourceFactory::FeaturedResourceFactory { &self.base }
    fn as_FeaturedResourceFactory_mut(&mut self) -> &mut crate::org::allbinary::game::resource::FeaturedResourceFactory::FeaturedResourceFactory { &mut self.base }
}



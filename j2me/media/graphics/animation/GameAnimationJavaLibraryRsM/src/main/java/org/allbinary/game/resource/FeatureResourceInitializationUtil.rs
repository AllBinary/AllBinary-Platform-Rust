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
use crate::org::allbinary::animation::FeaturedAnimationInterfaceFactoryInterfaceFactory::FeaturedAnimationInterfaceFactoryInterfaceFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::FeaturedResourceFactory::FeaturedResourceFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FeatureResourceInitializationUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static FeatureResourceInitializationUtil_instance: Option<Rc<RefCell<FeatureResourceInitializationUtil>>> = FeatureResourceInitializationUtil::new(); }

impl FeatureResourceInitializationUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<FeatureResourceInitializationUtil>>> {
        return FeatureResourceInitializationUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn init(&mut self, mut level: i32) {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        jcall!(self.logUtil).putF(jcall!(commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).INIT.clone());
        let mut featuredResourceFactory: Option<Rc<RefCell<dyn crate::org::allbinary::game::resource::FeaturedResourceFactory::FeaturedResourceFactoryDyn>>> = FeaturedAnimationInterfaceFactoryInterfaceFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::resource::FeaturedResourceFactory::FeaturedResourceFactoryDyn>>);
        jcall!(featuredResourceFactory).init(level.clone());
        jcall!(FeaturedResourceRelativeRelationshipFactory::FeaturedResourceRelativeRelationshipFactory::getInstance()).init(level.clone());
    }
}


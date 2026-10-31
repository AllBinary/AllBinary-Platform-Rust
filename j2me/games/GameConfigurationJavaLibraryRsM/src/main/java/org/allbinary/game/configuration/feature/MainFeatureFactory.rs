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
// package org.allbinary.game.configuration.feature
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::Feature::Feature;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MainFeatureFactory {
    pub FULL_SCREEN: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>,
    pub TITLE_BAR: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>,
    pub STATIC: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>,
    pub LOAD_ONDEMAND: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>,
    pub LOAD_ALL: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>,
}

thread_local! { pub static MainFeatureFactory_instance: Option<Rc<RefCell<MainFeatureFactory>>> = MainFeatureFactory::new(); }

impl MainFeatureFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            FULL_SCREEN: None,
            TITLE_BAR: None,
            STATIC: None,
            LOAD_ONDEMAND: None,
            LOAD_ALL: None,
        };
        this.FULL_SCREEN = Feature::new(String::from("Full Screen")).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>);
        this.TITLE_BAR = Feature::new(String::from("Title Bar")).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>);
        this.STATIC = Feature::new(String::from("Static Dependent Feature")).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>);
        this.LOAD_ONDEMAND = Feature::new(String::from("Load OnDemand")).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>);
        this.LOAD_ALL = Feature::new(String::from("Load All")).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<MainFeatureFactory>>> {
        return MainFeatureFactory_instance.with(Clone::clone).clone();
    }
}


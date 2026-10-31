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
use super::InputFeature::InputFeature;
use super::Features::Features;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputFeatureFactory {
    pub REMOVE_DUPLICATE_KEY_PRESSES: Option<Rc<RefCell<InputFeature>>>,
    pub INPUT_MAPPING: Option<Rc<RefCell<InputFeature>>>,
    pub MULTI_KEY_PRESS: Option<Rc<RefCell<InputFeature>>>,
    pub SINGLE_KEY_REPEAT_PRESS: Option<Rc<RefCell<InputFeature>>>,
    pub SINGLE_KEY_PRESS: Option<Rc<RefCell<InputFeature>>>,
}

thread_local! { pub static InputFeatureFactory_instance: Option<Rc<RefCell<InputFeatureFactory>>> = InputFeatureFactory::new(); }

impl InputFeatureFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<InputFeatureFactory>>> {
        return InputFeatureFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            REMOVE_DUPLICATE_KEY_PRESSES: None,
            INPUT_MAPPING: None,
            MULTI_KEY_PRESS: None,
            SINGLE_KEY_REPEAT_PRESS: None,
            SINGLE_KEY_PRESS: None,
        };
        this.REMOVE_DUPLICATE_KEY_PRESSES = InputFeature::new(String::from("Remove Duplicate Key Presses"));
        this.INPUT_MAPPING = InputFeature::new(String::from("Input Mapping"));
        this.MULTI_KEY_PRESS = InputFeature::new(String::from("Multi Key Press"));
        this.SINGLE_KEY_REPEAT_PRESS = InputFeature::new(String::from("Single Key Repeat Press"));
        this.SINGLE_KEY_PRESS = InputFeature::new(String::from("Single Key Press"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isSingleKeyProcessing(&mut self) -> bool {
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        if jcall!(features).isFeature(self.SINGLE_KEY_REPEAT_PRESS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) || jcall!(features).isFeature(self.SINGLE_KEY_PRESS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            return true;
        } else {
            return false;
        }
    }
}


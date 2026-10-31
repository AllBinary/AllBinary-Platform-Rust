// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
use super::AnimationBehavior::AnimationBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AnimationBehaviorFactory {
}

thread_local! { pub static AnimationBehaviorFactory_instance: Option<Rc<RefCell<dyn AnimationBehaviorFactoryDyn>>> = AnimationBehaviorFactory::new().map(|reference| reference as Rc<RefCell<dyn AnimationBehaviorFactoryDyn>>); }

impl AnimationBehaviorFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn AnimationBehaviorFactoryDyn>>> {
        return AnimationBehaviorFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn getOrCreateInstance(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>> {
        return AnimationBehavior::getInstance();
    }
}
pub trait AnimationBehaviorFactoryDyn {
    fn as_AnimationBehaviorFactory(&self) -> &AnimationBehaviorFactory;
    fn as_AnimationBehaviorFactory_mut(&mut self) -> &mut AnimationBehaviorFactory;
    fn getOrCreateInstance(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>> { AnimationBehaviorFactory::getOrCreateInstance(self.as_AnimationBehaviorFactory_mut()) }
}
impl AnimationBehaviorFactoryDyn for AnimationBehaviorFactory {
    fn as_AnimationBehaviorFactory(&self) -> &AnimationBehaviorFactory { self }
    fn as_AnimationBehaviorFactory_mut(&mut self) -> &mut AnimationBehaviorFactory { self }
}



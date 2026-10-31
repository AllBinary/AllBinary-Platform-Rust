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
use crate::org::allbinary::media::ScaleProperties::ScaleProperties;
use super::AnimationInterfaceFactoryInterface::AnimationInterfaceFactoryInterface;
use super::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface;
use super::Animation::Animation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BaseAnimationInterfaceFactoryInterfaceComposite {
    pub basicAnimationInterfaceFactoryInterfaceArray: Vec<Option<Rc<RefCell<dyn BasicAnimationInterfaceFactoryInterface>>>>,
}

impl AnimationInterfaceFactoryInterface for BaseAnimationInterfaceFactoryInterfaceComposite {
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
    fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) { Self::setInitialScale(self, scaleProperties) }
}
impl BasicAnimationInterfaceFactoryInterface for BaseAnimationInterfaceFactoryInterfaceComposite {
}

impl BaseAnimationInterfaceFactoryInterfaceComposite {
    

    pub fn new_struct(mut basicAnimationInterfaceFactoryInterfaceArray: Vec<Option<Rc<RefCell<dyn BasicAnimationInterfaceFactoryInterface>>>>) -> Self {
        let mut this = Self {
            basicAnimationInterfaceFactoryInterfaceArray: Vec::new(),
        };
        this.basicAnimationInterfaceFactoryInterfaceArray = basicAnimationInterfaceFactoryInterfaceArray.clone();
        this
    }

    pub fn new(mut basicAnimationInterfaceFactoryInterfaceArray: Vec<Option<Rc<RefCell<dyn BasicAnimationInterfaceFactoryInterface>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(basicAnimationInterfaceFactoryInterfaceArray))))
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return jcall!(NullAnimationFactory::NullAnimationFactory::getFactoryInstance()).getInstance(0);
    }
    

    pub fn getBasicAnimationInterfaceFactoryInterfaceArray(&mut self) -> Vec<Option<Rc<RefCell<dyn BasicAnimationInterfaceFactoryInterface>>>> {
        return self.basicAnimationInterfaceFactoryInterfaceArray.clone();
    }
    

    pub fn setBasicAnimationInterfaceFactoryInterfaceArray(&mut self, mut basicAnimationInterfaceFactoryInterfaceArray: Vec<Option<Rc<RefCell<dyn BasicAnimationInterfaceFactoryInterface>>>>) {
        self.basicAnimationInterfaceFactoryInterfaceArray = basicAnimationInterfaceFactoryInterfaceArray.clone();
    }
    

    pub fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) {
    }
}


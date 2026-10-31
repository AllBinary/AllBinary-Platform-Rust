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
use super::Animation::Animation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AnimationInterfaceFactoryInterfaceComposite {
    pub animationInterfaceFactoryInterfaceArray: Vec<Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>>,
}

impl AnimationInterfaceFactoryInterface for AnimationInterfaceFactoryInterfaceComposite {
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
    fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) { Self::setInitialScale(self, scaleProperties) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for AnimationInterfaceFactoryInterfaceComposite {
}

impl AnimationInterfaceFactoryInterfaceComposite {
    

    pub fn new_struct(mut basicAnimationInterfaceFactoryInterfaceArray: Vec<Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>>) -> Self {
        let mut this = Self {
            animationInterfaceFactoryInterfaceArray: Vec::new(),
        };
        this.animationInterfaceFactoryInterfaceArray = basicAnimationInterfaceFactoryInterfaceArray.clone();
        this
    }

    pub fn new(mut basicAnimationInterfaceFactoryInterfaceArray: Vec<Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(basicAnimationInterfaceFactoryInterfaceArray))))
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return jcall!(NullAnimationFactory::NullAnimationFactory::getFactoryInstance()).getInstance(0);
    }
    

    pub fn getAnimationInterfaceFactoryInterfaceArray(&mut self) -> Vec<Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>>> {
        return self.animationInterfaceFactoryInterfaceArray.clone();
    }
    

    pub fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) {
    }
}


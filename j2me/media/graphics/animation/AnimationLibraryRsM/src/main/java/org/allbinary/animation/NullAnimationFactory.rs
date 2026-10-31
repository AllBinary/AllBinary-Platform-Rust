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
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::media::ScaleProperties::ScaleProperties;
use super::AnimationInterfaceFactoryInterface::AnimationInterfaceFactoryInterface;
use super::ProceduralAnimationInterfaceFactoryInterface::ProceduralAnimationInterfaceFactoryInterface;
use super::Animation::Animation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullAnimationFactory {
    pub NULL_ANIMATION: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
    pub EMPTY_ARRAY: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>>,
}

pub static mut NullAnimationFactory_NULL_NOT_FOR_USE_ANIMATION_FACTORY: std::cell::LazyCell<Option<Rc<RefCell<NullAnimationFactory>>>> = std::cell::LazyCell::new(|| NullAnimationFactory::new());
pub static mut NullAnimationFactory_NULL_ANIMATION_FACTORY: std::cell::LazyCell<Option<Rc<RefCell<NullAnimationFactory>>>> = std::cell::LazyCell::new(|| NullAnimationFactory::new());
impl AnimationInterfaceFactoryInterface for NullAnimationFactory {
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
    fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) { Self::setInitialScale(self, scaleProperties) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for NullAnimationFactory {
}
impl ProceduralAnimationInterfaceFactoryInterface for NullAnimationFactory {
    fn getInstanceAnimation(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstanceAnimation(self, animationInterface) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for NullAnimationFactory {
}

impl NullAnimationFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NULL_ANIMATION: None,
            EMPTY_ARRAY: Vec::new(),
        };
        this.NULL_ANIMATION = Animation::new() /* Java anonymous class body: implement it as a named type
        

        pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        }
*/;
        this.EMPTY_ARRAY = vec![None; (0) as usize];
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getFactoryInstance() -> Option<Rc<RefCell<NullAnimationFactory>>> {
        return unsafe { NullAnimationFactory_NULL_ANIMATION_FACTORY.clone() }.clone();
    }
    

    pub fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return self.NULL_ANIMATION.clone();
    }
    

    // throws Exception
    pub fn getInstanceAnimation(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return self.NULL_ANIMATION.clone();
    }
    

    pub fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) {
    }
}


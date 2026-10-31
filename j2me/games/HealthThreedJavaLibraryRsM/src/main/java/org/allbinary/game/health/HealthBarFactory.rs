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
// package org.allbinary.game.health
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureUtil::OpenGLFeatureUtil;
use crate::org::allbinary::animation::FeaturedAnimationInterfaceFactoryInterfaceFactory::FeaturedAnimationInterfaceFactoryInterfaceFactory;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use super::HealthBar::HealthBar;
use super::HealthBarTwodAnimation::HealthBarTwodAnimation;
use super::HealthBarAnimationInterfaceFactory::HealthBarAnimationInterfaceFactory;
use super::Health::Health;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HealthBarFactory {
}


impl HealthBarFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut healthInterface: Option<Rc<RefCell<Health>>>, mut location: i32, mut direction: i32) -> Option<Rc<RefCell<HealthBar>>> {
        if jcall!(OpenGLFeatureUtil::getInstance()).isAnyThreed() {
            let mut featuredAnimationInterfaceFactoryInterfaceFactory: Option<Rc<RefCell<FeaturedAnimationInterfaceFactoryInterfaceFactory>>> = FeaturedAnimationInterfaceFactoryInterfaceFactory::getInstance();
            let mut healthBarAnimationInterfaceFactory: Option<Rc<RefCell<HealthBarAnimationInterfaceFactory>>> = (jcall!(featuredAnimationInterfaceFactoryInterfaceFactory).get(Health_NAME.with(Clone::clone).clone())).clone() /* as HealthBarAnimationInterfaceFactory */;
            return HealthBar::new(layerInterface.clone(), healthInterface.clone(), jcall!(healthBarAnimationInterfaceFactory).getInstanceByLocation(layerInterface.clone(), location.clone()), direction.clone());
        } else {
            return HealthBar::new(layerInterface.clone(), healthInterface.clone(), HealthBarTwodAnimation::new(layerInterface.clone(), location.clone()), direction.clone());
        }
    }
}


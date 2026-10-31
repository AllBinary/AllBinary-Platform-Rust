// 
// AllBinary Open License Version 1 
// Copyright (c) 2006 AllBinary 
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
// package org.allbinary.game.layer.unit
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources;
use crate::org::allbinary::animation::AnimationInterfaceFactoryInterface::AnimationInterfaceFactoryInterface;
use crate::org::allbinary::animation::FeaturedAnimationInterfaceFactoryInterfaceFactory::FeaturedAnimationInterfaceFactoryInterfaceFactory;
use crate::org::allbinary::animation::NullRotationAnimationFactory::NullRotationAnimationFactory;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DecalAnimationInterfaceFactory {
}

thread_local! { pub static DecalAnimationInterfaceFactory_instance: Option<Rc<RefCell<DecalAnimationInterfaceFactory>>> = DecalAnimationInterfaceFactory::new(); }

impl DecalAnimationInterfaceFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<DecalAnimationInterfaceFactory>>> {
        return DecalAnimationInterfaceFactory_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn getAnimationFactory(&mut self, mut basicGameResources: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResourcesDyn>>>, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>> {
        let mut animationInterfaceFactoryInterface: Option<Rc<RefCell<dyn AnimationInterfaceFactoryInterface>>> = None;
        if match (&basicColor, &jcall!(BasicColorFactory::getInstance()).RED) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            animationInterfaceFactoryInterface = jcall!(FeaturedAnimationInterfaceFactoryInterfaceFactory::getInstance()).get(jcall!(basicGameResources).as_BasicGameResources_mut().RESOURCE.clone());
        } else {
            animationInterfaceFactoryInterface = NullRotationAnimationFactory::getFactoryInstance();
        }
        return animationInterfaceFactoryInterface.clone();
    }
}


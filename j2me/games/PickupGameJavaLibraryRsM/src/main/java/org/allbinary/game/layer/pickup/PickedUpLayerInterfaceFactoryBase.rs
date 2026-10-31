// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package org.allbinary.game.layer.pickup
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use super::PickedUpLayerInterfaceFactoryInterface::PickedUpLayerInterfaceFactoryInterface;
use super::PickedUpLayerType::PickedUpLayerType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PickedUpLayerInterfaceFactoryBase {
}

thread_local! { pub static PickedUpLayerInterfaceFactoryBase_NULL_PICKED_UP_LAYER_FACTORY_BASE: Option<Rc<RefCell<dyn PickedUpLayerInterfaceFactoryBaseDyn>>> = PickedUpLayerInterfaceFactoryBase::new().map(|reference| reference as Rc<RefCell<dyn PickedUpLayerInterfaceFactoryBaseDyn>>); }
impl PickedUpLayerInterfaceFactoryInterface for PickedUpLayerInterfaceFactoryBase {
    fn getPickedUpLayerType(&mut self) -> Option<Rc<RefCell<PickedUpLayerType>>> { Self::getPickedUpLayerType(self) }
    fn getIconLayer(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> { Self::getIconLayer(self) }
    fn getAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getAnimationInterface(self) }
}

impl PickedUpLayerInterfaceFactoryBase {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getIconLayer(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getPickedUpLayerType(&mut self) -> Option<Rc<RefCell<PickedUpLayerType>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
pub trait PickedUpLayerInterfaceFactoryBaseDyn: crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryInterface::PickedUpLayerInterfaceFactoryInterface {
    fn as_PickedUpLayerInterfaceFactoryBase(&self) -> &PickedUpLayerInterfaceFactoryBase;
    fn as_PickedUpLayerInterfaceFactoryBase_mut(&mut self) -> &mut PickedUpLayerInterfaceFactoryBase;
}
impl PickedUpLayerInterfaceFactoryBaseDyn for PickedUpLayerInterfaceFactoryBase {
    fn as_PickedUpLayerInterfaceFactoryBase(&self) -> &PickedUpLayerInterfaceFactoryBase { self }
    fn as_PickedUpLayerInterfaceFactoryBase_mut(&mut self) -> &mut PickedUpLayerInterfaceFactoryBase { self }
}



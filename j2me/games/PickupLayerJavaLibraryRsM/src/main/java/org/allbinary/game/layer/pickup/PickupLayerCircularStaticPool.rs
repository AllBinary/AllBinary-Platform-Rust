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
// package org.allbinary.game.layer.pickup
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::layer::AllBinaryLayerCircularPool::AllBinaryLayerCircularPool;
use super::PickupLayer::PickupLayer;
use super::PickedUpLayerInterface::PickedUpLayerInterface;
use super::PickedUpLayerInterfaceFactoryInterface::PickedUpLayerInterfaceFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PickupLayerCircularStaticPool {
    pub base: AllBinaryLayerCircularPool,
}

pub static mut PickupLayerCircularStaticPool_SINGLETON: std::cell::LazyCell<Option<Rc<RefCell<PickupLayerCircularStaticPool>>>> = std::cell::LazyCell::new(|| PickupLayerCircularStaticPool::new());

impl PickupLayerCircularStaticPool {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AllBinaryLayerCircularPool::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PickupLayerCircularStaticPool>>> {
        return unsafe { PickupLayerCircularStaticPool_SINGLETON.clone() }.clone();
    }
    

    // throws Exception
    pub fn getInstanceXYZ(&mut self, mut pickedUpLayerInterfaceFactoryInterface: Option<Rc<RefCell<dyn PickedUpLayerInterfaceFactoryInterface>>>, mut x: i32, mut y: i32, mut z: i32) -> Option<Rc<RefCell<dyn PickedUpLayerInterface>>> {
        let mut pickupLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayerDyn>>> = { let java_object = (self.getNextInstance()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::pickup::PickupLayer::PickupLayerDyn>>>>().expect("ClassCastException").clone()) };
        jcall!(pickupLayer).initXYZ(x.clone(), y.clone(), z.clone());
        jcall!(pickupLayer).init(pickedUpLayerInterfaceFactoryInterface.clone(), jcall!(pickedUpLayerInterfaceFactoryInterface).getAnimationInterface());
        return pickupLayer.clone().map(|reference| reference as Rc<RefCell<dyn PickedUpLayerInterface>>);
    }
}
impl std::ops::Deref for PickupLayerCircularStaticPool {
    type Target = AllBinaryLayerCircularPool;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PickupLayerCircularStaticPool {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::layer::AllBinaryLayerCircularPool::AllBinaryLayerCircularPoolDyn for PickupLayerCircularStaticPool {
    fn as_AllBinaryLayerCircularPool(&self) -> &crate::org::allbinary::layer::AllBinaryLayerCircularPool::AllBinaryLayerCircularPool { &self.base }
    fn as_AllBinaryLayerCircularPool_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayerCircularPool::AllBinaryLayerCircularPool { &mut self.base }
}
impl crate::org::allbinary::logic::util::cache::BaseCircularPool::BaseCircularPoolDyn for PickupLayerCircularStaticPool {
    fn as_BaseCircularPool(&self) -> &crate::org::allbinary::logic::util::cache::BaseCircularPool::BaseCircularPool { &self.base.base }
    fn as_BaseCircularPool_mut(&mut self) -> &mut crate::org::allbinary::logic::util::cache::BaseCircularPool::BaseCircularPool { &mut self.base.base }
}



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
// package org.allbinary.game.ag.layer.pickup.health
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::health::HealthInterface::HealthInterface;
use crate::org::allbinary::game::health::HealthInterfaceCompositeInterface::HealthInterfaceCompositeInterface;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerCircularPool::AllBinaryLayerCircularPool;
use crate::org::allbinary::layer::AllBinaryLayerFactoryInterface::AllBinaryLayerFactoryInterface;
use super::HealLayer::HealLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HealLayerCircularStaticPool {
    pub base: AllBinaryLayerCircularPool,
}


impl HealLayerCircularStaticPool {
    

    pub fn new_struct(mut allbinaryLayerFactoryInterface: Option<Rc<RefCell<dyn AllBinaryLayerFactoryInterface>>>, mut total: i32) -> Self {
        let mut this = Self {
            base: AllBinaryLayerCircularPool::new_struct(),
        };
        this.base.initAllBinaryLayerCircularPool(allbinaryLayerFactoryInterface.clone(), total.clone());
        this
    }

    pub fn new(mut allbinaryLayerFactoryInterface: Option<Rc<RefCell<dyn AllBinaryLayerFactoryInterface>>>, mut total: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(allbinaryLayerFactoryInterface, total))))
    }
    

    // throws Exception
    pub fn visit(&mut self, mut sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        let mut healLayer: Option<Rc<RefCell<HealLayer>>> = { let java_object = (self.getNextInstance()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<HealLayer>>>>().expect("ClassCastException").clone()) };
        let mut healthInterfaceCompositeInterface: Option<Rc<RefCell<dyn HealthInterfaceCompositeInterface>>> = (sourceLayerInterface).clone() /* as HealthInterfaceCompositeInterface */;
        let mut healthInterface: Option<Rc<RefCell<dyn HealthInterface>>> = jcall!(healthInterfaceCompositeInterface).getHealthInterface();
        jcall!(healLayer).visit(healthInterface.clone());
    }
}
impl std::ops::Deref for HealLayerCircularStaticPool {
    type Target = AllBinaryLayerCircularPool;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HealLayerCircularStaticPool {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::layer::AllBinaryLayerCircularPool::AllBinaryLayerCircularPoolDyn for HealLayerCircularStaticPool {
    fn as_AllBinaryLayerCircularPool(&self) -> &crate::org::allbinary::layer::AllBinaryLayerCircularPool::AllBinaryLayerCircularPool { &self.base }
    fn as_AllBinaryLayerCircularPool_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayerCircularPool::AllBinaryLayerCircularPool { &mut self.base }
}
impl crate::org::allbinary::logic::util::cache::BaseCircularPool::BaseCircularPoolDyn for HealLayerCircularStaticPool {
    fn as_BaseCircularPool(&self) -> &crate::org::allbinary::logic::util::cache::BaseCircularPool::BaseCircularPool { &self.base.base }
    fn as_BaseCircularPool_mut(&mut self) -> &mut crate::org::allbinary::logic::util::cache::BaseCircularPool::BaseCircularPool { &mut self.base.base }
}



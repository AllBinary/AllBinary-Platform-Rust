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
use crate::org::allbinary::animation::FeaturedAnimationInterfaceFactoryInterfaceFactory::FeaturedAnimationInterfaceFactoryInterfaceFactory;
use crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactory;
use crate::org::allbinary::game::layer::pickup::IconLayerFactory::IconLayerFactory;
use crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactory;
use crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryInterface::PickedUpLayerInterfaceFactoryInterface;
use crate::org::allbinary::game::layer::pickup::PickedUpLayerTypeFactory::PickedUpLayerTypeFactory;
use crate::org::allbinary::game::layer::pickup::PickupProcessorInterface::PickupProcessorInterface;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use super::HealLayerCircularStaticPool::HealLayerCircularStaticPool;
use super::HealLayerFactory::HealLayerFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HealPickedUpLayerInterfaceFactory {
    pub base: PickedUpLayerInterfaceFactory,
    pub pool: Option<Rc<RefCell<HealLayerCircularStaticPool>>>,
}

pub static mut HealPickedUpLayerInterfaceFactory_pickedUpLayerInterfaceFactoryInterface: std::cell::LazyCell<Option<Rc<RefCell<dyn PickedUpLayerInterfaceFactoryInterface>>>> = std::cell::LazyCell::new(|| crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactory_NULL_COUNTED_PICKUP_LAYER_FACTORY.with(Clone::clone).clone());
impl PickupProcessorInterface for HealPickedUpLayerInterfaceFactory {
    fn process(&mut self, mut sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) { Self::process(self, sourceLayerInterface) }
}

impl HealPickedUpLayerInterfaceFactory {
    

    // throws Exception
    pub fn init() {
        unsafe { *HealPickedUpLayerInterfaceFactory_pickedUpLayerInterfaceFactoryInterface = HealPickedUpLayerInterfaceFactory::new() };
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: PickedUpLayerInterfaceFactory::new_struct(jcall!(PickedUpLayerTypeFactory::getInstance()).HEAL.clone(), IconLayerFactory::getInstance(jcall!(jcall!(FeaturedAnimationInterfaceFactoryInterfaceFactory::getInstance()).get(jcall!(HealthResources::HealthResources::getInstance()).RESOURCE.clone())).getInstance(0), 10, 10), jcall!(jcall!(FeaturedAnimationInterfaceFactoryInterfaceFactory::getInstance()).get(jcall!(HealthResources::HealthResources::getInstance()).RESOURCE.clone())).getInstance(0)),
            pool: None,
        };
        this.pool = HealLayerCircularStaticPool::new(HealLayerFactory::new(), 1);
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getTotal(&mut self) -> i32 {
        return 1;
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn PickedUpLayerInterfaceFactoryInterface>>> {
        return unsafe { HealPickedUpLayerInterfaceFactory_pickedUpLayerInterfaceFactoryInterface.clone() }.clone();
    }
    

    // throws Exception
    pub fn process(&mut self, mut sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        jcall!(self.pool).visit(sourceLayerInterface.clone());
    }
}
impl std::ops::Deref for HealPickedUpLayerInterfaceFactory {
    type Target = PickedUpLayerInterfaceFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HealPickedUpLayerInterfaceFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactoryDyn for HealPickedUpLayerInterfaceFactory {
    fn as_PickedUpLayerInterfaceFactory(&self) -> &crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactory { &self.base }
    fn as_PickedUpLayerInterfaceFactory_mut(&mut self) -> &mut crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactory { &mut self.base }
}
impl crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryBase::PickedUpLayerInterfaceFactoryBaseDyn for HealPickedUpLayerInterfaceFactory {
    fn as_PickedUpLayerInterfaceFactoryBase(&self) -> &crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryBase::PickedUpLayerInterfaceFactoryBase { &self.base.base }
    fn as_PickedUpLayerInterfaceFactoryBase_mut(&mut self) -> &mut crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryBase::PickedUpLayerInterfaceFactoryBase { &mut self.base.base }
}
impl crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryInterface::PickedUpLayerInterfaceFactoryInterface for HealPickedUpLayerInterfaceFactory {
    fn getPickedUpLayerType(&mut self) -> Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickedUpLayerType::PickedUpLayerType>>> { crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactory::getPickedUpLayerType(crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactoryDyn::as_PickedUpLayerInterfaceFactory_mut(self)) }
    fn getIconLayer(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> { crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactory::getIconLayer(crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactoryDyn::as_PickedUpLayerInterfaceFactory_mut(self)) }
    fn getAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactory::getAnimationInterface(crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactoryDyn::as_PickedUpLayerInterfaceFactory_mut(self)) }
}



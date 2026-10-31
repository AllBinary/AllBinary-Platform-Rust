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
// package org.allbinary.game.ag.layer.pickup.points
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::animation::FeaturedAnimationInterfaceFactoryInterfaceFactory::FeaturedAnimationInterfaceFactoryInterfaceFactory;
use crate::org::allbinary::game::configuration::GameConfigurationUtil::GameConfigurationUtil;
use crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactory;
use crate::org::allbinary::game::layer::pickup::IconLayerFactory::IconLayerFactory;
use crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactory;
use crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryInterface::PickedUpLayerInterfaceFactoryInterface;
use crate::org::allbinary::game::layer::pickup::PickedUpLayerTypeFactory::PickedUpLayerTypeFactory;
use crate::org::allbinary::game::layer::pickup::PickupProcessorInterface::PickupProcessorInterface;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use super::PointsLayerCircularStaticPool::PointsLayerCircularStaticPool;
use super::PointsLayerFactory::PointsLayerFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Points20kPickedUpLayerInterfaceFactory {
    pub base: PickedUpLayerInterfaceFactory,
    pub pool: Option<Rc<RefCell<PointsLayerCircularStaticPool>>>,
}

pub static mut Points20kPickedUpLayerInterfaceFactory_pickedUpLayerInterfaceFactoryInterface: std::cell::LazyCell<Option<Rc<RefCell<dyn PickedUpLayerInterfaceFactoryInterface>>>> = std::cell::LazyCell::new(|| crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactory_NULL_COUNTED_PICKUP_LAYER_FACTORY.with(Clone::clone).clone());
impl PickupProcessorInterface for Points20kPickedUpLayerInterfaceFactory {
    fn process(&mut self, mut sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) { Self::process(self, sourceLayerInterface) }
}

impl Points20kPickedUpLayerInterfaceFactory {
    

    // throws Exception
    pub fn init() {
        unsafe { *Points20kPickedUpLayerInterfaceFactory_pickedUpLayerInterfaceFactoryInterface = Points20kPickedUpLayerInterfaceFactory::new() };
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: PickedUpLayerInterfaceFactory::new_struct(jcall!(PickedUpLayerTypeFactory::getInstance()).POINTS.clone(), IconLayerFactory::getInstance(jcall!(jcall!(FeaturedAnimationInterfaceFactoryInterfaceFactory::getInstance()).get(jcall!(PointsResources::PointsResources::getInstance()).RESOURCE_2.clone())).getInstance(0), 10, 10), jcall!(jcall!(FeaturedAnimationInterfaceFactoryInterfaceFactory::getInstance()).get(jcall!(PointsResources::PointsResources::getInstance()).RESOURCE_2.clone())).getInstance(0)),
            pool: None,
        };
        this.pool = PointsLayerCircularStaticPool::new(PointsLayerFactory::new(jcall!(GameConfigurationUtil::getInstance()).getCompetitionValue() * 20000), 1);
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
        return unsafe { Points20kPickedUpLayerInterfaceFactory_pickedUpLayerInterfaceFactoryInterface.clone() }.clone();
    }
    

    // throws Exception
    pub fn process(&mut self, mut sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        jcall!(self.pool).visit(sourceLayerInterface.clone());
    }
}
impl std::ops::Deref for Points20kPickedUpLayerInterfaceFactory {
    type Target = PickedUpLayerInterfaceFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for Points20kPickedUpLayerInterfaceFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactoryDyn for Points20kPickedUpLayerInterfaceFactory {
    fn as_PickedUpLayerInterfaceFactory(&self) -> &crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactory { &self.base }
    fn as_PickedUpLayerInterfaceFactory_mut(&mut self) -> &mut crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactory { &mut self.base }
}
impl crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryBase::PickedUpLayerInterfaceFactoryBaseDyn for Points20kPickedUpLayerInterfaceFactory {
    fn as_PickedUpLayerInterfaceFactoryBase(&self) -> &crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryBase::PickedUpLayerInterfaceFactoryBase { &self.base.base }
    fn as_PickedUpLayerInterfaceFactoryBase_mut(&mut self) -> &mut crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryBase::PickedUpLayerInterfaceFactoryBase { &mut self.base.base }
}
impl crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryInterface::PickedUpLayerInterfaceFactoryInterface for Points20kPickedUpLayerInterfaceFactory {
    fn getPickedUpLayerType(&mut self) -> Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickedUpLayerType::PickedUpLayerType>>> { crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactory::getPickedUpLayerType(crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactoryDyn::as_PickedUpLayerInterfaceFactory_mut(self)) }
    fn getIconLayer(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> { crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactory::getIconLayer(crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactoryDyn::as_PickedUpLayerInterfaceFactory_mut(self)) }
    fn getAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactory::getAnimationInterface(crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactoryDyn::as_PickedUpLayerInterfaceFactory_mut(self)) }
}



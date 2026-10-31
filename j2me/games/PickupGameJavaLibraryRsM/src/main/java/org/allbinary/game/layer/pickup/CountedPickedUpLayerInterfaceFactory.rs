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
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::NullAnimationFactory::NullAnimationFactory;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactory;
use super::CountedPickedUpLayerInterfaceFactoryInterface::CountedPickedUpLayerInterfaceFactoryInterface;
use super::PickedUpLayerType::PickedUpLayerType;
use super::IconLayer::IconLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CountedPickedUpLayerInterfaceFactory {
    pub base: PickedUpLayerInterfaceFactory,
    pub id: i32,
}

thread_local! { pub static CountedPickedUpLayerInterfaceFactory_NULL_COUNTED_PICKUP_LAYER_FACTORY: Option<Rc<RefCell<dyn CountedPickedUpLayerInterfaceFactoryDyn>>> = CountedPickedUpLayerInterfaceFactory::new(jcall!(PickedUpLayerTypeFactory::PickedUpLayerTypeFactory::getInstance()).NONE.clone(), IconLayer_NULL_ICON_LAYER.with(Clone::clone).clone(), jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0)).map(|reference| reference as Rc<RefCell<dyn CountedPickedUpLayerInterfaceFactoryDyn>>); }
// impl CountedPickedUpLayerInterfaceFactoryInterface for CountedPickedUpLayerInterfaceFactory with the methods of the trait

impl CountedPickedUpLayerInterfaceFactory {
    

    pub fn new_struct(mut pickeUpLayerType: Option<Rc<RefCell<PickedUpLayerType>>>, mut iconLayer: Option<Rc<RefCell<IconLayer>>>, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Self {
        let mut this = Self {
            base: PickedUpLayerInterfaceFactory::new_struct(pickeUpLayerType.clone(), iconLayer.clone(), animationInterface.clone()),
            id: 0,
        };
        // super(...) is in the struct literal above
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(CountedPickedUpLayerInterfaceFactoryPool::CountedPickedUpLayerInterfaceFactoryPool::getInstance()).getList();
        this.id = jcall!(list).size();
        this
    }

    pub fn new(mut pickeUpLayerType: Option<Rc<RefCell<PickedUpLayerType>>>, mut iconLayer: Option<Rc<RefCell<IconLayer>>>, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(pickeUpLayerType, iconLayer, animationInterface))));
            jcall!(list).add((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    // throws Exception
    pub fn getNextInstance(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut x: i32, mut y: i32, mut z: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(CommonStrings::getInstance()).NOT_IMPLEMENTED.clone())));
    }
    

    pub fn getTotal(&mut self) -> i32 {
        return -1;
    }
    

    pub fn getId(&mut self) -> i32 {
        return self.id.clone();
    }
}
impl std::ops::Deref for CountedPickedUpLayerInterfaceFactory {
    type Target = PickedUpLayerInterfaceFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CountedPickedUpLayerInterfaceFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait CountedPickedUpLayerInterfaceFactoryDyn: crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactoryDyn {
    fn as_CountedPickedUpLayerInterfaceFactory(&self) -> &CountedPickedUpLayerInterfaceFactory;
    fn as_CountedPickedUpLayerInterfaceFactory_mut(&mut self) -> &mut CountedPickedUpLayerInterfaceFactory;
    fn getNextInstance(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut x: i32, mut y: i32, mut z: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> { CountedPickedUpLayerInterfaceFactory::getNextInstance(self.as_CountedPickedUpLayerInterfaceFactory_mut(), hashtable, x, y, z) }
    fn getTotal(&mut self) -> i32 { CountedPickedUpLayerInterfaceFactory::getTotal(self.as_CountedPickedUpLayerInterfaceFactory_mut()) }
    fn getId(&mut self) -> i32 { CountedPickedUpLayerInterfaceFactory::getId(self.as_CountedPickedUpLayerInterfaceFactory_mut()) }
}
impl CountedPickedUpLayerInterfaceFactoryDyn for CountedPickedUpLayerInterfaceFactory {
    fn as_CountedPickedUpLayerInterfaceFactory(&self) -> &CountedPickedUpLayerInterfaceFactory { self }
    fn as_CountedPickedUpLayerInterfaceFactory_mut(&mut self) -> &mut CountedPickedUpLayerInterfaceFactory { self }
}

impl crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactoryDyn for CountedPickedUpLayerInterfaceFactory {
    fn as_PickedUpLayerInterfaceFactory(&self) -> &crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactory { &self.base }
    fn as_PickedUpLayerInterfaceFactory_mut(&mut self) -> &mut crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactory { &mut self.base }
}
impl crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryBase::PickedUpLayerInterfaceFactoryBaseDyn for CountedPickedUpLayerInterfaceFactory {
    fn as_PickedUpLayerInterfaceFactoryBase(&self) -> &crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryBase::PickedUpLayerInterfaceFactoryBase { &self.base.base }
    fn as_PickedUpLayerInterfaceFactoryBase_mut(&mut self) -> &mut crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryBase::PickedUpLayerInterfaceFactoryBase { &mut self.base.base }
}



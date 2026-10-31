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
use crate::org::allbinary::game::health::HealthInterfaceCompositeInterface::HealthInterfaceCompositeInterface;
use crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer;
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;
use crate::org::allbinary::game::part::CountedLayerInterfaceFactoryPart::CountedLayerInterfaceFactoryPart;
use crate::org::allbinary::game::part::PartInterface::PartInterface;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::PickupBehaviorInterface::PickupBehaviorInterface;
use super::PickupProcessorInterface::PickupProcessorInterface;
use super::PickedUpLayerInterfaceFactoryInterface::PickedUpLayerInterfaceFactoryInterface;
use super::PickedUpLayerType::PickedUpLayerType;
use super::PickedUpLayerTypeFactory::PickedUpLayerTypeFactory;
use super::CountedPickedUpLayerInterfaceFactoryInterface::CountedPickedUpLayerInterfaceFactoryInterface;
use super::PickedUpLayerInterface::PickedUpLayerInterface;
use super::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PickupBehavior {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub countedIndex: i32,
    pub ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>,
}

pub static mut PickupBehavior_NULL_PICKUP_BEHAVIOR: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());
impl PickupBehaviorInterface for PickupBehavior {
    fn doPickup(&mut self, mut pickupProcessorInterface: Option<Rc<RefCell<dyn PickupProcessorInterface>>>) { Self::doPickup(self, pickupProcessorInterface) }
    fn doPickupLayer(&mut self, mut pickupLayerInterface: Option<Rc<RefCell<dyn PickedUpLayerInterface>>>) { Self::doPickupLayer(self, pickupLayerInterface) }
}

impl PickupBehavior {
    

    pub fn getNullInstance() -> Option<Rc<RefCell<PickupBehavior>>> {
        if match (&unsafe { PickupBehavior_NULL_PICKUP_BEHAVIOR.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *PickupBehavior_NULL_PICKUP_BEHAVIOR = PickupBehavior::new(CollidableCompositeLayer::getNullInstance(), 0) };
        }
        return (unsafe { PickupBehavior_NULL_PICKUP_BEHAVIOR.clone() }).clone() /* as PickupBehavior */;
    }
    

    pub fn new_struct(mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut countedIndex: i32) -> Self {
        let mut this = Self {
            logUtil: None,
            countedIndex: 0,
            ownerLayerInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.ownerLayerInterface = ownerLayerInterface.clone();
        this.countedIndex = countedIndex.clone();
        this
    }

    pub fn new(mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut countedIndex: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(ownerLayerInterface, countedIndex))))
    }
    

    pub fn doPickup(&mut self, mut pickupProcessorInterface: Option<Rc<RefCell<dyn PickupProcessorInterface>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut healthInterfaceCompositeInterface: Option<Rc<RefCell<dyn HealthInterfaceCompositeInterface>>> = (self.ownerLayerInterface).clone() /* as HealthInterfaceCompositeInterface */;
            if jcall!(jcall!(healthInterfaceCompositeInterface).getHealthInterface()).isAlive() {
                jcall!(pickupProcessorInterface).process(self.ownerLayerInterface.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("doPickup"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn doPickupLayer(&mut self, mut pickupLayerInterface: Option<Rc<RefCell<dyn PickedUpLayerInterface>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut pickedUpLayerInterfaceFactoryInterface: Option<Rc<RefCell<dyn PickedUpLayerInterfaceFactoryInterface>>> = jcall!(pickupLayerInterface).getPickedUpLayerInterfaceFactoryInterface();
            let mut pickedUpLayerType: Option<Rc<RefCell<PickedUpLayerType>>> = jcall!(pickedUpLayerInterfaceFactoryInterface).getPickedUpLayerType();
            let mut pickedUpLayerTypeFactory: Option<Rc<RefCell<PickedUpLayerTypeFactory>>> = PickedUpLayerTypeFactory::getInstance();
            if match (&pickedUpLayerType, &jcall!(pickedUpLayerTypeFactory).BEAM) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&pickedUpLayerType, &jcall!(pickedUpLayerTypeFactory).MINE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&pickedUpLayerType, &jcall!(pickedUpLayerTypeFactory).PROJECTILE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&pickedUpLayerType, &jcall!(pickedUpLayerTypeFactory).ENHANCEMENT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.add(panic!("ClassCastException: a Rust dyn PickedUpLayerInterfaceFactoryInterface cannot be cast to dyn CountedPickedUpLayerInterfaceFactoryInterface"));
            } else if match (&jcall!(pickedUpLayerInterfaceFactoryInterface).getPickedUpLayerType(), &jcall!(pickedUpLayerTypeFactory).PART) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut collidableDestroyableDamageableLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> = (self.ownerLayerInterface).clone() /* as CollidableDestroyableDamageableLayer */;
                jcall!(collidableDestroyableDamageableLayer).addPart(pickedUpLayerInterfaceFactoryInterface.clone());
            } else if match (&pickedUpLayerType, &jcall!(pickedUpLayerTypeFactory).NONE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            } else {
                let mut pickupProcessorInterface: Option<Rc<RefCell<dyn PickupProcessorInterface>>> = panic!("ClassCastException: a Rust dyn PickedUpLayerInterfaceFactoryInterface cannot be cast to dyn PickupProcessorInterface");
                self.doPickup(pickupProcessorInterface.clone());
            }
            jcall!(pickupLayerInterface).setPickedUp();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            PreLogUtil::putOE(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("doPickup"), e.clone());
            }
        }
    }
    

    pub fn add(&mut self, mut countedPickedUpLayerInterfaceFactoryInterface: Option<Rc<RefCell<dyn CountedPickedUpLayerInterfaceFactoryInterface>>>) {
        let mut partIndex: i32 = jcall!(countedPickedUpLayerInterfaceFactoryInterface).getId() + self.countedIndex;
        self.addAt(countedPickedUpLayerInterfaceFactoryInterface.clone(), partIndex.clone());
    }
    

    pub fn addAt(&mut self, mut countedPickedUpLayerInterfaceFactoryInterface: Option<Rc<RefCell<dyn CountedPickedUpLayerInterfaceFactoryInterface>>>, mut partIndex: i32) {
        let mut collidableDestroyableDamageableLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> = (self.ownerLayerInterface).clone() /* as CollidableDestroyableDamageableLayer */;
        let mut countedLayerInterfaceFactory: Option<Rc<RefCell<CountedLayerInterfaceFactoryPart>>> = (jcall!(collidableDestroyableDamageableLayer).getPartInterfaceArray()[(partIndex) as usize]).clone() /* as CountedLayerInterfaceFactoryPart */;
        jcall!(countedLayerInterfaceFactory).setTotal(jcall!(countedLayerInterfaceFactory).getTotal() + jcall!(countedPickedUpLayerInterfaceFactoryInterface).getTotal());
    }
    

    pub fn getFirstAvailableCountedLayerInterfaceFactory(&mut self, mut pickedUpLayerTypeArray: Vec<Option<Rc<RefCell<PickedUpLayerType>>>>) -> Option<Rc<RefCell<CountedLayerInterfaceFactoryPart>>> {
        let mut collidableDestroyableDamageableLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> = (self.ownerLayerInterface).clone() /* as CollidableDestroyableDamageableLayer */;
        let mut partInterfaceArray: Vec<Option<Rc<RefCell<dyn PartInterface>>>> = jcall!(collidableDestroyableDamageableLayer).getPartInterfaceArray();
        let mut size: i32 = (partInterfaceArray.len() as i32).clone();
        let mut nextCountedLayerInterfaceFactory: Option<Rc<RefCell<CountedLayerInterfaceFactoryPart>>> = None;
        let mut countedPickedUpLayerInterfaceFactory: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactoryDyn>>> = None;
        for index in (self.countedIndex..size) {
            nextCountedLayerInterfaceFactory = (jcall!(collidableDestroyableDamageableLayer).getPartInterfaceArray()[(index) as usize]).clone() /* as CountedLayerInterfaceFactoryPart */;
            countedPickedUpLayerInterfaceFactory = jcall!(nextCountedLayerInterfaceFactory).getCountedPickedUpLayerInterfaceFactory();
            {
                let mut index2: i32 = (pickedUpLayerTypeArray.len() as i32).clone();
                while { index2 -= 1; index2 } >= 0 {
                if match (&jcall!(countedPickedUpLayerInterfaceFactory).getPickedUpLayerType(), &pickedUpLayerTypeArray[(index2) as usize]) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    if jcall!(nextCountedLayerInterfaceFactory).getTotal() > 0 {
                        return nextCountedLayerInterfaceFactory.clone();
                    }
                }
                }
            }
        }
        return crate::org::allbinary::game::part::CountedLayerInterfaceFactoryPart::CountedLayerInterfaceFactoryPart_NULL_COUNTED_LAYER_INTERFACE_FACTORY.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn getCountedLayerInterfaceFactoryForSlot(&mut self, mut slotIndex: i32) -> Option<Rc<RefCell<CountedLayerInterfaceFactoryPart>>> {
        let mut currentSlot: i32 = 0;
        let mut collidableDestroyableDamageableLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> = (self.ownerLayerInterface).clone() /* as CollidableDestroyableDamageableLayer */;
        let mut partInterfaceArray: Vec<Option<Rc<RefCell<dyn PartInterface>>>> = jcall!(collidableDestroyableDamageableLayer).getPartInterfaceArray();
        let mut size: i32 = (partInterfaceArray.len() as i32).clone();
        let mut nextCountedLayerInterfaceFactory: Option<Rc<RefCell<CountedLayerInterfaceFactoryPart>>> = None;
        for index in (self.countedIndex..size) {
            nextCountedLayerInterfaceFactory = (jcall!(collidableDestroyableDamageableLayer).getPartInterfaceArray()[(index) as usize]).clone() /* as CountedLayerInterfaceFactoryPart */;
            if jcall!(nextCountedLayerInterfaceFactory).getTotal() > 0 {
                if currentSlot == slotIndex {
                    return nextCountedLayerInterfaceFactory.clone();
                } else {
                    currentSlot += 1;
                }
            }
        }
        return crate::org::allbinary::game::part::CountedLayerInterfaceFactoryPart::CountedLayerInterfaceFactoryPart_NULL_COUNTED_LAYER_INTERFACE_FACTORY.with(Clone::clone).clone();
    }
    

    pub fn getCountedIndex(&mut self) -> i32 {
        return self.countedIndex.clone();
    }
}


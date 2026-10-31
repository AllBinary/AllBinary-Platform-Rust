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
// package org.allbinary.game.ag.ai.scroller
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::org::allbinary::game::ai::BasicAI::BasicAI;
use crate::org::allbinary::game::identification::BasicGroupFactory::BasicGroupFactory;
use crate::org::allbinary::game::input::GameInput::GameInput;
use crate::org::allbinary::game::layer::identification::GroupLayerManagerListener::GroupLayerManagerListener;
use crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::math::MathUtil::MathUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FlockingAI {
    pub base: BasicAI,
    pub allowedDistance: i32,
}

impl crate::org::allbinary::ai::ArtificialIntelligenceInterface::ArtificialIntelligenceInterface for FlockingAI {
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::graphics::IdentifierInterface::IdentifierInterface for FlockingAI {
    fn getId(&mut self) -> i32 { self.base.getId() }
}

impl FlockingAI {
    

    pub fn new_struct(mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Self {
        let mut this = Self {
            base: BasicAI::new_struct(ownerLayerInterface.clone(), gameInput.clone()),
            allowedDistance: 0,
        };
        // super(...) is in the struct literal above
        let mut allowedDistance: Option<i32> = { let java_object = jcall!(hashtable).get(crate::java_into_object(BasePatrolAI_MAX_DISTANCE.with(Clone::clone).clone())); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<i32>().copied().or_else(|| object.downcast_ref::<Option<i32>>().copied().flatten()).expect("ClassCastException") }) };
        this.allowedDistance = ((allowedDistance).unwrap() as i32);
        this
    }

    pub fn new(mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(hashtable, ownerLayerInterface, gameInput))))
    }
    

    // throws Exception
    pub fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        let mut maxDistance: i32 = 0;
        let mut farAllbinaryLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer_NULL_ALLBINARY_LAYER.with(Clone::clone).clone();
        let mut groupLayerManagerListener: Option<Rc<RefCell<GroupLayerManagerListener>>> = GroupLayerManagerListener::getInstance();
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(groupLayerManagerListener).getList(jcall!(BasicGroupFactory::getInstance()).ENEMY.clone());
        let mut size: i32 = jcall!(list).size();
        let mut allBinaryLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = None;
        let mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = None;
        for index in (0..size) {
            allBinaryLayer = { let java_object = (jcall!(list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>>().expect("ClassCastException").clone()) };
            if jcall!(allBinaryLayer).getType() != WeaponLayer::getStaticType() {
                let mut distance: i32 = self.getXYDistance(allBinaryLayer.clone());
                if distance > maxDistance {
                    maxDistance = distance.clone();
                    farAllbinaryLayer = allBinaryLayer.clone();
                    if distance > self.allowedDistance {
                        break;
                    }
                }
            }
        }
        if maxDistance > self.allowedDistance {
            if !match (&farAllbinaryLayer, &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer_NULL_ALLBINARY_LAYER.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                ownerLayerInterface = self.getOwnerLayerInterface();
                if jcall!(farAllbinaryLayer).getXP() < jcall!(ownerLayerInterface).getXP() {
                    self.processKeyAI(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }.clone());
                } else if jcall!(farAllbinaryLayer).getXP() > jcall!(ownerLayerInterface).getXP() {
                    self.processKeyAI(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }.clone());
                } else if jcall!(farAllbinaryLayer).getYP() > jcall!(ownerLayerInterface).getYP() {
                    self.processKeyAI(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_UP.clone() }.clone());
                } else if jcall!(farAllbinaryLayer).getYP() < jcall!(ownerLayerInterface).getYP() {
                    self.processKeyAI(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_DOWN.clone() }.clone());
                }
            }
        }
    }
    

    pub fn getXYDistance(&mut self, mut allBinaryLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> i32 {
        let mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = self.getOwnerLayerInterface();
        let mut xTotalDistance: i32 = (jcall!(allBinaryLayer).getXP() + jcall!(allBinaryLayer).getHalfWidth()) - (jcall!(ownerLayerInterface).getXP() + jcall!(ownerLayerInterface).getHalfWidth());
        let mut yTotalDistance: i32 = (jcall!(allBinaryLayer).getYP() + jcall!(allBinaryLayer).getHalfHeight()) - (jcall!(ownerLayerInterface).getYP() + jcall!(ownerLayerInterface).getHalfHeight());
        let mut mathUtil: Option<Rc<RefCell<MathUtil>>> = MathUtil::getInstance();
        let mut totalDistance: i32 = jcall!(mathUtil).abs(xTotalDistance.clone()) + jcall!(mathUtil).abs(yTotalDistance.clone());
        return totalDistance.clone();
    }
}
impl std::ops::Deref for FlockingAI {
    type Target = BasicAI;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for FlockingAI {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::ai::BasicAI::BasicAIDyn for FlockingAI {
    fn as_BasicAI(&self) -> &crate::org::allbinary::game::ai::BasicAI::BasicAI { &self.base }
    fn as_BasicAI_mut(&mut self) -> &mut crate::org::allbinary::game::ai::BasicAI::BasicAI { &mut self.base }
}
impl crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligenceDyn for FlockingAI {
    fn as_ArtificialIntelligence(&self) -> &crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &self.base.base }
    fn as_ArtificialIntelligence_mut(&mut self) -> &mut crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &mut self.base.base }
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for FlockingAI {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::ai::BasicAI::BasicAI::getSourceId(crate::org::allbinary::game::ai::BasicAI::BasicAIDyn::as_BasicAI_mut(self)) }
}



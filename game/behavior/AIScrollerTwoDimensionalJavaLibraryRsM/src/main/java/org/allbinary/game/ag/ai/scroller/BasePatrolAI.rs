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
use crate::org::allbinary::game::input::GameInput::GameInput;
use crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityProperties;
use crate::org::allbinary::game::physics::velocity::VelocityInterfaceCompositeInterface::VelocityInterfaceCompositeInterface;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::logic::math::MathUtil::MathUtil;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::util::ABHashtable::ABHashtable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasePatrolAI {
    pub base: BasicAI,
    pub maxDistance: Option<i32>,
    pub currentDistance: i32,
    pub xTotalDistance: i32,
    pub lastKeyDirection: i32,
    pub keyDirection: i32,
}

thread_local! { pub static BasePatrolAI_MAX_DISTANCE: Option<i32> = jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(2); }
thread_local! { pub static BasePatrolAI_DIRECTION_ARRAY: Option<i32> = jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(3); }
thread_local! { pub static BasePatrolAI_FIRING_DISTANCE: Option<i32> = jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(4); }
pub static mut BasePatrolAI_SPEED: std::cell::LazyCell<Option<i32>> = std::cell::LazyCell::new(|| jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(5));
impl crate::org::allbinary::graphics::IdentifierInterface::IdentifierInterface for BasePatrolAI {
    fn getId(&mut self) -> i32 { self.base.getId() }
}

impl BasePatrolAI {
    

    pub fn new_struct(mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Self {
        let mut this = Self {
            base: BasicAI::new_struct(ownerLayerInterface.clone(), gameInput.clone()),
            maxDistance: None,
            currentDistance: 0,
            xTotalDistance: 0,
            lastKeyDirection: 0,
            keyDirection: 0,
        };
        this.lastKeyDirection = unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }.clone();
        this.keyDirection = unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }.clone();
        // super(...) is in the struct literal above
        this.maxDistance = { let java_object = jcall!(hashtable).get(crate::java_into_object(BasePatrolAI_MAX_DISTANCE.with(Clone::clone).clone())); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<i32>().copied().or_else(|| object.downcast_ref::<Option<i32>>().copied().flatten()).expect("ClassCastException") }) };
        this.currentDistance = ((this.maxDistance).unwrap() as i32);
        this
    }

    pub fn new(mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(hashtable, ownerLayerInterface, gameInput))))
    }
    

    pub fn update(&mut self) {
        self.updateTotalDistance();
        self.changeDirectionIfReachedPacingAreaMax();
    }
    

    pub fn updateTotalDistance(&mut self) {
        let mut velocityInterfaceCompositeInterface: Option<Rc<RefCell<dyn VelocityInterfaceCompositeInterface>>> = (self.getOwnerLayerInterface()).clone() /* as VelocityInterfaceCompositeInterface */;
        let mut velocityProperties: Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>> = jcall!(velocityInterfaceCompositeInterface).getVelocityProperties();
        self.xTotalDistance += jcall!(jcall!(velocityProperties).getVelocityXBasicDecimalP()).getScaled();
    }
    

    pub fn changeDirectionIfReachedPacingAreaMax(&mut self) {
        let mut mathUtil: Option<Rc<RefCell<MathUtil>>> = MathUtil::getInstance();
        let mut totalDistance: i32 = jcall!(mathUtil).abs(self.xTotalDistance.clone());
        if totalDistance > self.currentDistance {
            self.nextDirection();
        }
    }
    

    pub fn nextDirection(&mut self) {
        if self.lastKeyDirection == unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() } {
            self.keyDirection = unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }.clone();
            self.lastKeyDirection = self.keyDirection.clone();
            self.xTotalDistance = 0;
        } else if self.lastKeyDirection == unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() } {
            self.keyDirection = unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }.clone();
            self.lastKeyDirection = self.keyDirection.clone();
            self.xTotalDistance = 0;
        }
    }
    

    pub fn disable(&mut self) {
    }
}
impl std::ops::Deref for BasePatrolAI {
    type Target = BasicAI;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasePatrolAI {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait BasePatrolAIDyn: crate::org::allbinary::game::ai::BasicAI::BasicAIDyn {
    fn as_BasePatrolAI(&self) -> &BasePatrolAI;
    fn as_BasePatrolAI_mut(&mut self) -> &mut BasePatrolAI;
    fn update(&mut self) { BasePatrolAI::update(self.as_BasePatrolAI_mut()) }
    fn updateTotalDistance(&mut self) { BasePatrolAI::updateTotalDistance(self.as_BasePatrolAI_mut()) }
    fn changeDirectionIfReachedPacingAreaMax(&mut self) { BasePatrolAI::changeDirectionIfReachedPacingAreaMax(self.as_BasePatrolAI_mut()) }
    fn nextDirection(&mut self) { BasePatrolAI::nextDirection(self.as_BasePatrolAI_mut()) }
    fn disable(&mut self) { BasePatrolAI::disable(self.as_BasePatrolAI_mut()) }
}
impl BasePatrolAIDyn for BasePatrolAI {
    fn as_BasePatrolAI(&self) -> &BasePatrolAI { self }
    fn as_BasePatrolAI_mut(&mut self) -> &mut BasePatrolAI { self }
}

impl crate::org::allbinary::game::ai::BasicAI::BasicAIDyn for BasePatrolAI {
    fn as_BasicAI(&self) -> &crate::org::allbinary::game::ai::BasicAI::BasicAI { &self.base }
    fn as_BasicAI_mut(&mut self) -> &mut crate::org::allbinary::game::ai::BasicAI::BasicAI { &mut self.base }
}
impl crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligenceDyn for BasePatrolAI {
    fn as_ArtificialIntelligence(&self) -> &crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &self.base.base }
    fn as_ArtificialIntelligence_mut(&mut self) -> &mut crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &mut self.base.base }
    fn processAI(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { crate::org::allbinary::game::ai::BasicAI::BasicAI::processAI(crate::org::allbinary::game::ai::BasicAI::BasicAIDyn::as_BasicAI_mut(self), p1) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for BasePatrolAI {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::ai::BasicAI::BasicAI::getSourceId(crate::org::allbinary::game::ai::BasicAI::BasicAIDyn::as_BasicAI_mut(self)) }
}



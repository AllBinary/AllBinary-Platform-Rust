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
use crate::org::allbinary::direction::Direction::Direction;
use crate::org::allbinary::direction::DirectionCompositeInterface::DirectionCompositeInterface;
use crate::org::allbinary::direction::DirectionFactory::DirectionFactory;
use crate::org::allbinary::game::input::GameInput::GameInput;
use crate::org::allbinary::game::tracking::TrackingEvent::TrackingEvent;
use crate::org::allbinary::game::tracking::TrackingEventHandler::TrackingEventHandler;
use crate::org::allbinary::game::tracking::TrackingEventListenerInterface::TrackingEventListenerInterface;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::layer::LayerInterface::LayerInterface;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::BasePatrolAI::BasePatrolAI;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PacePatrolAI {
    pub base: BasePatrolAI,
    pub trackingList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub firingDistance: i32,
    pub isFollowLimitedByTerrain: bool,
}

impl TrackingEventListenerInterface for PacePatrolAI {
    fn onMovement(&mut self, mut trackingEvent: Option<Rc<RefCell<TrackingEvent>>>) { Self::onMovement(self, trackingEvent) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for PacePatrolAI {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}
impl crate::org::allbinary::ai::ArtificialIntelligenceInterface::ArtificialIntelligenceInterface for PacePatrolAI {
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::graphics::IdentifierInterface::IdentifierInterface for PacePatrolAI {
    fn getId(&mut self) -> i32 { self.base.getId() }
}

impl PacePatrolAI {
    

    pub fn new_struct(mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Self {
        let mut this = Self {
            base: BasePatrolAI::new_struct(hashtable.clone(), ownerLayerInterface.clone(), gameInput.clone()),
            trackingList: None,
            firingDistance: 0,
            isFollowLimitedByTerrain: false,
        };
        this.isFollowLimitedByTerrain = false;
        // super(...) is in the struct literal above
        this.trackingList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new(mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(hashtable, ownerLayerInterface, gameInput))));
            jcall!(TrackingEventHandler::getInstance()).addListenerInterface((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    // throws Exception
    pub fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        self.update();
        let mut direction: Option<Rc<RefCell<Direction>>> = self.setFiringDirectionForTargetIfInRange();
        let mut directionFactory: Option<Rc<RefCell<DirectionFactory>>> = DirectionFactory::getInstance();
        if match (&direction, &jcall!(directionFactory).LEFT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if !match (&self.lastKeyDirection, &self.base.keyDirection) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || !self.isFollowLimitedByTerrain {
                self.base.processKeyAI(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }.clone());
            }
            self.base.keyDirection = unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() }.clone();
            self.lastKeyDirection = self.base.keyDirection.clone();
            self.base.xTotalDistance = 0;
            self.base.processKeyAI(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM1.clone() }.clone());
        } else if match (&direction, &jcall!(directionFactory).RIGHT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if !match (&self.lastKeyDirection, &self.base.keyDirection) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || !self.isFollowLimitedByTerrain {
                self.base.processKeyAI(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }.clone());
            }
            self.base.keyDirection = unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }.clone();
            self.lastKeyDirection = self.base.keyDirection.clone();
            self.base.xTotalDistance = 0;
            self.base.processKeyAI(unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM1.clone() }.clone());
        } else {
            self.base.processKeyAI(self.keyDirection.clone());
        }
    }
    

    pub fn setFiringDirectionForTargetIfInRange(&mut self) -> Option<Rc<RefCell<Direction>>> {
        let mut directionFactory: Option<Rc<RefCell<DirectionFactory>>> = DirectionFactory::getInstance();
        let mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = self.getOwnerLayerInterface();
        let mut direction: Option<Rc<RefCell<Direction>>> = jcall!(directionFactory).NOT_BORDERED_WITH.clone();
        let mut lastTrackingEvent: Option<Rc<RefCell<TrackingEvent>>> = None;
        let mut lastTrackingLayerInterface: Option<Rc<RefCell<dyn LayerInterface>>> = None;
        let mut directionCompositeInterface: Option<Rc<RefCell<dyn DirectionCompositeInterface>>> = None;
        let mut size: i32 = jcall!(self.trackingList).size();
        for index in (0..size) {
            lastTrackingEvent = { let java_object = (jcall!(self.trackingList).get(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<TrackingEvent>>>>().expect("ClassCastException").clone()) };
            lastTrackingLayerInterface = jcall!(lastTrackingEvent).getLayerInterface();
            let mut x: i32 = jcall!(lastTrackingLayerInterface).getXP();
            let mut y: i32 = jcall!(lastTrackingLayerInterface).getYP();
            let mut yDistance: i32 = jcall!(ownerLayerInterface).getYP() - y - jcall!(ownerLayerInterface).getHeight();
            let mut xDistance: i32 = jcall!(ownerLayerInterface).getXP() - x - jcall!(ownerLayerInterface).getWidth();
            let mut absXDistance: i32 = (xDistance.clone()).abs();
            let mut absYDistance: i32 = (yDistance.clone()).abs();
            if absYDistance <= 100 {
                if absXDistance < self.getFiringDistance() / 2 {
                    directionCompositeInterface = (self.getOwnerLayerInterface()).clone() /* as DirectionCompositeInterface */;
                    if xDistance < 0 && match (&jcall!(directionCompositeInterface).getDirection(), &jcall!(directionFactory).RIGHT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                        direction = jcall!(directionFactory).RIGHT.clone();
                    } else if xDistance > 0 && match (&jcall!(directionCompositeInterface).getDirection(), &jcall!(directionFactory).LEFT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                        direction = jcall!(directionFactory).LEFT.clone();
                    }
                }
            }
        }
        return direction.clone();
    }
    

    pub fn disable(&mut self) {
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log(jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    pub fn onMovement(&mut self, mut trackingEvent: Option<Rc<RefCell<TrackingEvent>>>) {
        jcall!(self.trackingList).clear();
        jcall!(self.trackingList).add(crate::java_into_object(trackingEvent.clone()));
    }
    

    pub fn getFiringDistance(&mut self) -> i32 {
        return self.firingDistance.clone();
    }
    

    pub fn setFiringDistance(&mut self, mut firingDistance: i32) {
        self.firingDistance = firingDistance.clone();
    }
}
impl std::ops::Deref for PacePatrolAI {
    type Target = BasePatrolAI;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PacePatrolAI {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait PacePatrolAIDyn: crate::org::allbinary::game::ag::ai::scroller::BasePatrolAI::BasePatrolAIDyn {
    fn as_PacePatrolAI(&self) -> &PacePatrolAI;
    fn as_PacePatrolAI_mut(&mut self) -> &mut PacePatrolAI;
    fn setFiringDirectionForTargetIfInRange(&mut self) -> Option<Rc<RefCell<Direction>>> { PacePatrolAI::setFiringDirectionForTargetIfInRange(self.as_PacePatrolAI_mut()) }
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { PacePatrolAI::onEvent(self.as_PacePatrolAI_mut(), eventObject) }
    fn onMovement(&mut self, mut trackingEvent: Option<Rc<RefCell<TrackingEvent>>>) { PacePatrolAI::onMovement(self.as_PacePatrolAI_mut(), trackingEvent) }
    fn getFiringDistance(&mut self) -> i32 { PacePatrolAI::getFiringDistance(self.as_PacePatrolAI_mut()) }
    fn setFiringDistance(&mut self, mut firingDistance: i32) { PacePatrolAI::setFiringDistance(self.as_PacePatrolAI_mut(), firingDistance) }
}
impl PacePatrolAIDyn for PacePatrolAI {
    fn as_PacePatrolAI(&self) -> &PacePatrolAI { self }
    fn as_PacePatrolAI_mut(&mut self) -> &mut PacePatrolAI { self }
}

impl crate::org::allbinary::game::ag::ai::scroller::BasePatrolAI::BasePatrolAIDyn for PacePatrolAI {
    fn as_BasePatrolAI(&self) -> &crate::org::allbinary::game::ag::ai::scroller::BasePatrolAI::BasePatrolAI { &self.base }
    fn as_BasePatrolAI_mut(&mut self) -> &mut crate::org::allbinary::game::ag::ai::scroller::BasePatrolAI::BasePatrolAI { &mut self.base }
    fn disable(&mut self) { Self::disable(self) }
}
impl crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligenceDyn for PacePatrolAI {
    fn as_ArtificialIntelligence(&self) -> &crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &self.base.base.base }
    fn as_ArtificialIntelligence_mut(&mut self) -> &mut crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &mut self.base.base.base }
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::game::ai::BasicAI::BasicAIDyn for PacePatrolAI {
    fn as_BasicAI(&self) -> &crate::org::allbinary::game::ai::BasicAI::BasicAI { &self.base.base }
    fn as_BasicAI_mut(&mut self) -> &mut crate::org::allbinary::game::ai::BasicAI::BasicAI { &mut self.base.base }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for PacePatrolAI {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::ai::BasicAI::BasicAI::getSourceId(crate::org::allbinary::game::ai::BasicAI::BasicAIDyn::as_BasicAI_mut(self)) }
}



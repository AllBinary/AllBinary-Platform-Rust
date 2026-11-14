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
// package org.allbinary.game.ag.ai
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::org::allbinary::ai::ArtificialIntelligenceInterface::ArtificialIntelligenceInterface;
use crate::org::allbinary::direction::Direction::Direction;
use crate::org::allbinary::direction::DirectionFactory::DirectionFactory;
use crate::org::allbinary::direction::DirectionalCompositeInterface::DirectionalCompositeInterface;
use crate::org::allbinary::direction::DirectionalInterface::DirectionalInterface;
use crate::org::allbinary::game::ai::BasicAI::BasicAI;
use crate::org::allbinary::game::combat::destroy::event::DestroyedEvent::DestroyedEvent;
use crate::org::allbinary::game::combat::destroy::event::DestroyedEventHandler::DestroyedEventHandler;
use crate::org::allbinary::game::combat::destroy::event::DestroyedEventListenerInterface::DestroyedEventListenerInterface;
use crate::org::allbinary::game::identification::Group::Group;
use crate::org::allbinary::game::input::GameInput::GameInput;
use crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityProperties;
use crate::org::allbinary::game::physics::velocity::VelocityInterfaceCompositeInterface::VelocityInterfaceCompositeInterface;
use crate::org::allbinary::game::tracking::TrackingEvent::TrackingEvent;
use crate::org::allbinary::game::tracking::TrackingEventHandler::TrackingEventHandler;
use crate::org::allbinary::game::tracking::TrackingEventListenerInterface::TrackingEventListenerInterface;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::math::MathUtil::MathUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::org::allbinary::logic::util::visitor::Visitor::Visitor;
use crate::org::allbinary::time::GameTickTimeDelayHelper::GameTickTimeDelayHelper;
use crate::org::allbinary::time::GameTickTimeDelayHelperFactory::GameTickTimeDelayHelperFactory;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DiveAndDirectionalTrackingAI {
    pub base: BasicAI,
    pub mathUtil: Option<Rc<RefCell<MathUtil>>>,
    pub directionFactory: Option<Rc<RefCell<DirectionFactory>>>,
    pub gameTickTimeDelayHelper: Option<Rc<RefCell<GameTickTimeDelayHelper>>>,
    pub timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub directionalInterface: Option<Rc<RefCell<dyn DirectionalInterface>>>,
    pub velocityInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>>,
    pub directionToKeyMap: Vec<i32>,
    pub initialDropped: bool,
    pub diveP: bool,
    pub directionOfTarget: Option<Rc<RefCell<Direction>>>,
    pub MIN_DISTANCE: i32,
    pub lastTrackingLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>,
    pub artificialIntelligenceInterface: Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>>,
    pub aiVistor: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn>>>,
    pub lastDirection: Option<Rc<RefCell<Direction>>>,
}

impl TrackingEventListenerInterface for DiveAndDirectionalTrackingAI {
    fn onMovement(&mut self, mut trackingEvent: Option<Rc<RefCell<TrackingEvent>>>) { Self::onMovement(self, trackingEvent) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for DiveAndDirectionalTrackingAI {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}
impl DestroyedEventListenerInterface for DiveAndDirectionalTrackingAI {
    fn onDestroyed(&mut self, mut destroyedEvent: Option<Rc<RefCell<DestroyedEvent>>>) { Self::onDestroyed(self, destroyedEvent) }
}
impl ArtificialIntelligenceInterface for DiveAndDirectionalTrackingAI {
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::graphics::IdentifierInterface::IdentifierInterface for DiveAndDirectionalTrackingAI {
    fn getId(&mut self) -> i32 { crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence::getId(crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligenceDyn::as_ArtificialIntelligence_mut(self)) }
}

impl DiveAndDirectionalTrackingAI {
    

    pub fn new_struct(mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut artificialIntelligenceInterface: Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>, mut visitor: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn>>>) -> Self {
        let mut this = Self {
            base: BasicAI::new_struct(ownerLayerInterface.clone(), gameInput.clone()),
            mathUtil: None,
            directionFactory: None,
            gameTickTimeDelayHelper: None,
            timeDelayHelper: None,
            list: None,
            directionalInterface: None,
            velocityInterface: None,
            directionToKeyMap: Vec::new(),
            initialDropped: false,
            diveP: false,
            directionOfTarget: None,
            MIN_DISTANCE: 0,
            lastTrackingLayerInterface: None,
            artificialIntelligenceInterface: None,
            aiVistor: None,
            lastDirection: None,
        };
        this.mathUtil = MathUtil::getInstance();
        this.directionFactory = DirectionFactory::getInstance();
        this.gameTickTimeDelayHelper = GameTickTimeDelayHelperFactory::getInstance();
        this.timeDelayHelper = TimeDelayHelper::new(500).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.directionToKeyMap = vec![crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.with(Clone::clone).clone(), crate::javax::microedition::lcdui::Canvas::Canvas_DOWN.with(Clone::clone).clone(), crate::javax::microedition::lcdui::Canvas::Canvas_UP.with(Clone::clone).clone(), crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.with(Clone::clone).clone()];
        this.directionOfTarget = Direction::getInstance(0);
        this.MIN_DISTANCE = 40;
        this.lastTrackingLayerInterface = crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer_NULL_ALLBINARY_LAYER.with(Clone::clone).clone();
        this.lastDirection = { let java_field = jcall!(DirectionFactory::getInstance()).NO_DIRECTION.clone(); java_field }.clone();
        // super(...) is in the struct literal above
        this.aiVistor = visitor.clone();
        this.artificialIntelligenceInterface = artificialIntelligenceInterface.clone();
        let mut directionalCompositeInterface: Option<Rc<RefCell<dyn DirectionalCompositeInterface>>> = (|| -> Option<Rc<RefCell<dyn DirectionalCompositeInterface>>> { panic!("ClassCastException: a Rust dyn AllBinaryLayer cannot be cast to DirectionalCompositeInterface") })();
        this.directionalInterface = { let java_value = (jcall!(directionalCompositeInterface).getDirectionalInterface()).clone(); java_value };
        let mut velocityInterfaceCompositeInterface: Option<Rc<RefCell<dyn VelocityInterfaceCompositeInterface>>> = (|| -> Option<Rc<RefCell<dyn VelocityInterfaceCompositeInterface>>> { panic!("ClassCastException: a Rust dyn AllBinaryLayer cannot be cast to VelocityInterfaceCompositeInterface") })();
        this.velocityInterface = { let java_value = (jcall!(velocityInterfaceCompositeInterface).getVelocityProperties()).clone(); java_value };
        this
    }

    pub fn new(mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut artificialIntelligenceInterface: Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>, mut visitor: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(ownerLayerInterface.clone(), artificialIntelligenceInterface.clone(), gameInput.clone(), visitor.clone()))));
            let mut directionalCompositeInterface: Option<Rc<RefCell<dyn DirectionalCompositeInterface>>> = (|| -> Option<Rc<RefCell<dyn DirectionalCompositeInterface>>> { panic!("ClassCastException: a Rust dyn AllBinaryLayer cannot be cast to DirectionalCompositeInterface") })();
            let mut velocityInterfaceCompositeInterface: Option<Rc<RefCell<dyn VelocityInterfaceCompositeInterface>>> = (|| -> Option<Rc<RefCell<dyn VelocityInterfaceCompositeInterface>>> { panic!("ClassCastException: a Rust dyn AllBinaryLayer cannot be cast to VelocityInterfaceCompositeInterface") })();
            jcall!(DestroyedEventHandler::getInstance()).addListenerInterface(this.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>));
            jcall!(this).init();
        this
    }
    

    // throws Exception
    pub fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        if self.isBeyondTarget() {
            self.init();
            jcall!(self.velocityInterface).as_BasicVelocityProperties_mut().zero();
        }
        if !self.diveP {
            self.target(allBinaryLayerManager.clone());
        } else {
            self.dive();
            self.attack();
        }
        jcall!(self.list).clear();
    }
    

    pub fn init(&mut self) {
        self.diveP = false;
        self.directionOfTarget = { let java_value = ({ let java_field = jcall!(DirectionFactory::getInstance()).NOT_BORDERED_WITH.clone(); java_field }).clone(); java_value }.clone();
        jcall!(TrackingEventHandler::getInstance()).addListenerInterface(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    // throws Exception
    pub fn target(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        if jcall!(self.list).size() == 0 {
            return;
        }
        let mut lastTrackingEvent: Option<Rc<RefCell<TrackingEvent>>> = { let java_object = (jcall!(self.list).removeAt(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<TrackingEvent>>>>().expect("ClassCastException").clone()) };
        self.lastTrackingLayerInterface = { let java_value = (jcall!(lastTrackingEvent).getLayerInterface()).clone(); java_value };
        let mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = self.lastTrackingLayerInterface.clone();
        let mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = self.getOwnerLayerInterface();
        let mut groupArray: Vec<Option<Rc<RefCell<Group>>>> = jcall!(layerInterface).getGroupInterface();
        let mut ownerGroupArray: Vec<Option<Rc<RefCell<Group>>>> = jcall!(ownerLayerInterface).getGroupInterface();
        if crate::java::lang::Object::java_array_same(&groupArray, &ownerGroupArray) {
            return;
        }
        let mut x: i32 = jcall!(self.lastTrackingLayerInterface).getXP();
        let mut y: i32 = jcall!(self.lastTrackingLayerInterface).getYP();
        let mut yDistance: i32 = jcall!(ownerLayerInterface).getYP() - y - jcall!(ownerLayerInterface).getHeight();
        let mut xDistance: i32 = jcall!(ownerLayerInterface).getXP() - x - jcall!(ownerLayerInterface).getWidth();
        let mut directionFactory: Option<Rc<RefCell<DirectionFactory>>> = DirectionFactory::getInstance();
        if jcall!(self.mathUtil).abs(yDistance.clone()) > jcall!(self.mathUtil).abs(xDistance.clone()) {
            if yDistance > self.MIN_DISTANCE {
                self.directionOfTarget = { let java_value = ({ let java_field = jcall!(directionFactory).UP.clone(); java_field }).clone(); java_value }.clone();
            } else if yDistance < -self.MIN_DISTANCE {
                self.directionOfTarget = { let java_value = ({ let java_field = jcall!(directionFactory).DOWN.clone(); java_field }).clone(); java_value }.clone();
            }
        } else {
            if xDistance > self.MIN_DISTANCE {
                self.directionOfTarget = { let java_value = ({ let java_field = jcall!(directionFactory).LEFT.clone(); java_field }).clone(); java_value }.clone();
            } else if xDistance < -self.MIN_DISTANCE {
                self.directionOfTarget = { let java_value = ({ let java_field = jcall!(directionFactory).RIGHT.clone(); java_field }).clone(); java_value }.clone();
            }
        }
        if !self.initialDropped && !match (&self.directionOfTarget, &{ let java_field = jcall!(directionFactory).NOT_BORDERED_WITH.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.drop();
        }
        if self.initialDropped {
            if match (&self.directionOfTarget, &{ let java_field = jcall!(directionFactory).UP.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&self.directionOfTarget, &{ let java_field = jcall!(directionFactory).DOWN.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.verticalTargeting(x.clone());
            } else if match (&self.directionOfTarget, &{ let java_field = jcall!(directionFactory).LEFT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&self.directionOfTarget, &{ let java_field = jcall!(directionFactory).RIGHT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.horizontalTargeting(y.clone());
            } else {
                jcall!(self.artificialIntelligenceInterface).processAI(allBinaryLayerManager.clone());
            }
        }
    }
    

    // throws Exception
    pub fn verticalTargeting(&mut self, mut x: i32) {
        let mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = self.getOwnerLayerInterface();
        if jcall!(ownerLayerInterface).getXP() < x {
            self.moveRight();
        }
        if jcall!(ownerLayerInterface).getXP() > x {
            self.moveLeft();
        }
        if jcall!(ownerLayerInterface).getXP() > x - 3 && jcall!(ownerLayerInterface).getXP() < x + 3 {
            self.setDive();
        }
    }
    

    // throws Exception
    pub fn horizontalTargeting(&mut self, mut y: i32) {
        let mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = self.getOwnerLayerInterface();
        if jcall!(ownerLayerInterface).getYP() < y {
            self.moveDown();
        }
        if jcall!(ownerLayerInterface).getYP() > y {
            self.moveUp();
        }
        if jcall!(ownerLayerInterface).getYP() > y - 3 && jcall!(ownerLayerInterface).getYP() < y + 3 {
            self.setDive();
        }
    }
    

    pub fn setDive(&mut self) {
        self.diveP = true;
        jcall!(self.velocityInterface).as_BasicVelocityProperties_mut().zero();
        jcall!(TrackingEventHandler::getInstance()).removeListener(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    // throws Exception
    pub fn moveRight(&mut self) {
        { let java_arguments = ({ let java_field = jcall!(self.directionFactory).RIGHT.clone(); java_field }.clone(),); self.setLastDirection(java_arguments.0) };
        jcall!(self.directionalInterface).setFrameByDirection(self.lastDirection.clone());
        jcall!(self.aiVistor).as_Visitor_mut().visit(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    // throws Exception
    pub fn moveLeft(&mut self) {
        { let java_arguments = ({ let java_field = jcall!(self.directionFactory).LEFT.clone(); java_field }.clone(),); self.setLastDirection(java_arguments.0) };
        jcall!(self.directionalInterface).setFrameByDirection(self.lastDirection.clone());
        jcall!(self.aiVistor).as_Visitor_mut().visit(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    // throws Exception
    pub fn moveDown(&mut self) {
        { let java_arguments = ({ let java_field = jcall!(self.directionFactory).DOWN.clone(); java_field }.clone(),); self.setLastDirection(java_arguments.0) };
        jcall!(self.directionalInterface).setFrameByDirection(self.lastDirection.clone());
        jcall!(self.aiVistor).as_Visitor_mut().visit(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    // throws Exception
    pub fn moveUp(&mut self) {
        { let java_arguments = ({ let java_field = jcall!(self.directionFactory).UP.clone(); java_field }.clone(),); self.setLastDirection(java_arguments.0) };
        jcall!(self.directionalInterface).setFrameByDirection(self.lastDirection.clone());
        jcall!(self.aiVistor).as_Visitor_mut().visit(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    // throws Exception
    pub fn dive(&mut self) {
        jcall!(self.directionalInterface).setFrameByDirection(self.directionOfTarget.clone());
        jcall!(self.aiVistor).as_Visitor_mut().visit(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    // throws Exception
    pub fn attack(&mut self) {
        { let java_arguments = (crate::javax::microedition::lcdui::Canvas::Canvas_KEY_NUM1.with(Clone::clone).clone(),); self.base.processKeyAI(java_arguments.0) };
    }
    

    // throws Exception
    pub fn drop(&mut self) {
        if jcall!(self.timeDelayHelper).isTime({ let java_field = jcall!(self.gameTickTimeDelayHelper).startTime.clone(); java_field }.clone()) {
            self.initialDropped = true;
            jcall!(self.aiVistor).as_Visitor_mut().visit(None /* Java this: a &mut self method has no Rc of itself */);
        }
    }
    

    pub fn isBeyondTarget(&mut self) -> bool {
        if match (&self.directionOfTarget, &{ let java_field = jcall!(self.directionFactory).DOWN.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if { let java_value = jcall!(self.getOwnerLayerInterface()).getYP(); java_value } > jcall!(self.lastTrackingLayerInterface).getYP() + self.MIN_DISTANCE {
                return true;
            } else {
                return false;
            }
        } else if match (&self.directionOfTarget, &{ let java_field = jcall!(self.directionFactory).UP.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if { let java_value = jcall!(self.getOwnerLayerInterface()).getYP(); java_value } < jcall!(self.lastTrackingLayerInterface).getYP() + self.MIN_DISTANCE {
                return true;
            } else {
                return false;
            }
        } else if match (&self.directionOfTarget, &{ let java_field = jcall!(self.directionFactory).RIGHT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if { let java_value = jcall!(self.getOwnerLayerInterface()).getXP(); java_value } > jcall!(self.lastTrackingLayerInterface).getXP() + self.MIN_DISTANCE {
                return true;
            } else {
                return false;
            }
        } else if match (&self.directionOfTarget, &{ let java_field = jcall!(self.directionFactory).LEFT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if { let java_value = jcall!(self.getOwnerLayerInterface()).getXP(); java_value } < jcall!(self.lastTrackingLayerInterface).getXP() + self.MIN_DISTANCE {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log({ let java_field = jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    pub fn onMovement(&mut self, mut trackingEvent: Option<Rc<RefCell<TrackingEvent>>>) {
        jcall!(self.list).add(crate::java_into_object(trackingEvent.clone()));
    }
    

    pub fn onDestroyed(&mut self, mut destroyedEvent: Option<Rc<RefCell<DestroyedEvent>>>) {
        if match (&self.getOwnerLayerInterface(), &{ let java_value = jcall!(destroyedEvent).getLayerInterface(); java_value }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(TrackingEventHandler::getInstance()).removeListener(None /* Java this: a &mut self method has no Rc of itself */);
            jcall!(DestroyedEventHandler::getInstance()).removeListener(None /* Java this: a &mut self method has no Rc of itself */);
        }
    }
    

    pub fn setLastDirection(&mut self, mut lastDirection: Option<Rc<RefCell<Direction>>>) {
        self.lastDirection = lastDirection.clone();
        let mut value: i32 = jcall!(self.getLastDirection()).getValue();
        if value < 4 {
            { let java_arguments = (self.directionToKeyMap[(value) as usize].clone(),); self.base.setLastKey(java_arguments.0) };
        }
    }
    

    pub fn setLastKey(&mut self, mut lastKey: i32) {
        { let java_arguments = (lastKey.clone(),); self.base.setLastKey(java_arguments.0) };
        if self.getLastKey() == crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.with(Clone::clone) {
            self.lastDirection = { let java_value = ({ let java_field = jcall!(DirectionFactory::getInstance()).LEFT.clone(); java_field }).clone(); java_value }.clone();
        } else if self.getLastKey() == crate::javax::microedition::lcdui::Canvas::Canvas_DOWN.with(Clone::clone) {
            self.lastDirection = { let java_value = ({ let java_field = jcall!(DirectionFactory::getInstance()).DOWN.clone(); java_field }).clone(); java_value }.clone();
        } else if self.getLastKey() == crate::javax::microedition::lcdui::Canvas::Canvas_UP.with(Clone::clone) {
            self.lastDirection = { let java_value = ({ let java_field = jcall!(DirectionFactory::getInstance()).UP.clone(); java_field }).clone(); java_value }.clone();
        } else if self.getLastKey() == crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.with(Clone::clone) {
            self.lastDirection = { let java_value = ({ let java_field = jcall!(DirectionFactory::getInstance()).RIGHT.clone(); java_field }).clone(); java_value }.clone();
        }
    }
    

    pub fn getLastDirection(&mut self) -> Option<Rc<RefCell<Direction>>> {
        return self.lastDirection.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(self.base.toString());
        jcall!(stringBuffer).append(String::from(" LastDirection: "));
        jcall!(stringBuffer).append({ let java_argument = jcall!(StringUtil::getInstance()).toString(crate::java_into_object(self.getLastDirection())); java_argument });
        return jcall!(stringBuffer).toString();
    }
}
impl std::ops::Deref for DiveAndDirectionalTrackingAI {
    type Target = BasicAI;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DiveAndDirectionalTrackingAI {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligenceDyn for DiveAndDirectionalTrackingAI {
    fn as_ArtificialIntelligence(&self) -> &crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &self.base.base }
    fn as_ArtificialIntelligence_mut(&mut self) -> &mut crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence { &mut self.base.base }
    fn processAI(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processAI(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::game::ai::BasicAI::BasicAIDyn for DiveAndDirectionalTrackingAI {
    fn as_BasicAI(&self) -> &crate::org::allbinary::game::ai::BasicAI::BasicAI { &self.base }
    fn as_BasicAI_mut(&mut self) -> &mut crate::org::allbinary::game::ai::BasicAI::BasicAI { &mut self.base }
    fn setLastKey(&mut self, mut lastKey: i32) { Self::setLastKey(self, lastKey) }
}
impl crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface for DiveAndDirectionalTrackingAI {
    fn getSourceId(&mut self) -> i32 { crate::org::allbinary::game::ai::BasicAI::BasicAI::getSourceId(crate::org::allbinary::game::ai::BasicAI::BasicAIDyn::as_BasicAI_mut(self)) }
}



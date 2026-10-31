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
// package org.allbinary.input.motion.gesture
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::Line::Line;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::input::motion::gesture::configuration::MotionGestureConfiguration::MotionGestureConfiguration;
use crate::org::allbinary::input::motion::gesture::configuration::MotionGestureConfigurationFactory::MotionGestureConfigurationFactory;
use crate::org::allbinary::input::motion::gesture::observer::BasicMotionGesturesHandler::BasicMotionGesturesHandler;
use crate::org::allbinary::input::motion::gesture::observer::MotionEventCircularPool::MotionEventCircularPool;
use crate::org::allbinary::input::motion::gesture::observer::MotionGestureEvent::MotionGestureEvent;
use crate::org::allbinary::input::motion::gesture::observer::MovedMotionGesturesHandler::MovedMotionGesturesHandler;
use crate::org::allbinary::input::motion::gesture::observer::ScrolledMotionGesturesHandler::ScrolledMotionGesturesHandler;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::math::J2SEMath::J2SEMath;
use crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::TouchMotionGestureFactory::TouchMotionGestureFactory;
use super::MotionGestureInput::MotionGestureInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MotionGestureRecognizer {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub j2seMath: Option<Rc<RefCell<J2SEMath>>>,
    pub origin: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>,
    pub previous: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>,
    pub intermediate: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>,
    pub line: Option<Rc<RefCell<Line>>>,
    pub motionGesturesHandler: Option<Rc<RefCell<BasicMotionGesturesHandler>>>,
    pub movedMotionGesturesHandler: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn>>>,
    pub scrolledMotionGesturesHandler: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn>>>,
    pub motionEventCircularPool: Option<Rc<RefCell<MotionEventCircularPool>>>,
}


impl MotionGestureRecognizer {
    

    pub fn new_struct(mut id: i32) -> Self {
        let mut this = Self {
            logUtil: None,
            j2seMath: None,
            origin: None,
            previous: None,
            intermediate: None,
            line: None,
            motionGesturesHandler: None,
            movedMotionGesturesHandler: None,
            scrolledMotionGesturesHandler: None,
            motionEventCircularPool: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.j2seMath = J2SEMath::getInstance();
        this.origin = jcall!(PointFactory::getInstance()).ZERO_ZERO.clone();
        this.previous = this.origin.clone();
        this.intermediate = this.origin.clone();
        this.line = Line::new(this.origin.clone(), this.origin.clone());
        this.motionEventCircularPool = MotionEventCircularPool::createPool(id.clone());
        let mut motionGesturesHandler: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn>>> = BasicEventHandler::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn>>);
        let mut movedMotionGesturesHandler: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn>>> = motionGesturesHandler.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn>>);
        let mut scrolledMotionGesturesHandler: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn>>> = motionGesturesHandler.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn>>);
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            motionGesturesHandler = BasicMotionGesturesHandler::getInstance();
            movedMotionGesturesHandler = MovedMotionGesturesHandler::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn>>);
            scrolledMotionGesturesHandler = ScrolledMotionGesturesHandler::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn>>);
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(this.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(e.clone()));
            }
        }
        this.motionGesturesHandler = (motionGesturesHandler).clone() /* as BasicMotionGesturesHandler */;
        this.movedMotionGesturesHandler = movedMotionGesturesHandler.clone();
        this.scrolledMotionGesturesHandler = scrolledMotionGesturesHandler.clone();
        this
    }

    pub fn new(mut id: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(id))))
    }
    

    // throws Exception
    pub fn processPressedMotionEvent(&mut self, mut current: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut deviceId: i32, mut button: i32) -> bool {
        self.intermediate = self.origin.clone();
        self.previous = self.origin.clone();
        let mut event: Option<Rc<RefCell<MotionGestureEvent>>> = { let java_receiver = self.motionEventCircularPool.clone(); let java_arguments = (jcall!(TouchMotionGestureFactory::getInstance()).PRESSED.clone(),); let java_result = jcall!(java_receiver).getInstance(java_arguments.0); java_result };
        jcall!(event).setPreviousPoint(self.previous.clone());
        jcall!(event).setCurrentPoint(current.clone());
        jcall!(self.motionGesturesHandler).fireEvent(event.clone());
        return true;
    }
    

    // throws Exception
    pub fn processReleasedMotionEvent(&mut self, mut current: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut deviceId: i32, mut button: i32) -> bool {
        let mut event: Option<Rc<RefCell<MotionGestureEvent>>> = { let java_receiver = self.motionEventCircularPool.clone(); let java_arguments = (jcall!(TouchMotionGestureFactory::getInstance()).RELEASED.clone(),); let java_result = jcall!(java_receiver).getInstance(java_arguments.0); java_result };
        jcall!(event).setPreviousPoint(self.previous.clone());
        jcall!(event).setCurrentPoint(current.clone());
        jcall!(self.motionGesturesHandler).fireEvent(event.clone());
        return true;
    }
    

    // throws Exception
    pub fn processDraggedMotionEvent(&mut self, mut current: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut deviceId: i32, mut buttonMask: i32) {
        if match (&self.previous, &self.origin) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&self.intermediate, &self.origin) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.previous = current.clone();
            self.intermediate = current.clone();
            return;
        }
        jcall!(self.line).setP1(self.previous.clone());
        jcall!(self.line).setP2(current.clone());
        let mut minimumMotionGesture: i32 = jcall!(MotionGestureConfigurationFactory::getInstance()).getMinimumMotionGesture();
        if { let java_receiver = self.j2seMath.clone(); let java_arguments = (((jcall!(self.line).getDeltaX()) as f32),); let java_result = jcall!(java_receiver).abs(java_arguments.0); java_result } < ((minimumMotionGesture) as f32) && { let java_receiver = self.j2seMath.clone(); let java_arguments = (((jcall!(self.line).getDeltaY()) as f32),); let java_result = jcall!(java_receiver).abs(java_arguments.0); java_result } < ((minimumMotionGesture) as f32) {
            self.intermediate = current.clone();
            return;
        }
        let mut gradient: f64 = jcall!(self.line).getGradient();
        let mut absGradient: f64 = ((jcall!(self.j2seMath).abs(((gradient) as f32))) as f64);
        let mut conf: Option<Rc<RefCell<MotionGestureConfiguration>>> = MotionGestureConfigurationFactory::getInstance();
        let mut touchMotionGestureFactory: Option<Rc<RefCell<TouchMotionGestureFactory>>> = TouchMotionGestureFactory::getInstance();
        let mut newMotionGesture: Option<Rc<RefCell<MotionGestureInput>>> = jcall!(touchMotionGestureFactory).NO_MOTION.clone();
        let mut diagonalToleranceHigher: f64 = 12.0;
        let mut diagonalToleranceLower: f64 = 12.0;
        if jcall!(conf).isDiagonalMotionGestureAllowed() {
            diagonalToleranceHigher = (((90 - jcall!(conf).getDiagonalTolerance())) as f64);
            diagonalToleranceLower = ((jcall!(conf).getDiagonalTolerance()) as f64);
        }
        if absGradient > ((((diagonalToleranceHigher.clone()) as f64).to_radians()) as f64).tan() {
            if jcall!(self.line).getDeltaY() > 0.0 {
                newMotionGesture = jcall!(touchMotionGestureFactory).UP.clone();
            } else {
                newMotionGesture = jcall!(touchMotionGestureFactory).DOWN.clone();
            }
        } else {
            if absGradient < ((((diagonalToleranceLower.clone()) as f64).to_radians()) as f64).tan() {
                if jcall!(self.line).getDeltaX() > 0.0 {
                    newMotionGesture = jcall!(touchMotionGestureFactory).LEFT.clone();
                } else {
                    newMotionGesture = jcall!(touchMotionGestureFactory).RIGHT.clone();
                }
            } else {
                if gradient > 0.0 {
                    if jcall!(self.line).getDeltaX() > 0.0 {
                        newMotionGesture = jcall!(touchMotionGestureFactory).DIAGONAL_UP_LEFT.clone();
                    } else {
                        newMotionGesture = jcall!(touchMotionGestureFactory).DIAGONAL_DOWN_RIGHT.clone();
                    }
                } else {
                    if jcall!(self.line).getDeltaX() > 0.0 {
                        newMotionGesture = jcall!(touchMotionGestureFactory).DIAGONAL_DOWN_LEFT.clone();
                    } else {
                        newMotionGesture = jcall!(touchMotionGestureFactory).DIAGONAL_UP_RIGHT.clone();
                    }
                }
            }
        }
        self.previous = current.clone();
        self.intermediate = current.clone();
        let mut event: Option<Rc<RefCell<MotionGestureEvent>>> = jcall!(self.motionEventCircularPool).getInstance(newMotionGesture.clone());
        jcall!(event).setPreviousPoint(self.previous.clone());
        jcall!(event).setCurrentPoint(current.clone());
        jcall!(self.motionGesturesHandler).fireEvent(event.clone());
        return;
    }
    

    // throws Exception
    pub fn processMovedMotionEvent(&mut self, mut current: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut deviceId: i32, mut button: i32) -> bool {
        let mut event: Option<Rc<RefCell<MotionGestureEvent>>> = { let java_receiver = self.motionEventCircularPool.clone(); let java_arguments = (jcall!(TouchMotionGestureFactory::getInstance()).NO_MOTION.clone(),); let java_result = jcall!(java_receiver).getInstance(java_arguments.0); java_result };
        jcall!(event).setPreviousPoint(self.previous.clone());
        jcall!(event).setCurrentPoint(current.clone());
        jcall!(self.movedMotionGesturesHandler).fireEvent(event.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
        return true;
    }
    

    // throws Exception
    pub fn processScrolledMotionEvent(&mut self, mut current: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut deviceId: i32, mut button: i32) -> bool {
        let mut touchMotionGestureFactory: Option<Rc<RefCell<TouchMotionGestureFactory>>> = TouchMotionGestureFactory::getInstance();
        let mut newMotionGesture: Option<Rc<RefCell<MotionGestureInput>>> = jcall!(touchMotionGestureFactory).NO_MOTION.clone();
        if button > 0 {
            newMotionGesture = jcall!(touchMotionGestureFactory).SCROLL_UP.clone();
        } else if button < 0 {
            newMotionGesture = jcall!(touchMotionGestureFactory).SCROLL_DOWN.clone();
        }
        let mut event: Option<Rc<RefCell<MotionGestureEvent>>> = jcall!(self.motionEventCircularPool).getInstance(newMotionGesture.clone());
        jcall!(event).setPreviousPoint(self.previous.clone());
        jcall!(event).setCurrentPoint(current.clone());
        jcall!(self.scrolledMotionGesturesHandler).fireEvent(event.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
        return true;
    }
    

    pub fn getMotionGesturesHandler(&mut self) -> Option<Rc<RefCell<BasicMotionGesturesHandler>>> {
        return self.motionGesturesHandler.clone();
    }
}


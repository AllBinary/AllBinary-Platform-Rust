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
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::input::motion::gesture::observer::MotionEventCircularPool::MotionEventCircularPool;
use crate::org::allbinary::input::motion::gesture::observer::MotionGestureEvent::MotionGestureEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MirrorMotionGestureEventUtil {
    pub motionEventCircularPool: Option<Rc<RefCell<MotionEventCircularPool>>>,
}

thread_local! { pub static MirrorMotionGestureEventUtil_instance: Option<Rc<RefCell<MirrorMotionGestureEventUtil>>> = MirrorMotionGestureEventUtil::new(); }

impl MirrorMotionGestureEventUtil {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            motionEventCircularPool: None,
        };
        this.motionEventCircularPool = MotionEventCircularPool::createPool(i32::MAX.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn mirrorVerticle(&mut self, mut motionGestureEvent: Option<Rc<RefCell<MotionGestureEvent>>>, mut halfWidth: i32) -> Option<Rc<RefCell<MotionGestureEvent>>> {
        let mut event: Option<Rc<RefCell<MotionGestureEvent>>> = { let java_receiver = self.motionEventCircularPool.clone(); let java_arguments = (jcall!(motionGestureEvent).getMotionGesture(),); let java_result = jcall!(java_receiver).getInstance(java_arguments.0); java_result };
        let mut previousNewX: i32 = { let java_arguments = (jcall!(jcall!(motionGestureEvent).getPreviousPoint()).getX(), halfWidth.clone(),); self.getNewX(java_arguments.0, java_arguments.1) };
        let mut currentNewX: i32 = { let java_arguments = (jcall!(jcall!(motionGestureEvent).getCurrentPoint()).getX(), halfWidth.clone(),); self.getNewX(java_arguments.0, java_arguments.1) };
        let mut pointFactory: Option<Rc<RefCell<PointFactory>>> = PointFactory::getInstance();
        let mut mirroredPreviousPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(pointFactory).createXY(previousNewX.clone(), jcall!(jcall!(motionGestureEvent).getPreviousPoint()).getY());
        let mut mirroredCurrentPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(pointFactory).createXY(currentNewX.clone(), jcall!(jcall!(motionGestureEvent).getCurrentPoint()).getY());
        jcall!(event).setPreviousPoint(mirroredPreviousPoint.clone());
        jcall!(event).setCurrentPoint(mirroredCurrentPoint.clone());
        return event.clone();
    }
    

    pub fn getNewX(&mut self, mut x: i32, mut halfWidth: i32) -> i32 {
        let mut newX: i32 = 0;
        if x > halfWidth {
            newX = halfWidth - (x - halfWidth);
        } else {
            newX = halfWidth + (halfWidth - x);
        }
        return newX.clone();
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<MirrorMotionGestureEventUtil>>> {
        return MirrorMotionGestureEventUtil_instance.with(Clone::clone).clone();
    }
}


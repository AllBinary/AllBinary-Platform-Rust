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
// package org.allbinary.bounds
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::bounds::event::BoundsChangeEvent::BoundsChangeEvent;
use crate::org::allbinary::bounds::event::BoundsChangeEventListener::BoundsChangeEventListener;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use super::BoundsVisitorInterface::BoundsVisitorInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LayerBounds {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub rectangle: Option<Rc<RefCell<Rectangle>>>,
}

impl BoundsChangeEventListener for LayerBounds {
    fn onBoundsChangeEvent(&mut self, mut boundsChangeEvent: Option<Rc<RefCell<BoundsChangeEvent>>>) { Self::onBoundsChangeEvent(self, boundsChangeEvent) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for LayerBounds {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}

impl LayerBounds {
    

    pub fn new_struct(mut rectangle: Option<Rc<RefCell<Rectangle>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            rectangle: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.rectangle = rectangle.clone();
        this
    }

    pub fn new(mut rectangle: Option<Rc<RefCell<Rectangle>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(rectangle))))
    }
    

    pub fn getRectangleP(&mut self) -> Option<Rc<RefCell<Rectangle>>> {
        return self.rectangle.clone();
    }
    

    pub fn visit(&mut self, mut boundsVisitorInterface: Option<Rc<RefCell<dyn BoundsVisitorInterface>>>) {
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log(jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    pub fn onBoundsChangeEvent(&mut self, mut boundsChangeEvent: Option<Rc<RefCell<BoundsChangeEvent>>>) {
        self.rectangle = jcall!(boundsChangeEvent).getRectangle();
    }
}
pub trait LayerBoundsDyn {
    fn as_LayerBounds(&self) -> &LayerBounds;
    fn as_LayerBounds_mut(&mut self) -> &mut LayerBounds;
    fn getRectangleP(&mut self) -> Option<Rc<RefCell<Rectangle>>> { LayerBounds::getRectangleP(self.as_LayerBounds_mut()) }
    fn visit(&mut self, mut boundsVisitorInterface: Option<Rc<RefCell<dyn BoundsVisitorInterface>>>) { LayerBounds::visit(self.as_LayerBounds_mut(), boundsVisitorInterface) }
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { LayerBounds::onEvent(self.as_LayerBounds_mut(), eventObject) }
    fn onBoundsChangeEvent(&mut self, mut boundsChangeEvent: Option<Rc<RefCell<BoundsChangeEvent>>>) { LayerBounds::onBoundsChangeEvent(self.as_LayerBounds_mut(), boundsChangeEvent) }
}
impl LayerBoundsDyn for LayerBounds {
    fn as_LayerBounds(&self) -> &LayerBounds { self }
    fn as_LayerBounds_mut(&mut self) -> &mut LayerBounds { self }
}



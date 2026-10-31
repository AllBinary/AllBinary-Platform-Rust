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
// package org.allbinary.media.graphics.geography.map.event
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::CellPositionFactory::CellPositionFactory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPositionTracking::GeographicMapCellPositionTracking;
use super::GeographicMapCellPositionEvent::GeographicMapCellPositionEvent;
use super::GeographicMapCellPositionEventHandler::GeographicMapCellPositionEventHandler;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GeographicMapCellPositionEventUtil {
    pub geographicMapCellPositionEvent: Option<Rc<RefCell<GeographicMapCellPositionEvent>>>,
    pub geographicMapCellPositionEventHandler: Option<Rc<RefCell<GeographicMapCellPositionEventHandler>>>,
}


impl GeographicMapCellPositionEventUtil {
    

    pub fn new_struct(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Self {
        let mut this = Self {
            geographicMapCellPositionEvent: None,
            geographicMapCellPositionEventHandler: None,
        };
        this.geographicMapCellPositionEventHandler = GeographicMapCellPositionEventHandler::getInstance();
        this.geographicMapCellPositionEvent = GeographicMapCellPositionEvent::new(anyType.clone(), jcall!(CellPositionFactory::getInstance()).NONE.clone());
        this
    }

    pub fn new(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(anyType))))
    }
    

    // throws Exception
    pub fn update(&mut self, mut geographicMapCellPositionTracking: Option<Rc<RefCell<GeographicMapCellPositionTracking>>>) {
        { let java_receiver = self.geographicMapCellPositionEvent.clone(); let java_arguments = (jcall!(geographicMapCellPositionTracking).getCurrentGeographicMapCellPosition(),); jcall!(java_receiver).setGeographicMapCellPosition(java_arguments.0); };
        jcall!(self.geographicMapCellPositionEventHandler).fireEvent(self.geographicMapCellPositionEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
    }
    

    // throws Exception
    pub fn remove(&mut self, mut geographicMapCellPositionTracking: Option<Rc<RefCell<GeographicMapCellPositionTracking>>>) {
        { let java_receiver = self.geographicMapCellPositionEvent.clone(); let java_arguments = (jcall!(geographicMapCellPositionTracking).getCurrentGeographicMapCellPosition(),); jcall!(java_receiver).setGeographicMapCellPosition(java_arguments.0); };
        jcall!(self.geographicMapCellPositionEventHandler).fireRemoveEvent(self.geographicMapCellPositionEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
    }
}


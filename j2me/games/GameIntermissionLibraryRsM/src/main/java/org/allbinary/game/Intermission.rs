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
// package org.allbinary.game
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use super::IntermissionInterface::IntermissionInterface;
use super::IntermissionEnableListenerInterface::IntermissionEnableListenerInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Intermission {
    pub enabled: bool,
    pub timeElapsedHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub intermissionEnableListener: Option<Rc<RefCell<dyn IntermissionEnableListenerInterface>>>,
}

impl IntermissionInterface for Intermission {
    fn getTimeDelayHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>> { Self::getTimeDelayHelper(self) }
    fn isEnabled(&mut self) -> bool { Self::isEnabled(self) }
    fn setEnabled(&mut self, mut enabled: bool) { Self::setEnabled(self, enabled) }
    fn setListener(&mut self, mut enableListener: Option<Rc<RefCell<dyn IntermissionEnableListenerInterface>>>) { Self::setListener(self, enableListener) }
}

impl Intermission {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            enabled: false,
            timeElapsedHelper: None,
            intermissionEnableListener: None,
        };
        this.timeElapsedHelper = TimeDelayHelper::new(0).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.intermissionEnableListener = NullIntermissionEnableListener::NullIntermissionEnableListener::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setListener(&mut self, mut enableListener: Option<Rc<RefCell<dyn IntermissionEnableListenerInterface>>>) {
        self.intermissionEnableListener = enableListener.clone();
    }
    

    pub fn getTimeDelayHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>> {
        return self.timeElapsedHelper.clone();
    }
    

    pub fn setEnabled(&mut self, mut enabled: bool) {
        self.enabled = enabled.clone();
        if self.isEnabled() {
            jcall!(self.getTimeDelayHelper()).setStartTimeTNT();
        }
        jcall!(self.intermissionEnableListener).notifyIntermission(enabled.clone());
    }
    

    pub fn isEnabled(&mut self) -> bool {
        return self.enabled.clone();
    }
}


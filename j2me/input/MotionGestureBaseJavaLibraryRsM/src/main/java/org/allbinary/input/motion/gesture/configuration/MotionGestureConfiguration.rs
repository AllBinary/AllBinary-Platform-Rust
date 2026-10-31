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
// package org.allbinary.input.motion.gesture.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEvent;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::input::motion::gesture::MotionGestureToMotionGestureActionAssociation::MotionGestureToMotionGestureActionAssociation;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MotionGestureConfiguration {
    pub diagonalMotionGestureAllowed: bool,
    pub diagonalTolerance: i32,
    pub minimumMotionGesture: i32,
    pub button: String,
    pub color: Option<Rc<RefCell<BasicColor>>>,
    pub executingActions: bool,
    pub mouseButtonMask: i32,
    pub activeCommands: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl MotionGestureConfiguration {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            diagonalMotionGestureAllowed: false,
            diagonalTolerance: 0,
            minimumMotionGesture: 0,
            button: String::new(),
            color: None,
            executingActions: false,
            mouseButtonMask: 0,
            activeCommands: None,
        };
        this.diagonalMotionGestureAllowed = true;
        this.diagonalTolerance = 7;
        this.minimumMotionGesture = 9;
        this.button = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.color = jcall!(BasicColorFactory::getInstance()).BLUE.clone();
        this.executingActions = true;
        this.mouseButtonMask = 0;
        this.activeCommands = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn addMotionGestureAction(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut commandAction: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEvent::CompleteMotionGestureInputEventDyn>>>) {
        jcall!(self.activeCommands).add(crate::java_into_object(MotionGestureToMotionGestureActionAssociation::new(list.clone(), commandAction.clone())));
    }
    

    pub fn clearMotionGestureAction(&mut self) {
        jcall!(self.activeCommands).clear();
    }
    

    pub fn getAssociateCommandActionsList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.activeCommands.clone();
    }
    

    pub fn isDiagonalMotionGestureAllowed(&mut self) -> bool {
        return self.diagonalMotionGestureAllowed.clone();
    }
    

    pub fn isExecutingActionsAllowed(&mut self) -> bool {
        return self.executingActions.clone();
    }
    

    pub fn setExecutingActionsAllowed(&mut self, mut executingActionAllowed: bool) {
        self.executingActions = executingActionAllowed.clone();
    }
    

    pub fn setDiagonalMotionGestureAllowed(&mut self, mut diagonalMotionGestureAllowed: bool) {
        self.diagonalMotionGestureAllowed = diagonalMotionGestureAllowed.clone();
    }
    

    pub fn getDiagonalTolerance(&mut self) -> i32 {
        return self.diagonalTolerance.clone();
    }
    

    pub fn setDiagonalTolerance(&mut self, mut diagonalTolerance: i32) {
        self.diagonalTolerance = diagonalTolerance.clone();
    }
    

    pub fn getMinimumMotionGesture(&mut self) -> i32 {
        return self.minimumMotionGesture.clone();
    }
    

    pub fn setMinimumMotionGesture(&mut self, mut minimumMotionGesture: i32) {
        self.minimumMotionGesture = minimumMotionGesture.clone();
    }
    

    pub fn getButton(&mut self) -> String {
        return self.button.clone();
    }
    

    pub fn setButton(&mut self, mut button: String) {
        self.button = button.clone();
    }
    

    pub fn getColor(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        return self.color.clone();
    }
    

    pub fn setColor(&mut self, mut color: Option<Rc<RefCell<BasicColor>>>) {
        self.color = color.clone();
    }
    

    pub fn getMouseButtonMask(&mut self) -> i32 {
        return self.mouseButtonMask.clone();
    }
    

    pub fn setMouseButtonMask(&mut self, mut mouseButtonMask: i32) {
        self.mouseButtonMask = mouseButtonMask.clone();
    }
}


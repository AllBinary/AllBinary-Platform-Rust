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
// package org.allbinary.game.displayable.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::form::item::CommandTextItemArrayFactory::CommandTextItemArrayFactory;
use crate::org::allbinary::graphics::form::item::LimitCommandsVisitor::LimitCommandsVisitor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DemoLimitedCommandTextItemArrayFactory {
    pub commandTextItemArrayFactory: Option<Rc<RefCell<CommandTextItemArrayFactory>>>,
}

thread_local! { pub static DemoLimitedCommandTextItemArrayFactory_instance: Option<Rc<RefCell<DemoLimitedCommandTextItemArrayFactory>>> = DemoLimitedCommandTextItemArrayFactory::new(); }

impl DemoLimitedCommandTextItemArrayFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            commandTextItemArrayFactory: None,
        };
        this.commandTextItemArrayFactory = CommandTextItemArrayFactory::new(LimitCommandsVisitor::new());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<DemoLimitedCommandTextItemArrayFactory>>> {
        return DemoLimitedCommandTextItemArrayFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn getCommandTextItemArrayFactory(&mut self) -> Option<Rc<RefCell<CommandTextItemArrayFactory>>> {
        return self.commandTextItemArrayFactory.clone();
    }
}


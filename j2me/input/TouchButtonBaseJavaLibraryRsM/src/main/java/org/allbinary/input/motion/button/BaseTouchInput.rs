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
// package org.allbinary.input.motion.button
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BaseTouchInput {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub basicArrayListUtil: Option<Rc<RefCell<BasicArrayListUtil>>>,
}


impl BaseTouchInput {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            commonStrings: None,
            basicArrayListUtil: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this.basicArrayListUtil = BasicArrayListUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return jcall!(self.basicArrayListUtil).getImmutableInstance();
    }
    

    pub fn toString(&mut self) -> String {
        return std::any::type_name_of_val(&self).to_string();
    }
}
pub trait BaseTouchInputDyn {
    fn as_BaseTouchInput(&self) -> &BaseTouchInput;
    fn as_BaseTouchInput_mut(&mut self) -> &mut BaseTouchInput;
    fn getList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { BaseTouchInput::getList(self.as_BaseTouchInput_mut()) }
    fn toString(&mut self) -> String { BaseTouchInput::toString(self.as_BaseTouchInput_mut()) }
}
impl BaseTouchInputDyn for BaseTouchInput {
    fn as_BaseTouchInput(&self) -> &BaseTouchInput { self }
    fn as_BaseTouchInput_mut(&mut self) -> &mut BaseTouchInput { self }
}



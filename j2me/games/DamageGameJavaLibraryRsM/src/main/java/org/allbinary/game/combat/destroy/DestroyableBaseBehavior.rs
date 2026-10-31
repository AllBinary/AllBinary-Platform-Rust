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
// package org.allbinary.game.combat.destroy
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::DestroyableInterface::DestroyableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DestroyableBaseBehavior {
}

thread_local! { pub static DestroyableBaseBehavior_instance: Option<Rc<RefCell<dyn DestroyableBaseBehaviorDyn>>> = DestroyableBaseBehavior::new().map(|reference| reference as Rc<RefCell<dyn DestroyableBaseBehaviorDyn>>); }
impl DestroyableInterface for DestroyableBaseBehavior {
    fn isDestroyed(&mut self) -> bool { Self::isDestroyed(self) }
}
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for DestroyableBaseBehavior {
    fn getName(&mut self) -> String { Self::getName(self) }
}

impl DestroyableBaseBehavior {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn DestroyableBaseBehaviorDyn>>> {
        return DestroyableBaseBehavior_instance.with(Clone::clone).clone();
    }
    

    pub fn getName(&mut self) -> String {
        return std::any::type_name_of_val(&self).to_string();
    }
    

    // throws Exception
    pub fn isDestroyed(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(CommonStrings::getInstance()).NOT_IMPLEMENTED.clone())));
    }
    

    pub fn setDestroyed(&mut self, mut destroyed: bool) {
    }
}
pub trait DestroyableBaseBehaviorDyn {
    fn as_DestroyableBaseBehavior(&self) -> &DestroyableBaseBehavior;
    fn as_DestroyableBaseBehavior_mut(&mut self) -> &mut DestroyableBaseBehavior;
    fn getName(&mut self) -> String { DestroyableBaseBehavior::getName(self.as_DestroyableBaseBehavior_mut()) }
    fn isDestroyed(&mut self) -> bool { DestroyableBaseBehavior::isDestroyed(self.as_DestroyableBaseBehavior_mut()) }
    fn setDestroyed(&mut self, mut destroyed: bool) { DestroyableBaseBehavior::setDestroyed(self.as_DestroyableBaseBehavior_mut(), destroyed) }
}
impl DestroyableBaseBehaviorDyn for DestroyableBaseBehavior {
    fn as_DestroyableBaseBehavior(&self) -> &DestroyableBaseBehavior { self }
    fn as_DestroyableBaseBehavior_mut(&mut self) -> &mut DestroyableBaseBehavior { self }
}



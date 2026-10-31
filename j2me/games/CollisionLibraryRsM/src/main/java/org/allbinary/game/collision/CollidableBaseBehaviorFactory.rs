// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package org.allbinary.game.collision
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::CollidableBaseBehaviorFactoryInterface::CollidableBaseBehaviorFactoryInterface;
use super::CollidableBaseBehavior::CollidableBaseBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CollidableBaseBehaviorFactory {
}

impl CollidableBaseBehaviorFactoryInterface for CollidableBaseBehaviorFactory {
    fn createBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>> { Self::createBehavior(self) }
}

impl CollidableBaseBehaviorFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn createBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
pub trait CollidableBaseBehaviorFactoryDyn: crate::org::allbinary::game::collision::CollidableBaseBehaviorFactoryInterface::CollidableBaseBehaviorFactoryInterface {
    fn as_CollidableBaseBehaviorFactory(&self) -> &CollidableBaseBehaviorFactory;
    fn as_CollidableBaseBehaviorFactory_mut(&mut self) -> &mut CollidableBaseBehaviorFactory;
}
impl CollidableBaseBehaviorFactoryDyn for CollidableBaseBehaviorFactory {
    fn as_CollidableBaseBehaviorFactory(&self) -> &CollidableBaseBehaviorFactory { self }
    fn as_CollidableBaseBehaviorFactory_mut(&mut self) -> &mut CollidableBaseBehaviorFactory { self }
}



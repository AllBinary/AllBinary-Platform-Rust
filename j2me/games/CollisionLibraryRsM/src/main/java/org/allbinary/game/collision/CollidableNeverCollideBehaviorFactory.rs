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
// package org.allbinary.game.collision
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::CollidableBaseBehaviorFactory::CollidableBaseBehaviorFactory;
use super::CollidableNeverCollideBehavior::CollidableNeverCollideBehavior;
use super::CollidableBaseBehavior::CollidableBaseBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CollidableNeverCollideBehaviorFactory {
    pub base: CollidableBaseBehaviorFactory,
    pub instance: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableNeverCollideBehavior::CollidableNeverCollideBehaviorDyn>>>,
}

thread_local! { pub static CollidableNeverCollideBehaviorFactory_instanceC: Option<Rc<RefCell<CollidableNeverCollideBehaviorFactory>>> = CollidableNeverCollideBehaviorFactory::new(); }

impl CollidableNeverCollideBehaviorFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: CollidableBaseBehaviorFactory::new_struct(),
            instance: None,
        };
        this.instance = CollidableNeverCollideBehavior::new(false).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableNeverCollideBehavior::CollidableNeverCollideBehaviorDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<CollidableNeverCollideBehaviorFactory>>> {
        return CollidableNeverCollideBehaviorFactory_instanceC.with(Clone::clone).clone();
    }
    

    pub fn createBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>> {
        return self.instance.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>);
    }
}
impl std::ops::Deref for CollidableNeverCollideBehaviorFactory {
    type Target = CollidableBaseBehaviorFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CollidableNeverCollideBehaviorFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::collision::CollidableBaseBehaviorFactory::CollidableBaseBehaviorFactoryDyn for CollidableNeverCollideBehaviorFactory {
    fn as_CollidableBaseBehaviorFactory(&self) -> &crate::org::allbinary::game::collision::CollidableBaseBehaviorFactory::CollidableBaseBehaviorFactory { &self.base }
    fn as_CollidableBaseBehaviorFactory_mut(&mut self) -> &mut crate::org::allbinary::game::collision::CollidableBaseBehaviorFactory::CollidableBaseBehaviorFactory { &mut self.base }
}
impl crate::org::allbinary::game::collision::CollidableBaseBehaviorFactoryInterface::CollidableBaseBehaviorFactoryInterface for CollidableNeverCollideBehaviorFactory {
    fn createBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>> { Self::createBehavior(self) }
}



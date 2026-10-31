// Generated Code Do Not Modify
// package org.allbinary.game.collision
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CollisionProcessor {
}


impl CollisionProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isCollision(&mut self, mut myLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut myLayer2: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> bool {
        return false;
    }
}
pub trait CollisionProcessorDyn {
    fn as_CollisionProcessor(&self) -> &CollisionProcessor;
    fn as_CollisionProcessor_mut(&mut self) -> &mut CollisionProcessor;
    fn isCollision(&mut self, mut myLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut myLayer2: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> bool { CollisionProcessor::isCollision(self.as_CollisionProcessor_mut(), myLayer, myLayer2) }
}
impl CollisionProcessorDyn for CollisionProcessor {
    fn as_CollisionProcessor(&self) -> &CollisionProcessor { self }
    fn as_CollisionProcessor_mut(&mut self) -> &mut CollisionProcessor { self }
}



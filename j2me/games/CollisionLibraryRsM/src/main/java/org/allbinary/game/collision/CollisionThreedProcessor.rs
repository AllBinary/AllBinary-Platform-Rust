// Generated Code Do Not Modify
// package org.allbinary.game.collision
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use super::CollisionProcessor::CollisionProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CollisionThreedProcessor {
    pub base: CollisionProcessor,
}


impl CollisionThreedProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: CollisionProcessor::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isCollision(&mut self, mut myLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut myLayer2: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> bool {
        if jcall!(myLayer2).getXP() >= jcall!(myLayer).getX2() || jcall!(myLayer2).getYP() >= jcall!(myLayer).getY2() || jcall!(myLayer2).getX2() <= jcall!(myLayer).getXP() || jcall!(myLayer2).getY2() <= jcall!(myLayer).getYP() || jcall!(myLayer2).getZP() < jcall!(myLayer).getZ2() || jcall!(myLayer2).getZ2() > jcall!(myLayer).getZP() {
            return false;
        } else {
            return true;
        }
    }
}
impl std::ops::Deref for CollisionThreedProcessor {
    type Target = CollisionProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CollisionThreedProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::collision::CollisionProcessor::CollisionProcessorDyn for CollisionThreedProcessor {
    fn as_CollisionProcessor(&self) -> &crate::org::allbinary::game::collision::CollisionProcessor::CollisionProcessor { &self.base }
    fn as_CollisionProcessor_mut(&mut self) -> &mut crate::org::allbinary::game::collision::CollisionProcessor::CollisionProcessor { &mut self.base }
    fn isCollision(&mut self, mut myLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut myLayer2: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> bool { Self::isCollision(self, myLayer, myLayer2) }
}



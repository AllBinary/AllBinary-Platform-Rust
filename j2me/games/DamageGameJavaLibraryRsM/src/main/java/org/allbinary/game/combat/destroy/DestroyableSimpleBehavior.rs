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
use crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer;
use super::DestroyableBaseBehavior::DestroyableBaseBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DestroyableSimpleBehavior {
    pub base: DestroyableBaseBehavior,
    pub ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>,
    pub destroyed: bool,
}

impl crate::org::allbinary::game::combat::destroy::DestroyableInterface::DestroyableInterface for DestroyableSimpleBehavior {
    fn isDestroyed(&mut self) -> bool { Self::isDestroyed(self) }
}
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for DestroyableSimpleBehavior {
    fn getName(&mut self) -> String { self.base.getName() }
}

impl DestroyableSimpleBehavior {
    

    pub fn new_struct(mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> Self {
        let mut this = Self {
            base: DestroyableBaseBehavior::new_struct(),
            ownerLayer: None,
            destroyed: false,
        };
        this.destroyed = false;
        this.ownerLayer = ownerLayer.clone();
        this
    }

    pub fn new(mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(ownerLayer))))
    }
    

    pub fn isDestroyed(&mut self) -> bool {
        return self.destroyed.clone();
    }
    

    pub fn setDestroyed(&mut self, mut destroyed: bool) {
        self.destroyed = destroyed.clone();
        if self.isDestroyed() {
            jcall!(DestroyedLayerProcessor::DestroyedLayerProcessor::getInstance()).add(self.ownerLayer.clone());
        }
    }
}
impl std::ops::Deref for DestroyableSimpleBehavior {
    type Target = DestroyableBaseBehavior;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DestroyableSimpleBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::combat::destroy::DestroyableBaseBehavior::DestroyableBaseBehaviorDyn for DestroyableSimpleBehavior {
    fn as_DestroyableBaseBehavior(&self) -> &crate::org::allbinary::game::combat::destroy::DestroyableBaseBehavior::DestroyableBaseBehavior { &self.base }
    fn as_DestroyableBaseBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::combat::destroy::DestroyableBaseBehavior::DestroyableBaseBehavior { &mut self.base }
    fn isDestroyed(&mut self) -> bool { Self::isDestroyed(self) }
    fn setDestroyed(&mut self, mut destroyed: bool) { Self::setDestroyed(self, destroyed) }
}



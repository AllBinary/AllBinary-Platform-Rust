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
use crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer;
use super::CollidableBaseBehavior::CollidableBaseBehavior;
use super::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CollidableNeverCollideBehavior {
    pub base: CollidableBaseBehavior,
}

impl crate::org::allbinary::layer::NamedInterface::NamedInterface for CollidableNeverCollideBehavior {
    fn getName(&mut self) -> String { self.base.getName() }
}

impl CollidableNeverCollideBehavior {
    

    pub fn new_struct(mut isCollidable: bool) -> Self {
        let mut this = Self {
            base: CollidableBaseBehavior::new_struct(isCollidable.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut isCollidable: bool) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(isCollidable))))
    }
    

    pub fn isCollision(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut allbinaryCollidableLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool {
        return false;
    }
    

    // throws Exception
    pub fn collide(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut allbinaryCollidableLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) {
    }
    

    pub fn collideInterface(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) {
    }
    

    pub fn isCollisionInterface(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) -> bool {
        return false;
    }
}
impl std::ops::Deref for CollidableNeverCollideBehavior {
    type Target = CollidableBaseBehavior;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CollidableNeverCollideBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait CollidableNeverCollideBehaviorDyn: crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn {
    fn as_CollidableNeverCollideBehavior(&self) -> &CollidableNeverCollideBehavior;
    fn as_CollidableNeverCollideBehavior_mut(&mut self) -> &mut CollidableNeverCollideBehavior;
}
impl CollidableNeverCollideBehaviorDyn for CollidableNeverCollideBehavior {
    fn as_CollidableNeverCollideBehavior(&self) -> &CollidableNeverCollideBehavior { self }
    fn as_CollidableNeverCollideBehavior_mut(&mut self) -> &mut CollidableNeverCollideBehavior { self }
}

impl crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn for CollidableNeverCollideBehavior {
    fn as_CollidableBaseBehavior(&self) -> &crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehavior { &self.base }
    fn as_CollidableBaseBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehavior { &mut self.base }
    fn collide(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut allbinaryCollidableLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) { Self::collide(self, ownerLayer, allbinaryCollidableLayer) }
    fn isCollision(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut allbinaryCollidableLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool { Self::isCollision(self, ownerLayer, allbinaryCollidableLayer) }
    fn isCollisionInterface(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) -> bool { Self::isCollisionInterface(self, ownerLayer, collidableInterfaceCompositeInterface) }
    fn collideInterface(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) { Self::collideInterface(self, ownerLayer, collidableInterfaceCompositeInterface) }
}



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
// package org.allbinary.game.collision.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehavior;
use crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface;
use crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer;
use crate::org::allbinary::layer::LayerManager::LayerManager;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryCollisionManager {
}

pub static mut AllBinaryCollisionManager_allBinaryCollisionManager: std::cell::LazyCell<Option<Rc<RefCell<AllBinaryCollisionManager>>>> = std::cell::LazyCell::new(|| AllBinaryCollisionManager::new());

impl AllBinaryCollisionManager {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AllBinaryCollisionManager>>> {
        return unsafe { AllBinaryCollisionManager_allBinaryCollisionManager.clone() }.clone();
    }
    

    // throws Exception
    pub fn process(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerManager::LayerManagerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>, mut startIndex: i32) {
        let mut size: i32 = jcall!(layerManager).getSize();
        for index in (startIndex + 1..size) {
            let mut collidableInterfaceCompositeInterface2: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>> = (|| -> Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>> { panic!("ClassCastException: a Rust dyn Layer cannot be cast to CollidableInterfaceCompositeInterface") })();
            self.collide(layerManager.clone(), collidableInterfaceCompositeInterface.clone(), collidableInterfaceCompositeInterface2.clone());
        }
    }
    

    // throws Exception
    pub fn collide(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerManager::LayerManagerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>, mut collidableInterfaceCompositeInterface2: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) {
        let mut collidableBase2: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>> = jcall!(collidableInterfaceCompositeInterface2).getCollidableInferface();
        let mut collidableBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>> = jcall!(collidableInterfaceCompositeInterface).getCollidableInferface();
        if jcall!(collidableBase2).isCollidable((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>> { panic!("ClassCastException: a Rust dyn CollidableInterfaceCompositeInterface cannot be cast to CollidableCompositeLayer") })()) {
            if jcall!(collidableBase).isCollisionInterface((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>> { panic!("ClassCastException: a Rust dyn CollidableInterfaceCompositeInterface cannot be cast to CollidableCompositeLayer") })(), collidableInterfaceCompositeInterface2.clone()) {
                jcall!(collidableBase).collideInterface((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>> { panic!("ClassCastException: a Rust dyn CollidableInterfaceCompositeInterface cannot be cast to CollidableCompositeLayer") })(), collidableInterfaceCompositeInterface2.clone());
                jcall!(collidableBase2).collideInterface((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>> { panic!("ClassCastException: a Rust dyn CollidableInterfaceCompositeInterface cannot be cast to CollidableCompositeLayer") })(), collidableInterfaceCompositeInterface.clone());
            }
        }
    }
}


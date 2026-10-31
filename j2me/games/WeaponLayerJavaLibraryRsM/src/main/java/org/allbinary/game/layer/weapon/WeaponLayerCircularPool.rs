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
// package org.allbinary.game.layer.weapon
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::combat::weapon::WeaponProperties::WeaponProperties;
use crate::org::allbinary::game::score::ScoreableInterface::ScoreableInterface;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerCircularPool::AllBinaryLayerCircularPool;
use super::WeaponLayer::WeaponLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WeaponLayerCircularPool {
    pub base: AllBinaryLayerCircularPool,
}


impl WeaponLayerCircularPool {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AllBinaryLayerCircularPool::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstanceWeaponLayer(&mut self, mut sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut x: i32, mut y: i32, mut z: i32, mut angle: i32, mut otherAngle: i32, mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>>, mut scoreable: Option<Rc<RefCell<dyn ScoreableInterface>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn>>> {
        let mut weaponLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn>>> = (self.base.getNextInstance()).clone() /* as WeaponLayer */;
        jcall!(weaponLayer).initXYZ(x.clone(), y.clone(), z.clone());
        jcall!(weaponLayer).init(sourceLayerInterface.clone(), angle.clone(), otherAngle.clone(), weaponProperties.clone(), scoreable.clone());
        return weaponLayer.clone();
    }
}
impl std::ops::Deref for WeaponLayerCircularPool {
    type Target = AllBinaryLayerCircularPool;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for WeaponLayerCircularPool {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::layer::AllBinaryLayerCircularPool::AllBinaryLayerCircularPoolDyn for WeaponLayerCircularPool {
    fn as_AllBinaryLayerCircularPool(&self) -> &crate::org::allbinary::layer::AllBinaryLayerCircularPool::AllBinaryLayerCircularPool { &self.base }
    fn as_AllBinaryLayerCircularPool_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayerCircularPool::AllBinaryLayerCircularPool { &mut self.base }
}
impl crate::org::allbinary::logic::util::cache::BaseCircularPool::BaseCircularPoolDyn for WeaponLayerCircularPool {
    fn as_BaseCircularPool(&self) -> &crate::org::allbinary::logic::util::cache::BaseCircularPool::BaseCircularPool { &self.base.base }
    fn as_BaseCircularPool_mut(&mut self) -> &mut crate::org::allbinary::logic::util::cache::BaseCircularPool::BaseCircularPool { &mut self.base.base }
}



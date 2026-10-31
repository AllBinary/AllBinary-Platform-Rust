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
// package org.allbinary.game.ag.layer.weapon.mine
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::weapon::WeaponLayerCircularPool::WeaponLayerCircularPool;
use crate::org::allbinary::view::ViewPositionFactoryInterface::ViewPositionFactoryInterface;
use super::MineLayerFactory::MineLayerFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MineLayerCircularStaticPool {
}

pub static mut MineLayerCircularStaticPool_WEAPON_LAYER_CIRCULAR_STATIC_POOL: std::cell::LazyCell<Option<Rc<RefCell<WeaponLayerCircularPool>>>> = std::cell::LazyCell::new(|| WeaponLayerCircularPool::new());

impl MineLayerCircularStaticPool {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn init(mut viewPositionFactoryInterface: Option<Rc<RefCell<dyn ViewPositionFactoryInterface>>>) {
        MineCountedPickedUpLayerInterfaceFactory::MineCountedPickedUpLayerInterfaceFactory::init();
        jcall!(unsafe { MineLayerCircularStaticPool_WEAPON_LAYER_CIRCULAR_STATIC_POOL.clone() }).initAllBinaryLayerCircularPool(MineLayerFactory::new(viewPositionFactoryInterface.clone()), 20);
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<WeaponLayerCircularPool>>> {
        return unsafe { MineLayerCircularStaticPool_WEAPON_LAYER_CIRCULAR_STATIC_POOL.clone() }.clone();
    }
}


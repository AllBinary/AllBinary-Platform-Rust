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
// package org.allbinary.game.part.weapon
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::combat::weapon::WeaponProperties::WeaponProperties;
use crate::org::allbinary::game::score::ScoreableInterface::ScoreableInterface;
use crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationship;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use super::BasicWeaponPart::BasicWeaponPart;
use super::BasicWeaponPartFactoryInterface::BasicWeaponPartFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicWeaponPartCircularPool {
    pub basicWeaponPart: Option<Rc<RefCell<dyn crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPartDyn>>>,
}

thread_local! { pub static BasicWeaponPartCircularPool_NULL_BASIC_WEAPON_PART_CIRCULAR_POOL: Option<Rc<RefCell<BasicWeaponPartCircularPool>>> = BasicWeaponPartCircularPool::new(); }

impl BasicWeaponPartCircularPool {

    pub fn new_struct() -> Self {
        let mut this = Self {
            basicWeaponPart: None,
        };
        this.basicWeaponPart = BasicWeaponPart_NULL_BASIC_WEAPON_PART.with(Clone::clone).clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn init(&mut self, mut basicWeaponPartFactoryInterface: Option<Rc<RefCell<dyn BasicWeaponPartFactoryInterface>>>) {
        self.basicWeaponPart = jcall!(basicWeaponPartFactoryInterface).getInstance();
    }
    

    // throws Exception
    // synchronized
    pub fn getInstance(&mut self, mut sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>>, mut scoreableInterface: Option<Rc<RefCell<dyn ScoreableInterface>>>, mut relativeRelationship: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationshipDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPartDyn>>> {
        jcall!(self.basicWeaponPart).init(sourceLayerInterface.clone(), weaponProperties.clone(), scoreableInterface.clone(), relativeRelationship.clone());
        return self.basicWeaponPart.clone();
    }
}


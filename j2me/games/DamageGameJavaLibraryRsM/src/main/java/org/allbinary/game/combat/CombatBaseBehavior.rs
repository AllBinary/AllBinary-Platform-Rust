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
// package org.allbinary.game.combat
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::combat::damage::DamageableBaseBehavior::DamageableBaseBehavior;
use crate::org::allbinary::game::combat::destroy::DestroyableBaseBehavior::DestroyableBaseBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CombatBaseBehavior {
    pub damageableBaseBehavior: Option<Rc<RefCell<DamageableBaseBehavior>>>,
    pub destroyableBaseBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::game::combat::destroy::DestroyableBaseBehavior::DestroyableBaseBehaviorDyn>>>,
    pub readyForExplosion: bool,
}

thread_local! { pub static CombatBaseBehavior_NULL_COMBAT_BEHAVIOR: Option<Rc<RefCell<CombatBaseBehavior>>> = CombatBaseBehavior::new(DamageableBaseBehavior::getInstance(), DestroyableBaseBehavior::getInstance()); }

impl CombatBaseBehavior {
    

    pub fn new_struct(mut damageableBaseBehavior: Option<Rc<RefCell<DamageableBaseBehavior>>>, mut destroyableBaseBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::game::combat::destroy::DestroyableBaseBehavior::DestroyableBaseBehaviorDyn>>>) -> Self {
        let mut this = Self {
            damageableBaseBehavior: None,
            destroyableBaseBehavior: None,
            readyForExplosion: false,
        };
        this.damageableBaseBehavior = damageableBaseBehavior.clone();
        this.destroyableBaseBehavior = destroyableBaseBehavior.clone();
        this
    }

    pub fn new(mut damageableBaseBehavior: Option<Rc<RefCell<DamageableBaseBehavior>>>, mut destroyableBaseBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::game::combat::destroy::DestroyableBaseBehavior::DestroyableBaseBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(damageableBaseBehavior, destroyableBaseBehavior))))
    }
    

    pub fn getDamageableBaseBehavior(&mut self) -> Option<Rc<RefCell<DamageableBaseBehavior>>> {
        return self.damageableBaseBehavior.clone();
    }
    

    pub fn getDestroyableBaseBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::combat::destroy::DestroyableBaseBehavior::DestroyableBaseBehaviorDyn>>> {
        return self.destroyableBaseBehavior.clone();
    }
    

    pub fn isReadyForExplosion(&mut self) -> bool {
        return self.readyForExplosion.clone();
    }
    

    pub fn setReadyForExplosion(&mut self, mut readyForExplosion: bool) {
        self.readyForExplosion = readyForExplosion.clone();
    }
}


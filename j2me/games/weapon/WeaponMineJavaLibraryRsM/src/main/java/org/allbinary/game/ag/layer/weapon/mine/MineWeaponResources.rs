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
use crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources;
use crate::org::allbinary::logic::NullUtil::NullUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MineWeaponResources {
    pub base: BasicGameResources,
    pub DROP_TEXT_RESOURCE: String,
    pub DROP_RESOURCE: String,
    pub DROP_ICON_RESOURCE: String,
}

pub static mut MineWeaponResources_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl MineWeaponResources {
    

    pub fn getInstance() -> Option<Rc<RefCell<MineWeaponResources>>> {
        if match (&unsafe { MineWeaponResources_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *MineWeaponResources_instance = MineWeaponResources::new() };
        }
        return (unsafe { MineWeaponResources_instance.clone() }).clone() /* as MineWeaponResources */;
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BasicGameResources::new_struct(),
            DROP_TEXT_RESOURCE: String::new(),
            DROP_RESOURCE: String::new(),
            DROP_ICON_RESOURCE: String::new(),
        };
        this.DROP_TEXT_RESOURCE = String::from("/mine_drop_text_20_by_20.png");
        this.DROP_RESOURCE = String::from("/mine_drop_20_by_20.png");
        this.DROP_ICON_RESOURCE = String::from("/mine_drop_icon_20_by_20.png");
        let mut ROOT: String = String::from("/mine");
        let mut SMALL: String = String::from("_20_by_20.png");
        let mut MEDIUM: String = SMALL.clone();
        let mut SIZE_FOUR: String = SMALL.clone();
        let mut SIZE_FIVE: String = SMALL.clone();
        let mut SIZE_SIX: String = SMALL.clone();
        let mut SIZE: Vec<String> = vec![SMALL.clone(), MEDIUM.clone(), SIZE_FOUR.clone(), SIZE_FIVE.clone(), SIZE_SIX.clone()];
        this.base.init(ROOT.clone(), SIZE.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl std::ops::Deref for MineWeaponResources {
    type Target = BasicGameResources;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MineWeaponResources {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResourcesDyn for MineWeaponResources {
    fn as_BasicGameResources(&self) -> &crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources { &self.base }
    fn as_BasicGameResources_mut(&mut self) -> &mut crate::org::allbinary::game::layer::resources::BasicGameResources::BasicGameResources { &mut self.base }
}



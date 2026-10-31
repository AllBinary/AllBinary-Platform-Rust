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
// package org.allbinary.input.automation.module.game.skill
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameSkillType {
    pub SPELL_GAME_SKILL_TYPE: String,
    pub name: String,
}

pub static mut GameSkillType_MANA: std::cell::LazyCell<Option<Rc<RefCell<GameSkillType>>>> = std::cell::LazyCell::new(|| GameSkillType::new(String::from("Mana")));
pub static mut GameSkillType_REPLENISH: std::cell::LazyCell<Option<Rc<RefCell<GameSkillType>>>> = std::cell::LazyCell::new(|| GameSkillType::new(String::from("Replenish")));
pub static mut GameSkillType_TIMED: std::cell::LazyCell<Option<Rc<RefCell<GameSkillType>>>> = std::cell::LazyCell::new(|| GameSkillType::new(String::from("Timed")));
pub static mut GameSkillType_ATTACK: std::cell::LazyCell<Option<Rc<RefCell<GameSkillType>>>> = std::cell::LazyCell::new(|| GameSkillType::new(String::from("Attack")));
pub static mut GameSkillType_TRANSPORT: std::cell::LazyCell<Option<Rc<RefCell<GameSkillType>>>> = std::cell::LazyCell::new(|| GameSkillType::new(String::from("Transport")));

impl GameSkillType {
    

    pub fn new_struct(mut name: String) -> Self {
        let mut this = Self {
            SPELL_GAME_SKILL_TYPE: String::new(),
            name: String::new(),
        };
        this.SPELL_GAME_SKILL_TYPE = String::from("SpellGameSkillType: ");
        this.setName(name.clone());
        this
    }

    pub fn new(mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name))))
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn setName(&mut self, mut name: String) {
        self.name = name.clone();
    }
    

    pub fn toString(&mut self) -> String {
        return format!("{}{}", self.SPELL_GAME_SKILL_TYPE, self.getName());
    }
}


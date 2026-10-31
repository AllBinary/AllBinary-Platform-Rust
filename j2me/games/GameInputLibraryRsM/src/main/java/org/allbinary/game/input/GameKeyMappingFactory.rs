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
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use super::GameKey::GameKey;
use super::GameKeyFactory::GameKeyFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameKeyMappingFactory {
    pub intKeyToGameKey: Vec<Option<Rc<RefCell<GameKey>>>>,
}

pub static mut GameKeyMappingFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl GameKeyMappingFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<GameKeyMappingFactory>>> {
        if match (&unsafe { GameKeyMappingFactory_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *GameKeyMappingFactory_instance = GameKeyMappingFactory::new() };
        }
        return (unsafe { GameKeyMappingFactory_instance.clone() }).clone() /* as GameKeyMappingFactory */;
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            intKeyToGameKey: Vec::new(),
        };
        this.intKeyToGameKey = vec![None; (jcall!(InputFactory::InputFactory::getInstance()).MAX) as usize];
        let mut gameKeyFactory: Option<Rc<RefCell<GameKeyFactory>>> = GameKeyFactory::getInstance();
        this.setGameKey(jcall!(gameKeyFactory).NONE.clone());
        this.setGameKey(jcall!(gameKeyFactory).UP.clone());
        this.setGameKey(jcall!(gameKeyFactory).DOWN.clone());
        this.setGameKey(jcall!(gameKeyFactory).LEFT.clone());
        this.setGameKey(jcall!(gameKeyFactory).RIGHT.clone());
        this.setGameKey(jcall!(gameKeyFactory).KEY_POUND.clone());
        this.setGameKey(jcall!(gameKeyFactory).KEY_STAR.clone());
        this.setGameKey(jcall!(gameKeyFactory).KEY_NUM0.clone());
        this.setGameKey(jcall!(gameKeyFactory).KEY_NUM1.clone());
        this.setGameKey(jcall!(gameKeyFactory).KEY_NUM2.clone());
        this.setGameKey(jcall!(gameKeyFactory).KEY_NUM3.clone());
        this.setGameKey(jcall!(gameKeyFactory).KEY_NUM4.clone());
        this.setGameKey(jcall!(gameKeyFactory).KEY_NUM5.clone());
        this.setGameKey(jcall!(gameKeyFactory).KEY_NUM6.clone());
        this.setGameKey(jcall!(gameKeyFactory).KEY_NUM7.clone());
        this.setGameKey(jcall!(gameKeyFactory).KEY_NUM8.clone());
        this.setGameKey(jcall!(gameKeyFactory).KEY_NUM9.clone());
        this.setGameKey(jcall!(gameKeyFactory).GAME_A.clone());
        this.setGameKey(jcall!(gameKeyFactory).GAME_B.clone());
        this.setGameKey(jcall!(gameKeyFactory).GAME_C.clone());
        this.setGameKey(jcall!(gameKeyFactory).GAME_D.clone());
        this.setGameKey(jcall!(gameKeyFactory).FIRE.clone());
        this.setGameKey(jcall!(gameKeyFactory).LEVEL_DOWN.clone());
        this.setGameKey(jcall!(gameKeyFactory).LEVEL_UP.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getGameKey(&mut self, mut key: i32) -> Option<Rc<RefCell<GameKey>>> {
        return self.intKeyToGameKey[(key) as usize].clone();
    }
    

    pub fn setGameKey(&mut self, mut gameKey: Option<Rc<RefCell<GameKey>>>) {
        self.intKeyToGameKey[(jcall!(gameKey).getId()) as usize] = gameKey.clone();
    }
}


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

pub static mut GameKeyMappingFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| { let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }.clone());

impl GameKeyMappingFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GameKeyMappingFactory>>> {
        if match (&unsafe { GameKeyMappingFactory_instance.clone() }, &{ let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *GameKeyMappingFactory_instance = crate::java_into_object(GameKeyMappingFactory::new()) };
        }
        return { let java_object = (unsafe { GameKeyMappingFactory_instance.clone() }).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<GameKeyMappingFactory>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            intKeyToGameKey: Vec::new(),
        };
        this.intKeyToGameKey = vec![None; ({ let java_field = jcall!(InputFactory::InputFactory::getInstance()).MAX.clone(); java_field }) as usize];
        let mut gameKeyFactory: Option<Rc<RefCell<GameKeyFactory>>> = GameKeyFactory::getInstance();
        { let java_arguments = ({ let java_field = jcall!(gameKeyFactory).NONE.clone(); java_field }.clone(),); this.setGameKey(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(gameKeyFactory).UP.clone(); java_field }.clone(),); this.setGameKey(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(gameKeyFactory).DOWN.clone(); java_field }.clone(),); this.setGameKey(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(gameKeyFactory).LEFT.clone(); java_field }.clone(),); this.setGameKey(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(gameKeyFactory).RIGHT.clone(); java_field }.clone(),); this.setGameKey(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(gameKeyFactory).KEY_POUND.clone(); java_field }.clone(),); this.setGameKey(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(gameKeyFactory).KEY_STAR.clone(); java_field }.clone(),); this.setGameKey(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(gameKeyFactory).KEY_NUM0.clone(); java_field }.clone(),); this.setGameKey(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(gameKeyFactory).KEY_NUM1.clone(); java_field }.clone(),); this.setGameKey(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(gameKeyFactory).KEY_NUM2.clone(); java_field }.clone(),); this.setGameKey(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(gameKeyFactory).KEY_NUM3.clone(); java_field }.clone(),); this.setGameKey(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(gameKeyFactory).KEY_NUM4.clone(); java_field }.clone(),); this.setGameKey(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(gameKeyFactory).KEY_NUM5.clone(); java_field }.clone(),); this.setGameKey(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(gameKeyFactory).KEY_NUM6.clone(); java_field }.clone(),); this.setGameKey(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(gameKeyFactory).KEY_NUM7.clone(); java_field }.clone(),); this.setGameKey(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(gameKeyFactory).KEY_NUM8.clone(); java_field }.clone(),); this.setGameKey(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(gameKeyFactory).KEY_NUM9.clone(); java_field }.clone(),); this.setGameKey(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(gameKeyFactory).GAME_A.clone(); java_field }.clone(),); this.setGameKey(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(gameKeyFactory).GAME_B.clone(); java_field }.clone(),); this.setGameKey(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(gameKeyFactory).GAME_C.clone(); java_field }.clone(),); this.setGameKey(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(gameKeyFactory).GAME_D.clone(); java_field }.clone(),); this.setGameKey(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(gameKeyFactory).FIRE.clone(); java_field }.clone(),); this.setGameKey(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(gameKeyFactory).LEVEL_DOWN.clone(); java_field }.clone(),); this.setGameKey(java_arguments.0) };
        { let java_arguments = ({ let java_field = jcall!(gameKeyFactory).LEVEL_UP.clone(); java_field }.clone(),); this.setGameKey(java_arguments.0) };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getGameKey(&mut self, mut key: i32) -> Option<Rc<RefCell<GameKey>>> {
        return self.intKeyToGameKey[(key) as usize].clone();
    }
    

    pub fn setGameKey(&mut self, mut gameKey: Option<Rc<RefCell<GameKey>>>) {
        { let java_index = (jcall!(gameKey).getId()) as usize; self.intKeyToGameKey[java_index] = gameKey.clone(); };
    }
}


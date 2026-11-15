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
use super::GameKeyFactory::GameKeyFactory;
use super::GameKey::GameKey;
use super::InputFactory::InputFactory;
use super::Input::Input;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct J2MEKeyFactory {
    pub gameKeyFactory: Option<Rc<RefCell<GameKeyFactory>>>,
}

pub static J2MEKeyFactory_instance: crate::JavaStatic<Option<Rc<RefCell<J2MEKeyFactory>>>> = crate::JavaStatic::new(|| J2MEKeyFactory::new());

impl J2MEKeyFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<J2MEKeyFactory>>> {
        return J2MEKeyFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            gameKeyFactory: None,
        };
        this.gameKeyFactory = GameKeyFactory::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self) {
        let mut gameKeyArray: Vec<Option<Rc<RefCell<GameKey>>>> = vec![{ let java_field = jcall!(self.gameKeyFactory).DOWN.clone(); java_field }.clone(), { let java_field = jcall!(self.gameKeyFactory).FIRE.clone(); java_field }.clone(), { let java_field = jcall!(self.gameKeyFactory).GAME_A.clone(); java_field }.clone(), { let java_field = jcall!(self.gameKeyFactory).GAME_B.clone(); java_field }.clone(), { let java_field = jcall!(self.gameKeyFactory).GAME_C.clone(); java_field }.clone(), { let java_field = jcall!(self.gameKeyFactory).GAME_D.clone(); java_field }.clone(), { let java_field = jcall!(self.gameKeyFactory).KEY_NUM0.clone(); java_field }.clone(), { let java_field = jcall!(self.gameKeyFactory).KEY_NUM1.clone(); java_field }.clone(), { let java_field = jcall!(self.gameKeyFactory).KEY_NUM2.clone(); java_field }.clone(), { let java_field = jcall!(self.gameKeyFactory).KEY_NUM3.clone(); java_field }.clone(), { let java_field = jcall!(self.gameKeyFactory).KEY_NUM4.clone(); java_field }.clone(), { let java_field = jcall!(self.gameKeyFactory).KEY_NUM5.clone(); java_field }.clone(), { let java_field = jcall!(self.gameKeyFactory).KEY_NUM6.clone(); java_field }.clone(), { let java_field = jcall!(self.gameKeyFactory).KEY_NUM7.clone(); java_field }.clone(), { let java_field = jcall!(self.gameKeyFactory).KEY_NUM8.clone(); java_field }.clone(), { let java_field = jcall!(self.gameKeyFactory).KEY_NUM9.clone(); java_field }.clone(), { let java_field = jcall!(self.gameKeyFactory).KEY_POUND.clone(); java_field }.clone(), { let java_field = jcall!(self.gameKeyFactory).KEY_STAR.clone(); java_field }.clone(), { let java_field = jcall!(self.gameKeyFactory).LEFT.clone(); java_field }.clone(), { let java_field = jcall!(self.gameKeyFactory).LEVEL_DOWN.clone(); java_field }.clone(), { let java_field = jcall!(self.gameKeyFactory).LEVEL_UP.clone(); java_field }.clone(), { let java_field = jcall!(self.gameKeyFactory).NONE.clone(); java_field }.clone(), { let java_field = jcall!(self.gameKeyFactory).RIGHT.clone(); java_field }.clone(), { let java_field = jcall!(self.gameKeyFactory).UP.clone(); java_field }.clone()];
        let mut inputFactory: Option<Rc<RefCell<InputFactory>>> = InputFactory::getInstance();
        {
            let mut index: i32 = (gameKeyArray.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            jcall!(inputFactory).add(jcall!(gameKeyArray[(index) as usize]).getId(), gameKeyArray[(index) as usize].clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
            }
        }
    }
    

    pub fn isSubmission(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        if match (&input, &{ let java_field = jcall!(self.gameKeyFactory).FIRE.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isDelete(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        if match (&input, &{ let java_field = jcall!(self.gameKeyFactory).GAME_D.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isBackSpace(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        return false;
    }
    

    pub fn isUp(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        if match (&input, &{ let java_field = jcall!(self.gameKeyFactory).UP.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isDown(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        if match (&input, &{ let java_field = jcall!(self.gameKeyFactory).DOWN.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isLeft(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        if match (&input, &{ let java_field = jcall!(self.gameKeyFactory).LEFT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isRight(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        if match (&input, &{ let java_field = jcall!(self.gameKeyFactory).RIGHT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
    

    pub fn isEnter(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        if match (&input, &{ let java_field = jcall!(self.gameKeyFactory).FIRE.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
}


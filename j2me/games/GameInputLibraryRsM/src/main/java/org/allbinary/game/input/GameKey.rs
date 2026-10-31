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
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::Input::Input;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameKey {
    pub base: Input,
}

thread_local! { pub static GameKey_NULL_GAME_KEY: Option<Rc<RefCell<GameKey>>> = GameKey::new(-1, jcall!(StringUtil::getInstance()).NULL_STRING.clone()); }

impl GameKey {
    

    pub fn new_struct(mut key: i32, mut name: String) -> Self {
        let mut this = Self {
            base: Input::new_struct(key.clone(), name.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut key: i32, mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(key, name))))
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(StringMaker::new()).append(String::from("GameKey ")).append(self.base.toString()).toString();
    }
}
impl std::ops::Deref for GameKey {
    type Target = Input;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GameKey {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::Input::InputDyn for GameKey {
    fn as_Input(&self) -> &crate::org::allbinary::game::input::Input::Input { &self.base }
    fn as_Input_mut(&mut self) -> &mut crate::org::allbinary::game::input::Input::Input { &mut self.base }
    fn toString(&mut self) -> String { Self::toString(self) }
}



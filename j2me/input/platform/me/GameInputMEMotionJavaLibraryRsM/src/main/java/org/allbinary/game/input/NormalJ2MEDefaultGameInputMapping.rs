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
use crate::org::allbinary::game::configuration::persistance::GamePersistanceStrings::GamePersistanceStrings;
use crate::org::allbinary::game::input::mapping::InputToGameKeyMapping::InputToGameKeyMapping;
use crate::org::allbinary::game::input::mapping::PersistentInputMapping::PersistentInputMapping;
use super::GameKeyFactory::GameKeyFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NormalJ2MEDefaultGameInputMapping {
    pub base: PersistentInputMapping,
}


impl NormalJ2MEDefaultGameInputMapping {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: PersistentInputMapping::new_struct({ let java_field = jcall!(GamePersistanceStrings::getInstance()).DEFAULT_INPUT_MAPPING_RECORD_ID.clone(); java_field }.clone()),
        };
        // super(...) is in the struct literal above
        let mut inputToGameKeyMapping: Option<Rc<RefCell<InputToGameKeyMapping>>> = this.getInputMapping();
        let mut gameKeyFactory: Option<Rc<RefCell<GameKeyFactory>>> = GameKeyFactory::getInstance();
        jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).UP.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(gameKeyFactory).KEY_NUM2.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).LEFT.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(gameKeyFactory).KEY_NUM4.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).RIGHT.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(gameKeyFactory).KEY_NUM6.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).DOWN.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(gameKeyFactory).KEY_NUM8.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).KEY_NUM1.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(gameKeyFactory).GAME_A.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).KEY_NUM3.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(gameKeyFactory).GAME_B.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).KEY_NUM7.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(gameKeyFactory).GAME_C.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).KEY_NUM9.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(gameKeyFactory).GAME_D.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).LEVEL_DOWN.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(gameKeyFactory).LEVEL_DOWN.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).LEVEL_UP.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(gameKeyFactory).LEVEL_UP.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).UP.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(gameKeyFactory).UP.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).DOWN.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(gameKeyFactory).DOWN.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).LEFT.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(gameKeyFactory).LEFT.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).RIGHT.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(gameKeyFactory).RIGHT.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).KEY_POUND.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(gameKeyFactory).KEY_POUND.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).KEY_STAR.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(gameKeyFactory).KEY_STAR.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).KEY_NUM1.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(gameKeyFactory).KEY_NUM1.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).KEY_NUM3.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(gameKeyFactory).KEY_NUM3.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).KEY_NUM5.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(gameKeyFactory).KEY_NUM5.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).KEY_NUM7.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(gameKeyFactory).KEY_NUM7.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).KEY_NUM9.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(gameKeyFactory).KEY_NUM9.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).KEY_NUM0.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(gameKeyFactory).KEY_NUM0.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).KEY_NUM1.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(gameKeyFactory).FIRE.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl std::ops::Deref for NormalJ2MEDefaultGameInputMapping {
    type Target = PersistentInputMapping;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NormalJ2MEDefaultGameInputMapping {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::mapping::PersistentInputMapping::PersistentInputMappingDyn for NormalJ2MEDefaultGameInputMapping {
    fn as_PersistentInputMapping(&self) -> &crate::org::allbinary::game::input::mapping::PersistentInputMapping::PersistentInputMapping { &self.base }
    fn as_PersistentInputMapping_mut(&mut self) -> &mut crate::org::allbinary::game::input::mapping::PersistentInputMapping::PersistentInputMapping { &mut self.base }
}



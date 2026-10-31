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
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::PlayerGameInput::PlayerGameInput;
use super::GameInputProcessorInterface::GameInputProcessorInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameInputProcessorComposite {
    pub base: PlayerGameInput,
    pub name: String,
    pub gameInputProcessorInterface: Option<Rc<RefCell<dyn GameInputProcessorInterface>>>,
    pub NAME_LABEL: String,
}

impl crate::org::allbinary::game::input::event::GameKeyEventListenerInterface::GameKeyEventListenerInterface for GameInputProcessorComposite {
}

impl GameInputProcessorComposite {
    

    pub fn new_struct(mut name: String, mut gameInputProcessorInterface: Option<Rc<RefCell<dyn GameInputProcessorInterface>>>) -> Self {
        let mut this = Self {
            base: PlayerGameInput::new_struct(BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>), -1),
            name: String::new(),
            gameInputProcessorInterface: None,
            NAME_LABEL: String::new(),
        };
        this.NAME_LABEL = String::from(" GameInputProcessorInterface: ");
        // super(...) is in the struct literal above
        this.name = name.clone();
        this.gameInputProcessorInterface = gameInputProcessorInterface.clone();
        this
    }

    pub fn new(mut name: String, mut gameInputProcessorInterface: Option<Rc<RefCell<dyn GameInputProcessorInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, gameInputProcessorInterface))))
    }
    

    // synchronized
    pub fn update(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getGameKeyEventList();
            jcall!(self.gameInputProcessorInterface).onInput(list.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).UPDATE.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(StringMaker::new()).append(self.base.toString()).append(self.NAME_LABEL.clone()).append(self.name.clone()).toString();
    }
}
impl std::ops::Deref for GameInputProcessorComposite {
    type Target = PlayerGameInput;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GameInputProcessorComposite {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::GameInput::GameInputDyn for GameInputProcessorComposite {
    fn as_GameInput(&self) -> &crate::org::allbinary::game::input::GameInput::GameInput { &self.base.base }
    fn as_GameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::GameInput::GameInput { &mut self.base.base }
    fn update(&mut self) { Self::update(self) }
}
impl crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn for GameInputProcessorComposite {
    fn as_PlayerGameInput(&self) -> &crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &self.base }
    fn as_PlayerGameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &mut self.base }
}



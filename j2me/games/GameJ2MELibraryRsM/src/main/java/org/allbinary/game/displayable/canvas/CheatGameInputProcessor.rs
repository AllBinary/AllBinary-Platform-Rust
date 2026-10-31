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
// package org.allbinary.game.displayable.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::GameKeyFactory::GameKeyFactory;
use crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::game::input::event::GameKeyEventUtil::GameKeyEventUtil;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::layer::special::SpecialGameInputFactory::SpecialGameInputFactory;
use crate::org::allbinary::game::layer::special::SpecialGameInputInterface::SpecialGameInputInterface;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::AllBinaryGameCanvas::AllBinaryGameCanvas;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CheatGameInputProcessor {
    pub base: PlayerGameInput,
    pub PROCESS_GAME: String,
    pub gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>,
    pub gameKeyFactory: Option<Rc<RefCell<GameKeyFactory>>>,
}

pub static mut CheatGameInputProcessor_inputProcessor: std::cell::LazyCell<Option<Rc<RefCell<dyn SpecialGameInputInterface>>>> = std::cell::LazyCell::new(|| crate::org::allbinary::game::layer::special::SpecialGameInputFactory::SpecialGameInputFactory_NO_SPECIAL_GAME_INPUT.with(Clone::clone).clone());
impl crate::org::allbinary::game::input::event::GameKeyEventListenerInterface::GameKeyEventListenerInterface for CheatGameInputProcessor {
}

impl CheatGameInputProcessor {
    

    pub fn new_struct(mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) -> Self {
        let mut this = Self {
            base: PlayerGameInput::new_struct(BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>), -1),
            PROCESS_GAME: String::new(),
            gameCanvas: None,
            gameKeyFactory: None,
        };
        this.PROCESS_GAME = String::from("processGame");
        this.gameKeyFactory = GameKeyFactory::getInstance();
        // super(...) is in the struct literal above
        this.gameCanvas = gameCanvas.clone();
        this
    }

    pub fn new(mut gameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(gameCanvas))))
    }
    

    // synchronized
    pub fn update(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut gameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>> = jcall!(self.gameCanvas).getLayerManager();
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getGameKeyEventList();
            let mut size: i32 = jcall!(list).size();
            for index in (0..size) {
                let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize].clone();
                let mut key: i32 = GameKeyEventUtil::getKey(anyType.clone());
                if key == jcall!(jcall!(self.gameKeyFactory).LEVEL_DOWN).getId() {
                    jcall!(unsafe { CheatGameInputProcessor_inputProcessor.clone() }).strafeLeft();
                    break;
                } else if key == jcall!(jcall!(self.gameKeyFactory).LEVEL_UP).getId() {
                    jcall!(unsafe { CheatGameInputProcessor_inputProcessor.clone() }).strafeRight();
                    break;
                }
                if key == jcall!(jcall!(self.gameKeyFactory).LEFT).getId() {
                    jcall!(unsafe { CheatGameInputProcessor_inputProcessor.clone() }).left();
                    break;
                } else if key == jcall!(jcall!(self.gameKeyFactory).RIGHT).getId() {
                    jcall!(unsafe { CheatGameInputProcessor_inputProcessor.clone() }).right();
                    break;
                } else if key == jcall!(jcall!(self.gameKeyFactory).UP).getId() {
                    jcall!(unsafe { CheatGameInputProcessor_inputProcessor.clone() }).up();
                    break;
                } else if key == jcall!(jcall!(self.gameKeyFactory).DOWN).getId() {
                    jcall!(unsafe { CheatGameInputProcessor_inputProcessor.clone() }).down();
                    break;
                } else if key == jcall!(jcall!(self.gameKeyFactory).KEY_NUM1).getId() {
                    jcall!(unsafe { CheatGameInputProcessor_inputProcessor.clone() }).special1(gameLayerManager.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>), crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent_NONE.with(Clone::clone).clone());
                    break;
                } else if key == jcall!(jcall!(self.gameKeyFactory).KEY_NUM3).getId() {
                    jcall!(unsafe { CheatGameInputProcessor_inputProcessor.clone() }).special2(gameLayerManager.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>), crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent_NONE.with(Clone::clone).clone());
                    break;
                }
            }
            jcall!(list).clear();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).UPDATE.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}
impl std::ops::Deref for CheatGameInputProcessor {
    type Target = PlayerGameInput;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CheatGameInputProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::GameInput::GameInputDyn for CheatGameInputProcessor {
    fn as_GameInput(&self) -> &crate::org::allbinary::game::input::GameInput::GameInput { &self.base.base }
    fn as_GameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::GameInput::GameInput { &mut self.base.base }
    fn update(&mut self) { Self::update(self) }
}
impl crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn for CheatGameInputProcessor {
    fn as_PlayerGameInput(&self) -> &crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &self.base }
    fn as_PlayerGameInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput { &mut self.base }
}



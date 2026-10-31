// Generated Code Do Not Modify
// package org.allbinary.game.midlet
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::state::GameState::GameState;
use crate::org::allbinary::game::state::GameStateFactory::GameStateFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameMidletStateFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub currentGameState: Option<Rc<RefCell<GameState>>>,
}

thread_local! { pub static GameMidletStateFactory_INSTANCE: Option<Rc<RefCell<GameMidletStateFactory>>> = GameMidletStateFactory::new(); }

impl GameMidletStateFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            currentGameState: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.currentGameState = jcall!(GameStateFactory::getInstance()).NO_GAME_STATE.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GameMidletStateFactory>>> {
        return GameMidletStateFactory_INSTANCE.with(Clone::clone).clone();
    }
    

    pub fn getCurrentGameState(&mut self) -> Option<Rc<RefCell<GameState>>> {
        return self.currentGameState.clone();
    }
    

    pub fn setCurrentGameState(&mut self, mut currentGameState: Option<Rc<RefCell<GameState>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(CommonLabels::getInstance()).CURRENT.clone()).append(String::from(" GameState: ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(currentGameState.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setCurrentGameState"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        self.currentGameState = currentGameState.clone();
    }
}


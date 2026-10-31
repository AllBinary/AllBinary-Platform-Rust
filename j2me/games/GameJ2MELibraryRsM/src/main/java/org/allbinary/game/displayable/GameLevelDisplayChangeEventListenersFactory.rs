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
// package org.allbinary.game.displayable
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::displayable::event::DisplayChangeEventHandler::DisplayChangeEventHandler;
use crate::org::allbinary::graphics::displayable::event::DisplayChangeEventListener::DisplayChangeEventListener;
use crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameLevelDisplayChangeEventListenersFactory {
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static GameLevelDisplayChangeEventListenersFactory_instance: Option<Rc<RefCell<GameLevelDisplayChangeEventListenersFactory>>> = GameLevelDisplayChangeEventListenersFactory::new(); }

impl GameLevelDisplayChangeEventListenersFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            list: None,
        };
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GameLevelDisplayChangeEventListenersFactory>>> {
        return GameLevelDisplayChangeEventListenersFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn add(&mut self, mut displayChangeEventListener: Option<Rc<RefCell<dyn DisplayChangeEventListener>>>) {
        jcall!(self.list).add(crate::java_into_object(displayChangeEventListener.clone()));
        jcall!(DisplayChangeEventHandler::getInstance()).addListenerInterface(displayChangeEventListener.clone());
    }
    

    pub fn clear(&mut self) {
        let mut displayChangeEventHandler: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn>>> = DisplayChangeEventHandler::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn>>);
        let mut displayChangeEventListener: Option<Rc<RefCell<dyn DisplayChangeEventListener>>> = None;
        {
            let mut index: i32 = jcall!(self.list).size();
            while { index -= 1; index } >= 0 {
            displayChangeEventListener = { let java_object = (jcall!(self.list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn DisplayChangeEventListener>>>>().expect("ClassCastException").clone()) };
            jcall!(displayChangeEventHandler).removeListener(displayChangeEventListener.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>));
            }
        }
        jcall!(self.list).clear();
    }
}


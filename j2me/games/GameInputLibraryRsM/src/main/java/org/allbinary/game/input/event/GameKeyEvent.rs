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
// package org.allbinary.game.input.event
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameKeyEvent {
    pub base: AllBinaryEventObject,
    pub sourceId: i32,
    pub repeated: bool,
    pub key: i32,
    pub gameActionKey: i32,
    pub originalKey: i32,
}

thread_local! { pub static GameKeyEvent_NONE: Option<Rc<RefCell<GameKeyEvent>>> = GameKeyEvent::new(jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(), -1, -1, -1, -1, false); }

impl GameKeyEvent {
    

    pub fn createEvent(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut sourceId: i32, mut key: i32) -> Option<Rc<RefCell<GameKeyEvent>>> {
        return GameKeyEvent::new(anyType.clone(), sourceId.clone(), 0, 0, key.clone(), false);
    }
    

    pub fn new_struct(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut sourceId: i32, mut originalKey: i32, mut gameActionKey: i32, mut key: i32, mut repeated: bool) -> Self {
        let mut this = Self {
            base: AllBinaryEventObject::new_struct(anyType.clone()),
            sourceId: 0,
            repeated: false,
            key: 0,
            gameActionKey: 0,
            originalKey: 0,
        };
        // super(...) is in the struct literal above
        this.sourceId = sourceId.clone();
        this.setOriginalKey(originalKey.clone());
        this.key = key.clone();
        this.setGameActionKey(gameActionKey.clone());
        this.setRepeatEvents(repeated.clone());
        this
    }

    pub fn new(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut sourceId: i32, mut originalKey: i32, mut gameActionKey: i32, mut key: i32, mut repeated: bool) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(anyType, sourceId, originalKey, gameActionKey, key, repeated))))
    }
    

    pub fn init(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        self.setSource(anyType.clone());
    }
    

    pub fn setRepeatEvents(&mut self, mut repeated: bool) {
        self.repeated = repeated.clone();
    }
    

    pub fn hasRepeatEvents(&mut self) -> bool {
        return self.repeated.clone();
    }
    

    pub fn getKey(&mut self) -> i32 {
        return self.key.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("\nKey: "));
        jcall!(stringBuffer).appendint(self.getKey());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn getOriginalKey(&mut self) -> i32 {
        return self.originalKey.clone();
    }
    

    pub fn setOriginalKey(&mut self, mut originalKey: i32) {
        self.originalKey = originalKey.clone();
    }
    

    pub fn getGameActionKey(&mut self) -> i32 {
        return self.gameActionKey.clone();
    }
    

    pub fn setGameActionKey(&mut self, mut gameActionKey: i32) {
        self.gameActionKey = gameActionKey.clone();
    }
    

    pub fn getSourceId(&mut self) -> i32 {
        return self.sourceId.clone();
    }
}
impl std::ops::Deref for GameKeyEvent {
    type Target = AllBinaryEventObject;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GameKeyEvent {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn for GameKeyEvent {
    fn as_AllBinaryEventObject(&self) -> &crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &self.base }
    fn as_AllBinaryEventObject_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &mut self.base }
}



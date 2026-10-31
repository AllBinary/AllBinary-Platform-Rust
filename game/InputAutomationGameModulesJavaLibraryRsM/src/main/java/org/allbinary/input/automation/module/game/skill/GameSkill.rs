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
// package org.allbinary.input.automation.module.game.skill
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use super::GameSkillType::GameSkillType;
use super::GameSkillPropertyInterface::GameSkillPropertyInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameSkill {
    pub gameSkillType: Option<Rc<RefCell<GameSkillType>>>,
    pub extra: String,
    pub time: i32,
    pub vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub GAME_SKILL_LABEL: String,
    pub EXTRA_LABEL: String,
    pub TIME_LABEL: String,
}

pub static mut GameSkill_hashMap: std::cell::LazyCell<Option<Rc<RefCell<HashMap>>>> = std::cell::LazyCell::new(|| jcall!(StdUtil::getInstance()).createHashMap());

impl GameSkill {
    

    pub fn new_struct(mut gameSkillType: Option<Rc<RefCell<GameSkillType>>>, mut extra: String, mut time: i32) -> Self {
        let mut this = Self {
            gameSkillType: None,
            extra: String::new(),
            time: 0,
            vector: None,
            GAME_SKILL_LABEL: String::new(),
            EXTRA_LABEL: String::new(),
            TIME_LABEL: String::new(),
        };
        this.GAME_SKILL_LABEL = String::from("GameSkill: ");
        this.EXTRA_LABEL = String::from("Extra: ");
        this.TIME_LABEL = String::from("Time: ");
        this.setGameSkillType(gameSkillType.clone());
        this.setExtra(extra.clone());
        this.setTime(time.clone());
        this.vector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new(mut gameSkillType: Option<Rc<RefCell<GameSkillType>>>, mut extra: String, mut time: i32) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(gameSkillType, extra, time))));
            jcall!(unsafe { GameSkill_hashMap.clone() }).put(jcall!(StringMaker::new()).append(jcall!(this.getGameSkillType()).getName()).append(jcall!(CommonSeps::getInstance()).SPACE.clone()).append(extra.clone()).toString(), (|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn getGameSkillType(&mut self) -> Option<Rc<RefCell<GameSkillType>>> {
        return self.gameSkillType.clone();
    }
    

    pub fn setGameSkillType(&mut self, mut gameSkillType: Option<Rc<RefCell<GameSkillType>>>) {
        self.gameSkillType = gameSkillType.clone();
    }
    

    pub fn getExtra(&mut self) -> String {
        return self.extra.clone();
    }
    

    pub fn setExtra(&mut self, mut extra: String) {
        self.extra = extra.clone();
    }
    

    pub fn getTime(&mut self) -> i32 {
        return self.time.clone();
    }
    

    pub fn setTime(&mut self, mut time: i32) {
        self.time = time.clone();
    }
    

    pub fn addProperty(&mut self, mut gameSkillPropertyInterface: Option<Rc<RefCell<dyn GameSkillPropertyInterface>>>) {
        jcall!(self.vector).add(crate::java_into_object(gameSkillPropertyInterface.clone()));
    }
    

    pub fn getProperties(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.vector.clone();
    }
    

    pub fn getInstance(mut gameSkillType: Option<Rc<RefCell<GameSkillType>>>, mut extra: String) -> Option<Rc<RefCell<dyn GameSkillDyn>>> {
        return (jcall!(unsafe { GameSkill_hashMap.clone() }).get(jcall!(StringMaker::new()).append(jcall!(gameSkillType).getName()).append(jcall!(CommonSeps::getInstance()).SPACE.clone()).append(extra.clone()).toString())).clone() /* as GameSkill */;
    }
    

    pub fn toString(&mut self) -> String {
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(self.GAME_SKILL_LABEL.clone());
        jcall!(stringBuffer).append(jcall!(self.getGameSkillType()).toString());
        jcall!(stringBuffer).append(jcall!(commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(self.EXTRA_LABEL.clone());
        jcall!(stringBuffer).append(self.getExtra());
        jcall!(stringBuffer).append(jcall!(commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(self.TIME_LABEL.clone());
        jcall!(stringBuffer).appendlong(((self.getTime()) as i64));
        return jcall!(stringBuffer).toString();
    }
}
pub trait GameSkillDyn {
    fn as_GameSkill(&self) -> &GameSkill;
    fn as_GameSkill_mut(&mut self) -> &mut GameSkill;
    fn getGameSkillType(&mut self) -> Option<Rc<RefCell<GameSkillType>>> { GameSkill::getGameSkillType(self.as_GameSkill_mut()) }
    fn setGameSkillType(&mut self, mut gameSkillType: Option<Rc<RefCell<GameSkillType>>>) { GameSkill::setGameSkillType(self.as_GameSkill_mut(), gameSkillType) }
    fn getExtra(&mut self) -> String { GameSkill::getExtra(self.as_GameSkill_mut()) }
    fn setExtra(&mut self, mut extra: String) { GameSkill::setExtra(self.as_GameSkill_mut(), extra) }
    fn getTime(&mut self) -> i32 { GameSkill::getTime(self.as_GameSkill_mut()) }
    fn setTime(&mut self, mut time: i32) { GameSkill::setTime(self.as_GameSkill_mut(), time) }
    fn addProperty(&mut self, mut gameSkillPropertyInterface: Option<Rc<RefCell<dyn GameSkillPropertyInterface>>>) { GameSkill::addProperty(self.as_GameSkill_mut(), gameSkillPropertyInterface) }
    fn getProperties(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { GameSkill::getProperties(self.as_GameSkill_mut()) }
    fn toString(&mut self) -> String { GameSkill::toString(self.as_GameSkill_mut()) }
}
impl GameSkillDyn for GameSkill {
    fn as_GameSkill(&self) -> &GameSkill { self }
    fn as_GameSkill_mut(&mut self) -> &mut GameSkill { self }
}



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
// package org.allbinary.game
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::GameInfoData::GameInfoData;
use super::GameType::GameType;
use super::GameMode::GameMode;
use super::PlayerType::PlayerType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameInfo {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub gameInfoData: Option<Rc<RefCell<GameInfoData>>>,
    pub gameType: Option<Rc<RefCell<GameType>>>,
    pub gameMode: Option<Rc<RefCell<GameMode>>>,
    pub playerType: Option<Rc<RefCell<PlayerType>>>,
    pub highestLevel: i32,
    pub currentLevel: i32,
    pub NEW_LEVEL: String,
    pub SET_CURRENT_LEVEL: String,
    pub NEXT_GAME_LEVEL: String,
}

thread_local! { pub static GameInfo_NONE: Option<Rc<RefCell<GameInfo>>> = GameInfo::new(jcall!(GameTypeFactory::GameTypeFactory::getInstance()).NONE.clone(), unsafe { GameMode_NONE.clone() }.clone(), jcall!(PlayerTypesFactory::PlayerTypesFactory::getInstance()).PLAYER_TYPE_ONE.clone(), -1, -1); }
thread_local! { pub static GameInfo_LEVEL_NAME: String = String::from("LEVEL"); }

impl GameInfo {
    

    pub fn new_struct(mut gameType: Option<Rc<RefCell<GameType>>>, mut gameMode: Option<Rc<RefCell<GameMode>>>, mut playerType: Option<Rc<RefCell<PlayerType>>>, mut highestLevel: i32, mut currentLevel: i32) -> Self {
        let mut this = Self {
            logUtil: None,
            gameInfoData: None,
            gameType: None,
            gameMode: None,
            playerType: None,
            highestLevel: 0,
            currentLevel: 0,
            NEW_LEVEL: String::new(),
            SET_CURRENT_LEVEL: String::new(),
            NEXT_GAME_LEVEL: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.gameInfoData = GameInfoData::getInstance();
        this.NEW_LEVEL = String::from("New Level: ");
        this.SET_CURRENT_LEVEL = String::from("setCurrentLevel");
        this.NEXT_GAME_LEVEL = String::from("nextGameLevel");
        this.gameType = gameType.clone();
        this.gameMode = gameMode.clone();
        this.setHighestLevel(highestLevel.clone());
        this.currentLevel = currentLevel.clone();
        this.playerType = playerType.clone();
        this
    }

    pub fn new(mut gameType: Option<Rc<RefCell<GameType>>>, mut gameMode: Option<Rc<RefCell<GameMode>>>, mut playerType: Option<Rc<RefCell<PlayerType>>>, mut highestLevel: i32, mut currentLevel: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(gameType, gameMode, playerType, highestLevel, currentLevel))))
    }
    

    pub fn getGameType(&mut self) -> Option<Rc<RefCell<GameType>>> {
        return self.gameType.clone();
    }
    

    pub fn getGameMode(&mut self) -> Option<Rc<RefCell<GameMode>>> {
        return self.gameMode.clone();
    }
    

    pub fn getPlayerType(&mut self) -> Option<Rc<RefCell<PlayerType>>> {
        return self.playerType.clone();
    }
    

    pub fn getHighestLevel(&mut self) -> i32 {
        return self.highestLevel.clone();
    }
    

    pub fn setHighestLevel(&mut self, mut highestLevel: i32) {
        self.highestLevel = highestLevel.clone();
    }
    

    pub fn getCurrentLevel(&mut self) -> i32 {
        return self.currentLevel.clone();
    }
    

    pub fn setCurrentLevel(&mut self, mut currentLevel: i32) {
        self.currentLevel = currentLevel.clone();
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(self.NEW_LEVEL.clone()).appendint(self.getCurrentLevel()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.SET_CURRENT_LEVEL.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn nextGameLevel(&mut self) {
        if self.getCurrentLevel() < self.getHighestLevel() {
            self.currentLevel += 1;
        }
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(CommonLabels::getInstance()).NEXT.clone()).append(jcall!(CommonSeps::getInstance()).SPACE.clone()).append(self.NEW_LEVEL.clone()).appendint(self.getCurrentLevel()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.NEXT_GAME_LEVEL.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn previousGameLevel(&mut self) {
        if self.getCurrentLevel() > 1 {
            self.currentLevel -= 1;
        }
    }
    

    pub fn isLastLevel(&mut self) -> bool {
        if self.getCurrentLevel() >= self.getHighestLevel() {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn toHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = jcall!(StdUtil::getInstance()).createHashtable();
        jcall!(hashtable).put(crate::java_into_object(jcall!(self.gameInfoData).GAME_TYPE.clone()), crate::java_into_object(jcall!(self.getGameType()).toString()));
        jcall!(hashtable).put(crate::java_into_object(jcall!(self.gameInfoData).GAME_MODE.clone()), crate::java_into_object(jcall!(self.gameMode).toString()));
        jcall!(hashtable).put(crate::java_into_object(jcall!(self.gameInfoData).PLAYER_TYPE.clone()), crate::java_into_object(jcall!(self.playerType).toString()));
        jcall!(hashtable).put(crate::java_into_object(jcall!(self.gameInfoData).HIGHEST_LEVEL.clone()), crate::java_into_object((self.getHighestLevel()).to_string()));
        jcall!(hashtable).put(crate::java_into_object(jcall!(self.gameInfoData).CURRENT_LEVEL.clone()), crate::java_into_object((self.currentLevel.clone()).to_string()));
        return hashtable.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        jcall!(stringBuffer).append(jcall!(self.gameInfoData).GAME_TYPE.clone());
        jcall!(stringBuffer).append(jcall!(commonSeps).EQUALS.clone());
        jcall!(stringBuffer).append(jcall!(self.getGameType()).toString());
        jcall!(stringBuffer).append(jcall!(commonSeps).COMMA_SEP.clone());
        jcall!(stringBuffer).append(jcall!(self.gameInfoData).GAME_MODE.clone());
        jcall!(stringBuffer).append(jcall!(commonSeps).EQUALS.clone());
        jcall!(stringBuffer).append(jcall!(self.gameMode).toString());
        jcall!(stringBuffer).append(jcall!(commonSeps).COMMA_SEP.clone());
        jcall!(stringBuffer).append(jcall!(self.gameInfoData).PLAYER_TYPE.clone());
        jcall!(stringBuffer).append(jcall!(commonSeps).EQUALS.clone());
        jcall!(stringBuffer).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(self.playerType.clone())));
        jcall!(stringBuffer).append(jcall!(commonSeps).COMMA_SEP.clone());
        jcall!(stringBuffer).append(jcall!(self.gameInfoData).HIGHEST_LEVEL.clone());
        jcall!(stringBuffer).append(jcall!(commonSeps).EQUALS.clone());
        jcall!(stringBuffer).appendint(self.getHighestLevel());
        jcall!(stringBuffer).append(jcall!(commonSeps).COMMA_SEP.clone());
        jcall!(stringBuffer).append(jcall!(self.gameInfoData).CURRENT_LEVEL.clone());
        jcall!(stringBuffer).append(jcall!(commonSeps).EQUALS.clone());
        jcall!(stringBuffer).appendint(self.currentLevel.clone());
        return jcall!(stringBuffer).toString();
    }
}


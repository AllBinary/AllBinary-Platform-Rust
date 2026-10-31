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
// package org.allbinary.game.score
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::ByteArrayOutputStream::ByteArrayOutputStream;
// use crate::java::io::DataOutputStream::DataOutputStream;
use crate::org::allbinary::game::GameInfo::GameInfo;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HighScore {
    pub id: i32,
    pub name: String,
    pub gameInfo: Option<Rc<RefCell<GameInfo>>>,
    pub score: i64,
    pub scoreString: String,
}


impl HighScore {
    

    pub fn new_struct(mut id: i32, mut name: String, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut score: i64) -> Self {
        let mut this = Self {
            id: 0,
            name: String::new(),
            gameInfo: None,
            score: 0_i64,
            scoreString: String::new(),
        };
        this.id = id.clone();
        this.name = name.clone();
        this.gameInfo = gameInfo.clone();
        this.score = score.clone();
        this.scoreString = (this.score.clone()).to_string();
        this
    }

    pub fn new(mut id: i32, mut name: String, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut score: i64) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(id, name, gameInfo, score))))
    }
    

    pub fn getId(&mut self) -> i32 {
        return self.id.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn getScore(&mut self) -> i64 {
        return self.score.clone();
    }
    

    // throws Exception
    pub fn getAsBytes(&mut self) -> Vec<i8> {
        let mut byteArrayOutputStream: Vec<u8> = Vec::new();
        let mut outputStream: Option<Rc<RefCell<DataOutputStream>>> = DataOutputStream::new(byteArrayOutputStream.clone());
        jcall!(outputStream).writeUTF(self.getName());
        jcall!(outputStream).writeLong(self.getScore());
        return jcall!(byteArrayOutputStream).toByteArray();
    }
    

    pub fn getGameInfo(&mut self) -> Option<Rc<RefCell<GameInfo>>> {
        return self.gameInfo.clone();
    }
    

    pub fn getScoreString(&mut self) -> String {
        return self.scoreString.clone();
    }
    

    pub fn setName(&mut self, mut name: String) {
        self.name = name.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        return jcall!(StringMaker::new()).append(self.name.clone()).append(jcall!(commonSeps).COLON.clone()).appendlong(self.score.clone()).append(jcall!(commonSeps).FORWARD_SLASH.clone()).append(self.scoreString.clone()).toString();
    }
}


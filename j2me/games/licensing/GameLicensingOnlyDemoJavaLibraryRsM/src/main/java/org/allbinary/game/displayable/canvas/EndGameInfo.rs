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
use crate::org::allbinary::game::GameInfo::GameInfo;
use crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct EndGameInfo {
    pub NORMAL_GAME_OVER_WIN: String,
    pub TEASER_DEMO_GAME_END_LAST_LEVEL: String,
    pub TEASER_DEMO_GAME_END: String,
    pub gameOverStr: String,
    pub winStr: String,
}


impl EndGameInfo {

    pub fn new_struct() -> Self {
        let mut this = Self {
            NORMAL_GAME_OVER_WIN: String::new(),
            TEASER_DEMO_GAME_END_LAST_LEVEL: String::new(),
            TEASER_DEMO_GAME_END: String::new(),
            gameOverStr: String::new(),
            winStr: String::new(),
        };
        this.NORMAL_GAME_OVER_WIN = String::from("You Won");
        this.TEASER_DEMO_GAME_END_LAST_LEVEL = String::from("End of Demo.");
        this.TEASER_DEMO_GAME_END = String::from("Demo Game Over.");
        this.gameOverStr = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.winStr = this.gameOverStr.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn update(&mut self, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut myCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>>) {
        if jcall!(gameInfo).isLastLevel() {
            self.setWinStr(self.TEASER_DEMO_GAME_END_LAST_LEVEL.clone());
            self.setGameOverStr(self.TEASER_DEMO_GAME_END_LAST_LEVEL.clone());
        } else {
            self.setWinStr(self.NORMAL_GAME_OVER_WIN.clone());
            self.setGameOverStr(self.TEASER_DEMO_GAME_END.clone());
        }
    }
    

    pub fn getGameOverStr(&mut self) -> String {
        return self.gameOverStr.clone();
    }
    

    pub fn setGameOverStr(&mut self, mut gameOverStr: String) {
        self.gameOverStr = gameOverStr.clone();
    }
    

    pub fn getWinStr(&mut self) -> String {
        return self.winStr.clone();
    }
    

    pub fn setWinStr(&mut self, mut winStr: String) {
        self.winStr = winStr.clone();
    }
}
pub trait EndGameInfoDyn {
    fn as_EndGameInfo(&self) -> &EndGameInfo;
    fn as_EndGameInfo_mut(&mut self) -> &mut EndGameInfo;
    fn update(&mut self, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut myCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>>) { EndGameInfo::update(self.as_EndGameInfo_mut(), gameInfo, myCanvas) }
    fn getGameOverStr(&mut self) -> String { EndGameInfo::getGameOverStr(self.as_EndGameInfo_mut()) }
    fn setGameOverStr(&mut self, mut gameOverStr: String) { EndGameInfo::setGameOverStr(self.as_EndGameInfo_mut(), gameOverStr) }
    fn getWinStr(&mut self) -> String { EndGameInfo::getWinStr(self.as_EndGameInfo_mut()) }
    fn setWinStr(&mut self, mut winStr: String) { EndGameInfo::setWinStr(self.as_EndGameInfo_mut(), winStr) }
}
impl EndGameInfoDyn for EndGameInfo {
    fn as_EndGameInfo(&self) -> &EndGameInfo { self }
    fn as_EndGameInfo_mut(&mut self) -> &mut EndGameInfo { self }
}



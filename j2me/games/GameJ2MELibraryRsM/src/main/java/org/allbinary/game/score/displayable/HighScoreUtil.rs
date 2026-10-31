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
// package org.allbinary.game.score.displayable
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Command::Command;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
use crate::org::allbinary::game::GameInfo::GameInfo;
use crate::org::allbinary::game::score::HighScore::HighScore;
use crate::org::allbinary::game::score::HighScoreNamePersistanceSingleton::HighScoreNamePersistanceSingleton;
use crate::org::allbinary::game::score::HighScores::HighScores;
use crate::org::allbinary::game::score::HighScoresFactoryInterface::HighScoresFactoryInterface;
use crate::org::allbinary::game::score::HighScoresHelperBaseInterface::HighScoresHelperBaseInterface;
use crate::org::allbinary::game::score::HighScoresResultsListener::HighScoresResultsListener;
use crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HighScoreUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub highScoresFactoryInterface: Option<Rc<RefCell<dyn HighScoresFactoryInterface>>>,
    pub highScoresArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>>,
    pub highScore: Option<Rc<RefCell<HighScore>>>,
    pub abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>,
    pub gameInfo: Option<Rc<RefCell<GameInfo>>>,
    pub highScoresHelper: Option<Rc<RefCell<dyn HighScoresHelperBaseInterface>>>,
    pub firstTime: bool,
}

thread_local! { pub static HighScoreUtil_SUBMIT_TEXTBOX_COMMAND: Option<Rc<RefCell<Command>>> = Command::new(String::from("Submit"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1); }
impl HighScoresResultsListener for HighScoreUtil {
    fn setHighScoresArray(&mut self, mut highScoresArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>>) { Self::setHighScoresArray(self, highScoresArray) }
}

impl HighScoreUtil {
    

    pub fn new_struct(mut highScoresFactoryInterface: Option<Rc<RefCell<dyn HighScoresFactoryInterface>>>, mut highScoresHelper: Option<Rc<RefCell<dyn HighScoresHelperBaseInterface>>>, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut cmdListener: Option<Rc<RefCell<CommandListener>>>, mut name: String, mut highScore: Option<Rc<RefCell<HighScore>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            highScoresFactoryInterface: None,
            highScoresArray: Vec::new(),
            highScore: None,
            abeClientInformation: None,
            gameInfo: None,
            highScoresHelper: None,
            firstTime: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.firstTime = true;
        this.highScoresFactoryInterface = highScoresFactoryInterface.clone();
        this.highScoresHelper = highScoresHelper.clone();
        this.highScoresArray = jcall!(this.highScoresHelper).getHighScoresArray();
        this.highScore = highScore.clone();
        this.abeClientInformation = abeClientInformation.clone();
        this.gameInfo = gameInfo.clone();
        this
    }

    pub fn new(mut highScoresFactoryInterface: Option<Rc<RefCell<dyn HighScoresFactoryInterface>>>, mut highScoresHelper: Option<Rc<RefCell<dyn HighScoresHelperBaseInterface>>>, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut cmdListener: Option<Rc<RefCell<CommandListener>>>, mut name: String, mut highScore: Option<Rc<RefCell<HighScore>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(highScoresFactoryInterface, highScoresHelper, abeClientInformation, gameInfo, cmdListener, name, highScore))))
    }
    

    pub fn setHighScoresArray(&mut self, mut highScoresArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>>) {
        self.highScoresArray = highScoresArray.clone();
        self.firstTime = false;
        self.saveHighScore();
    }
    

    pub fn update(&mut self, mut name: String) {
        jcall!(HighScoreNamePersistanceSingleton::getInstance()).save(self.abeClientInformation.clone(), self.gameInfo.clone(), name.clone());
        jcall!(self.highScore).setName(name.clone());
    }
    

    pub fn saveHighScore(&mut self) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(self.commonStrings).START.clone()).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(self.highScore.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("saveHighScore"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut size: i32 = (self.highScoresArray.len() as i32).clone();
        if self.firstTime && size == 0 {
            jcall!(self.logUtil).putF(String::from("Games canvas did not give us any HighScores"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("saveHighScore"));
            jcall!(self.highScoresFactoryInterface).fetchHighScores(self.gameInfo.clone(), None /* Java this: a &mut self method has no Rc of itself */);
            return;
        }
        let mut highScores: Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>> = None;
        let mut highScoresAsString: String = String::new();
        for index in (0..size) {
            highScores = self.highScoresArray[(index) as usize].clone();
            jcall!(highScores).addHighScore(self.highScore.clone());
            highScoresAsString = jcall!(highScores).toString();
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Added/Adding Score: ")).append(highScoresAsString.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("saveHighScore"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        jcall!(self.highScoresHelper).setHighScoresArray(self.highScoresArray.clone());
    }
    

    pub fn submit(&mut self, mut myCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>>) {
        let mut commandListener: Option<Rc<RefCell<CommandListener>>> = jcall!(myCanvas).getCustomCommandListener();
        jcall!(commandListener).commandAction(HighScoreUtil_SUBMIT_TEXTBOX_COMMAND.with(Clone::clone).clone(), myCanvas.clone());
    }
}


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
use crate::org::allbinary::thread::ARunnable::ARunnable;
use crate::org::allbinary::game::BasicHighScoresGameInfoFactory::BasicHighScoresGameInfoFactory;
use crate::org::allbinary::game::GameInfo::GameInfo;
use crate::org::allbinary::game::GameType::GameType;
use crate::org::allbinary::game::GameTypeFactory::GameTypeFactory;
use crate::org::allbinary::game::score::remote::RemoteHighScores::RemoteHighScores;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::logic::system::SoftwareInformation::SoftwareInformation;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::thread::SecondaryThreadPool::SecondaryThreadPool;
use super::HighScoresBase::HighScoresBase;
use super::HighScores::HighScores;
use super::HighScoresResultsListener::HighScoresResultsListener;
use super::ScoreComparator::ScoreComparator;
use super::HighScoresHelper2::HighScoresHelper2;
use super::HighScoresHelperBaseInterface::HighScoresHelperBaseInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicHighScoresFactory {
    pub base: HighScoresBase,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>,
    pub softwareInformation: Option<Rc<RefCell<SoftwareInformation>>>,
    pub highScoresArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>>,
    pub TOP: String,
    pub SCORES: String,
    pub PERSONAL_HIGH_SCORES: String,
    pub WORLD_TOP_SCORES: String,
    pub FETCH: String,
}


impl BasicHighScoresFactory {
    

    pub fn new_struct(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut softwareInformation: Option<Rc<RefCell<SoftwareInformation>>>) -> Self {
        let mut this = Self {
            base: HighScoresBase::new_struct(),
            logUtil: None,
            abeClientInformation: None,
            softwareInformation: None,
            highScoresArray: Vec::new(),
            TOP: String::new(),
            SCORES: String::new(),
            PERSONAL_HIGH_SCORES: String::new(),
            WORLD_TOP_SCORES: String::new(),
            FETCH: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.highScoresArray = vec![None; (2) as usize];
        this.TOP = String::from("Top");
        this.SCORES = String::from("Scores");
        this.PERSONAL_HIGH_SCORES = String::from("Personal Top Scores");
        this.WORLD_TOP_SCORES = String::from("World Top Scores");
        this.FETCH = String::from("fetchHighScores");
        this.abeClientInformation = abeClientInformation.clone();
        this.softwareInformation = softwareInformation.clone();
        this
    }

    pub fn new(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut softwareInformation: Option<Rc<RefCell<SoftwareInformation>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(abeClientInformation, softwareInformation))))
    }
    

    pub fn fetchHighScores(&mut self, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut highScoresResultsListener: Option<Rc<RefCell<dyn HighScoresResultsListener>>>) {
        jcall!(self.logUtil).putF(String::from("Getting Remote/Local HighScores"), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.FETCH.clone());
        self.fetchHighScoresPreload(gameInfo.clone(), highScoresResultsListener.clone(), true);
    }
    

    pub fn fetchHighScoresPreload(&mut self, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut highScoresResultsListener: Option<Rc<RefCell<dyn HighScoresResultsListener>>>, mut preload: bool) {
        jcall!(SecondaryThreadPool::getInstance()).runTask(ARunnable::new() /* Java anonymous class body: implement it as a named type
            

            pub fn run(&mut self) {
                let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    jcall!(logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.FETCH.clone());
                    self.highScoresArray[(0) as usize] = RecordStoreHighScores::RecordStoreHighScores::getInstance(self.abeClientInformation.clone(), gameInfo.clone(), self.TOP.clone(), self.PERSONAL_HIGH_SCORES.clone(), self.SCORES.clone(), ScoreComparator::new(true));
                    let mut gameType: Option<Rc<RefCell<GameType>>> = jcall!(gameInfo).getGameType();
                    let mut gameTypeFactory: Option<Rc<RefCell<GameTypeFactory>>> = GameTypeFactory::getInstance();
                    let mut gameInfo2: Option<Rc<RefCell<GameInfo>>> = gameInfo.clone();
                    if match (&gameType, &jcall!(gameTypeFactory).SINGLE_PLAYER) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&gameType, &jcall!(gameTypeFactory).BOT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                        gameInfo2 = jcall!(BasicHighScoresGameInfoFactory::getInstance()).SINGLE_PLAYER_SERVER.clone();
                    } else if match (&gameType, &jcall!(gameTypeFactory).MULTI_PLAYER) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                        gameInfo2 = jcall!(BasicHighScoresGameInfoFactory::getInstance()).MULTI_PLAYER_SERVER.clone();
                    }
                    self.highScoresArray[(1) as usize] = RemoteHighScores::getInstancePreload(self.abeClientInformation.clone(), self.softwareInformation.clone(), gameInfo2.clone(), self.WORLD_TOP_SCORES.clone(), self.SCORES.clone(), jcall!(BooleanFactory::getInstance()).FALSE.clone(), preload.clone());
                    jcall!(logUtil).putF(jcall!(self.commonStrings).END.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.FETCH.clone());
                    jcall!(LastFetchHighScoresFactory::LastFetchHighScoresFactory::getInstance()).highScoresArray = self.highScoresArray.clone();
                    jcall!(highScoresResultsListener).setHighScoresArray(self.highScoresArray.clone());
                })) {
                    Ok(_) => {}
                    Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                    jcall!(logUtil).put(jcall!(self.base.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.FETCH.clone(), crate::java_into_object(e.clone()));
                    }
                }
            }
*/);
    }
    

    pub fn createHighScoresHelper(&mut self) -> Option<Rc<RefCell<dyn HighScoresHelperBaseInterface>>> {
        return HighScoresHelper2::new().map(|reference| reference as Rc<RefCell<dyn HighScoresHelperBaseInterface>>);
    }
    

    pub fn loaded(mut index2: i32) -> bool {
        if index2 >= 0 {
            return true;
        }
        return false;
    }
}
impl std::ops::Deref for BasicHighScoresFactory {
    type Target = HighScoresBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicHighScoresFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::score::HighScoresBase::HighScoresBaseDyn for BasicHighScoresFactory {
    fn as_HighScoresBase(&self) -> &crate::org::allbinary::game::score::HighScoresBase::HighScoresBase { &self.base }
    fn as_HighScoresBase_mut(&mut self) -> &mut crate::org::allbinary::game::score::HighScoresBase::HighScoresBase { &mut self.base }
}
impl crate::org::allbinary::game::score::HighScoresFactoryInterface::HighScoresFactoryInterface for BasicHighScoresFactory {
    fn fetchHighScores(&mut self, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut highScoresResultsListener: Option<Rc<RefCell<dyn HighScoresResultsListener>>>) { Self::fetchHighScores(self, gameInfo, highScoresResultsListener) }
    fn fetchHighScoresPreload(&mut self, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut highScoresResultsListener: Option<Rc<RefCell<dyn HighScoresResultsListener>>>, mut preload: bool) { Self::fetchHighScoresPreload(self, gameInfo, highScoresResultsListener, preload) }
    fn createHighScoresHelper(&mut self) -> Option<Rc<RefCell<dyn HighScoresHelperBaseInterface>>> { Self::createHighScoresHelper(self) }
}



// Generated Code Do Not Modify
// package org.allbinary.game.score
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::GameInfo::GameInfo;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::system::SoftwareInformation::SoftwareInformation;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
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
    pub highScoresArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>>,
    pub TOP: String,
    pub SCORES: String,
    pub PERSONAL_HIGH_SCORES: String,
    pub FETCH: String,
}


impl BasicHighScoresFactory {
    

    pub fn new_struct(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut softwareInformation: Option<Rc<RefCell<SoftwareInformation>>>) -> Self {
        let mut this = Self {
            base: HighScoresBase::new_struct(),
            logUtil: None,
            abeClientInformation: None,
            highScoresArray: Vec::new(),
            TOP: String::new(),
            SCORES: String::new(),
            PERSONAL_HIGH_SCORES: String::new(),
            FETCH: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.highScoresArray = vec![None; (1) as usize];
        this.TOP = String::from("Top");
        this.SCORES = String::from("Scores");
        this.PERSONAL_HIGH_SCORES = String::from("Personal Top Scores");
        this.FETCH = String::from("fetchHighScores");
        this.abeClientInformation = abeClientInformation.clone();
        this
    }

    pub fn new(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut softwareInformation: Option<Rc<RefCell<SoftwareInformation>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(abeClientInformation, softwareInformation))))
    }
    

    pub fn fetchHighScores(&mut self, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut highScoresResultsListener: Option<Rc<RefCell<dyn HighScoresResultsListener>>>) {
        jcall!(self.logUtil).putF(String::from("Getting Local HighScores"), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.FETCH.clone());
        self.fetchHighScoresPreload(gameInfo.clone(), highScoresResultsListener.clone(), true);
    }
    

    pub fn fetchHighScoresPreload(&mut self, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut highScoresResultsListener: Option<Rc<RefCell<dyn HighScoresResultsListener>>>, mut preload: bool) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.highScoresArray[(0) as usize] = RecordStoreHighScores::RecordStoreHighScores::getInstance(self.abeClientInformation.clone(), gameInfo.clone(), self.TOP.clone(), self.PERSONAL_HIGH_SCORES.clone(), self.SCORES.clone(), ScoreComparator::new(true));
            jcall!(highScoresResultsListener).setHighScoresArray(self.highScoresArray.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.FETCH.clone(), crate::java_into_object(e.clone()));
            }
        }
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



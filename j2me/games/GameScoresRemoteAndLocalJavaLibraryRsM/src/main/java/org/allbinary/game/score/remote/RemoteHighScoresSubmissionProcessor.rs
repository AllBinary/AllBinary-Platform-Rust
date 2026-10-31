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
// package org.allbinary.game.score.remote
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::GameInfoData::GameInfoData;
use crate::org::allbinary::game::configuration::GameConfigurationCentral::GameConfigurationCentral;
use crate::org::allbinary::game::score::HighScore::HighScore;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::xmlrpc::XmlRpcAbeClient::XmlRpcAbeClient;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::logic::system::security::crypt::jcehelper::NoCrypt::NoCrypt;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::HashtableUtil::HashtableUtil;
use super::RemoteHighScoresSubmissionProcessorInterface::RemoteHighScoresSubmissionProcessorInterface;
use super::XmlRpcRemoteHighScoresClient::XmlRpcRemoteHighScoresClient;
use super::RemoteHighScores::RemoteHighScores;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RemoteHighScoresSubmissionProcessor {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub noCrypt: Option<Rc<RefCell<NoCrypt>>>,
}

impl RemoteHighScoresSubmissionProcessorInterface for RemoteHighScoresSubmissionProcessor {
    fn process(&mut self, mut remoteHighScores: Option<Rc<RefCell<RemoteHighScores>>>, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut highScore: Option<Rc<RefCell<HighScore>>>) { Self::process(self, remoteHighScores, abeClientInformation, highScore) }
}

impl RemoteHighScoresSubmissionProcessor {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            noCrypt: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.noCrypt = NoCrypt::new();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // synchronized
    pub fn process(&mut self, mut remoteHighScores: Option<Rc<RefCell<RemoteHighScores>>>, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut highScore: Option<Rc<RefCell<HighScore>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF(String::from("Begin Remote HighScores Submission"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).PROCESS.clone());
            let mut gameInfoData: Option<Rc<RefCell<GameInfoData>>> = GameInfoData::getInstance();
            let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = jcall!(abeClientInformation).toHashtable();
            jcall!(HashtableUtil::getInstance()).putAll(jcall!(jcall!(highScore).getGameInfo()).toHashtable(), hashtable.clone());
            jcall!(hashtable).put(crate::java_into_object(jcall!(RemoteHighScoresData::RemoteHighScoresData::getInstance()).CUSTOMER_USER_NAME.clone()), crate::java_into_object(String::from("None")));
            jcall!(hashtable).put(crate::java_into_object(jcall!(RemoteHighScoresData::RemoteHighScoresData::getInstance()).DISPLAY_NAME.clone()), crate::java_into_object(jcall!(highScore).getName()));
            jcall!(hashtable).put(crate::java_into_object(jcall!(gameInfoData).SOFTWARE_INFORMATION.clone()), crate::java_into_object(jcall!(jcall!(remoteHighScores).getSoftwareInformation()).toString()));
            jcall!(hashtable).put(crate::java_into_object(jcall!(remoteHighScores).ASCENDING.clone()), crate::java_into_object(jcall!(jcall!(remoteHighScores).getAscending()).toString()));
            let mut displayInfoSingleton: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
            jcall!(hashtable).put(crate::java_into_object(jcall!(displayInfoSingleton).ORIENTATION.clone()), crate::java_into_object(jcall!(BooleanFactory::getInstance()).toStringb(jcall!(displayInfoSingleton).isPortrait())));
            jcall!(hashtable).put(crate::java_into_object(jcall!(RemoteHighScoresData::RemoteHighScoresData::getInstance()).GAME_CONFIGURATION.clone()), crate::java_into_object(jcall!(GameConfigurationCentral::getInstance()).toString()));
            jcall!(hashtable).put(crate::java_into_object(jcall!(RemoteHighScoresData::RemoteHighScoresData::getInstance()).SCORE.clone()), crate::java_into_object((jcall!(highScore).getScore()).to_string()));
            if unsafe { crate::org::allbinary::logic::communication::xmlrpc::XmlRpcAbeClient::XmlRpcAbeClient_isOnline.clone() } {
                let mut resultHashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = (jcall!(XmlRpcRemoteHighScoresClient::new(abeClientInformation.clone(), String::from("highscoresubmissionservicessl.php"), String::from("HighScoreSubmissionService.process"))).get(hashtable.clone(), self.noCrypt.clone())).clone() /* as ABHashtable */;
                jcall!(remoteHighScores).update(resultHashtable.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).PROCESS.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}


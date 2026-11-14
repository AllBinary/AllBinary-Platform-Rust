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
use crate::java::lang::Exception::*;
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

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

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
            jcall!(self.logUtil).putF(String::from("Begin Remote HighScores Submission"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).PROCESS.clone(); java_field }.clone());
            let mut gameInfoData: Option<Rc<RefCell<GameInfoData>>> = GameInfoData::getInstance();
            let mut hashtable: Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>> = jcall!(abeClientInformation).toHashtable();
            jcall!(HashtableUtil::getInstance()).putAll(jcall!(jcall!(highScore).getGameInfo()).toHashtable(), hashtable.clone());
            jcall!(hashtable).put(crate::java_into_object({ let java_field = jcall!(RemoteHighScoresData::RemoteHighScoresData::getInstance()).CUSTOMER_USER_NAME.clone(); java_field }.clone()), crate::java_into_object(String::from("None")));
            jcall!(hashtable).put(crate::java_into_object({ let java_field = jcall!(RemoteHighScoresData::RemoteHighScoresData::getInstance()).DISPLAY_NAME.clone(); java_field }.clone()), crate::java_into_object(jcall!(highScore).getName()));
            jcall!(hashtable).put(crate::java_into_object({ let java_field = jcall!(gameInfoData).SOFTWARE_INFORMATION.clone(); java_field }.clone()), crate::java_into_object(jcall!(jcall!(remoteHighScores).getSoftwareInformation()).toString()));
            jcall!(hashtable).put(crate::java_into_object({ let java_field = jcall!(remoteHighScores).ASCENDING.clone(); java_field }.clone()), crate::java_into_object((jcall!(remoteHighScores).getAscending()).expect("null Boolean").to_string()));
            let mut displayInfoSingleton: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
            jcall!(hashtable).put(crate::java_into_object({ let java_field = jcall!(displayInfoSingleton).ORIENTATION.clone(); java_field }.clone()), crate::java_into_object(jcall!(BooleanFactory::getInstance()).toStringb(jcall!(displayInfoSingleton).isPortrait())));
            jcall!(hashtable).put(crate::java_into_object({ let java_field = jcall!(RemoteHighScoresData::RemoteHighScoresData::getInstance()).GAME_CONFIGURATION.clone(); java_field }.clone()), crate::java_into_object(jcall!(GameConfigurationCentral::getInstance()).toString()));
            jcall!(hashtable).put(crate::java_into_object({ let java_field = jcall!(RemoteHighScoresData::RemoteHighScoresData::getInstance()).SCORE.clone(); java_field }.clone()), crate::java_into_object((jcall!(highScore).getScore()).to_string()));
            if unsafe { crate::org::allbinary::logic::communication::xmlrpc::XmlRpcAbeClient::XmlRpcAbeClient_isOnline.clone() } {
                let mut resultHashtable: Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>> = { let java_object = (jcall!(XmlRpcRemoteHighScoresClient::new(abeClientInformation.clone(), String::from("highscoresubmissionservicessl.php"), String::from("HighScoreSubmissionService.process"))).get(crate::java_into_object(hashtable.clone()), self.noCrypt.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::init::crypt::jcehelper::CryptInterface::CryptInterface>>))).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>>>().expect("ClassCastException").clone()) };
                jcall!(remoteHighScores).update(resultHashtable.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).PROCESS.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}


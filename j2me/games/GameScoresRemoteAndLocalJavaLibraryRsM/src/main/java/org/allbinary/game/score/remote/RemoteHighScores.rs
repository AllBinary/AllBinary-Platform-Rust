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
use crate::Enumeration;
use super::*;
// use crate::java::util::Enumeration::Enumeration;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::game::GameInfo::GameInfo;
use crate::org::allbinary::game::score::HighScore::HighScore;
use crate::org::allbinary::game::score::HighScores::HighScores;
use crate::org::allbinary::game::score::RemoteErrorHighScoresSingletonFactory::RemoteErrorHighScoresSingletonFactory;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::SoftwareInformation::SoftwareInformation;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::EnumerationUtil::EnumerationUtil;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RemoteHighScores {
    pub base: HighScores,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub enumerationUtil: Option<Rc<RefCell<EnumerationUtil>>>,
    pub abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>,
    pub softwareInformation: Option<Rc<RefCell<SoftwareInformation>>>,
    pub ascending: Option<bool>,
    pub ASCENDING: String,
}

thread_local! { pub static RemoteHighScores_hashTable: Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashtable(); }

impl RemoteHighScores {
    

    pub fn new_struct(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut softwareInformation: Option<Rc<RefCell<SoftwareInformation>>>, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut heading: String, mut columnTwoHeading: String, mut ascending: Option<bool>, mut preload: bool) -> Self {
        let mut this = Self {
            base: HighScores::new_struct(jcall!(gameInfo).toString(), heading.clone(), columnTwoHeading.clone()),
            logUtil: None,
            enumerationUtil: None,
            abeClientInformation: None,
            softwareInformation: None,
            ascending: None,
            ASCENDING: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.enumerationUtil = EnumerationUtil::getInstance();
        this.ASCENDING = String::from("ASCENDING");
        // super(...) is in the struct literal above
        this.abeClientInformation = abeClientInformation.clone();
        this.softwareInformation = softwareInformation.clone();
        this.setAscending(ascending.clone());
        this
    }

    pub fn new(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut softwareInformation: Option<Rc<RefCell<SoftwareInformation>>>, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut heading: String, mut columnTwoHeading: String, mut ascending: Option<bool>, mut preload: bool) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(abeClientInformation.clone(), softwareInformation.clone(), gameInfo.clone(), heading.clone(), columnTwoHeading.clone(), ascending.clone(), preload.clone()))));
            if preload {
            jcall!(RemoteHighScoresProcessorFactory::RemoteHighScoresProcessorFactory::getInstance()).process(this.clone(), jcall!(this).abeClientInformation.clone(), gameInfo.clone());
        }
        this
    }
    

    // synchronized
    pub fn getInstance(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut softwareInformation: Option<Rc<RefCell<SoftwareInformation>>>, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut heading: String, mut columnTwoHeading: String, mut isAscending: Option<bool>) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>> {
        return RemoteHighScores::getInstancePreload(abeClientInformation.clone(), softwareInformation.clone(), gameInfo.clone(), heading.clone(), columnTwoHeading.clone(), isAscending.clone(), true);
    }
    

    // synchronized
    pub fn getInstancePreload(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut softwareInformation: Option<Rc<RefCell<SoftwareInformation>>>, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut heading: String, mut columnTwoHeading: String, mut isAscending: Option<bool>, mut preload: bool) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>> {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut highScores: Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>> = { let java_object = (jcall!(RemoteHighScores_hashTable.with(Clone::clone)).get(crate::java_into_object(gameInfo.clone()))).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>>().expect("ClassCastException").clone()) };
            if highScores.is_none() {
                highScores = RemoteHighScores::new(abeClientInformation.clone(), softwareInformation.clone(), gameInfo.clone(), heading.clone(), columnTwoHeading.clone(), isAscending.clone(), preload.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>);
                jcall!(RemoteHighScores_hashTable.with(Clone::clone)).put(crate::java_into_object(gameInfo.clone()), crate::java_into_object(highScores.clone()));
            }
            return Some(highScores.clone());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(RemoteErrorHighScoresSingletonFactory::getInstance()), { let java_field = jcall!(commonStrings).GET_INSTANCE.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            return RemoteErrorHighScoresSingletonFactory::getInstance();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn addHighScore(&mut self, mut newHighScore: Option<Rc<RefCell<HighScore>>>) {
        jcall!(RemoteHighScoresSubmissionProcessorFactory::RemoteHighScoresSubmissionProcessorFactory::getInstance()).process(None /* Java this: a &mut self method has no Rc of itself */, self.abeClientInformation.clone(), newHighScore.clone());
    }
    

    pub fn update(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>>) {
        jcall!(self.getList()).clear();
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_object = (jcall!(hashtable).get(crate::java_into_object(crate::java_into_object(({ let java_field = jcall!(RemoteHighScoresData::RemoteHighScoresData::getInstance()).HIGH_SCORES.clone(); java_field }).clone())))).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
        if vector.is_some() {
            let mut size: i32 = jcall!(vector).size();
            for index in (0..size) {
                let mut highScoreVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_object = (jcall!(vector).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
                let mut displayName: String = { let java_object = (jcall!(highScoreVector).get(0)).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
                let mut score: String = { let java_object = (jcall!(highScoreVector).get(1)).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
                let mut longScore: i64 = score.clone().parse::<i64>().unwrap();
                let mut highScore: Option<Rc<RefCell<HighScore>>> = HighScore::new(-1, displayName.clone(), None, longScore.clone());
                jcall!(self.getList()).add(crate::java_into_object(highScore.clone()));
            }
        } else {
            let mut NEXT_ELEMENT: String = jcall!(StringMaker::new()).append({ let java_field = jcall!(CommonLabels::getInstance()).NEXT.clone(); java_field }.clone()).append(String::from("Element: ")).toString();
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            let mut enumeration: Option<Rc<RefCell<dyn Enumeration>>> = jcall!(hashtable).elements();
            let mut nextElement: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = None;
            while jcall!(self.enumerationUtil).hasMoreElements(enumeration.clone()) {
                nextElement = { let java_value = (jcall!(self.enumerationUtil).nextElement(enumeration.clone())).clone(); java_value };
                jcall!(self.logUtil).putF(format!("{}{}", NEXT_ELEMENT, crate::java::lang::Object::java_object_string(&nextElement)), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).PROCESS.clone(); java_field }.clone());
            }
        }
    }
    

    pub fn setAscending(&mut self, mut ascending: Option<bool>) {
        self.ascending = ascending.clone();
    }
    

    pub fn getAscending(&mut self) -> Option<bool> {
        return self.ascending.clone();
    }
    

    pub fn getSoftwareInformation(&mut self) -> Option<Rc<RefCell<SoftwareInformation>>> {
        return self.softwareInformation.clone();
    }
}
impl std::ops::Deref for RemoteHighScores {
    type Target = HighScores;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RemoteHighScores {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::score::HighScores::HighScoresDyn for RemoteHighScores {
    fn as_HighScores(&self) -> &crate::org::allbinary::game::score::HighScores::HighScores { &self.base }
    fn as_HighScores_mut(&mut self) -> &mut crate::org::allbinary::game::score::HighScores::HighScores { &mut self.base }
}
impl crate::org::allbinary::game::score::HighScoresInterface::HighScoresInterface for RemoteHighScores {
    fn getList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { crate::org::allbinary::game::score::HighScores::HighScores::getList(crate::org::allbinary::game::score::HighScores::HighScoresDyn::as_HighScores_mut(self)) }
    fn addHighScore(&mut self, mut newHighScore: Option<Rc<RefCell<HighScore>>>) { Self::addHighScore(self, newHighScore) }
    fn getTotal(&mut self) -> i32 { crate::org::allbinary::game::score::HighScores::HighScores::getTotal(crate::org::allbinary::game::score::HighScores::HighScoresDyn::as_HighScores_mut(self)) }
    fn isBestScore(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::game::score::HighScore::HighScore>>>) -> bool { crate::org::allbinary::game::score::HighScores::HighScores::isBestScore(crate::org::allbinary::game::score::HighScores::HighScoresDyn::as_HighScores_mut(self), p1) }
}



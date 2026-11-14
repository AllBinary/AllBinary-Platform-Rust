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
// use crate::java::io::ByteArrayInputStream::ByteArrayInputStream;
use crate::java::io::DataInputStream::DataInputStream;
// use crate::java::io::EOFException::EOFException;
use crate::java::io::IOException::IOException;
use crate::javax::microedition::rms::RecordComparator::RecordComparator;
use crate::javax::microedition::rms::RecordEnumeration::RecordEnumeration;
use crate::javax::microedition::rms::RecordStore::RecordStore;
use crate::javax::microedition::rms::RecordStoreException::RecordStoreException;
use crate::javax::microedition::rms::RecordStoreNotFoundException::RecordStoreNotFoundException;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::game::GameInfo::GameInfo;
use crate::org::allbinary::game::configuration::persistance::NullRecordComparator::NullRecordComparator;
use crate::org::allbinary::game::configuration::persistance::NullRecordFilter::NullRecordFilter;
use crate::org::allbinary::game::configuration::persistance::NullRecordStore::NullRecordStore;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::persistance::PlatformRecordIdUtil::PlatformRecordIdUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::java::lang::Exception::*;
use super::HighScores::HighScores;
use super::HighScore::HighScore;
use super::ScoreComparator::ScoreComparator;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RecordStoreHighScores {
    pub base: HighScores,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub platformRecordIdUtil: Option<Rc<RefCell<PlatformRecordIdUtil>>>,
    pub tsUtil: Option<Rc<RefCell<TsUtil>>>,
    pub RECORD_ID: String,
    pub MAXHIGHSCORES: i32,
    pub gameInfo: Option<Rc<RefCell<GameInfo>>>,
    pub abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>,
    pub recordComparatorInterface: Option<Rc<RefCell<dyn RecordComparator>>>,
}

thread_local! { pub static RecordStoreHighScores_hashTable: Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashtable(); }

impl RecordStoreHighScores {
    

    pub fn new_struct(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut name: String, mut heading: String, mut columnTwoHeading: String, mut recordComparatorInterface: Option<Rc<RefCell<dyn RecordComparator>>>) -> Self {
        let mut this = Self {
            base: HighScores::new_struct(name.clone(), heading.clone(), columnTwoHeading.clone()),
            logUtil: None,
            commonStrings: None,
            platformRecordIdUtil: None,
            tsUtil: None,
            RECORD_ID: String::new(),
            MAXHIGHSCORES: 0,
            gameInfo: None,
            abeClientInformation: None,
            recordComparatorInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.platformRecordIdUtil = PlatformRecordIdUtil::getInstance();
        this.tsUtil = TsUtil::getInstance();
        this.RECORD_ID = String::from("_HS");
        this.MAXHIGHSCORES = 100;
        // super(...) is in the struct literal above
        this.abeClientInformation = abeClientInformation.clone();
        this.gameInfo = gameInfo.clone();
        this.recordComparatorInterface = recordComparatorInterface.clone();
        this.load();
        this
    }

    pub fn new(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut name: String, mut heading: String, mut columnTwoHeading: String, mut recordComparatorInterface: Option<Rc<RefCell<dyn RecordComparator>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(abeClientInformation, gameInfo, name, heading, columnTwoHeading, recordComparatorInterface))))
    }
    

    // synchronized
    pub fn getInstance(mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut gameInfo: Option<Rc<RefCell<GameInfo>>>, mut highScoreName: String, mut heading: String, mut columnTwoHeading: String, mut recordComparatorInterface: Option<Rc<RefCell<dyn RecordComparator>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>> {
        let mut highScoresCanBeNull: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(RecordStoreHighScores_hashTable.with(Clone::clone)).get(crate::java_into_object(crate::java_into_object((highScoreName).clone())));
        if highScoresCanBeNull.is_none() {
            let mut highScores: Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>> = RecordStoreHighScores::new(abeClientInformation.clone(), gameInfo.clone(), highScoreName.clone(), heading.clone(), columnTwoHeading.clone(), recordComparatorInterface.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>);
            jcall!(RecordStoreHighScores_hashTable.with(Clone::clone)).put(crate::java_into_object(jcall!(highScores).getName()), crate::java_into_object(highScores.clone()));
            return highScores.clone();
        }
        return { let java_object = (highScoresCanBeNull).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn getRecordId(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>) -> String {
        return { let java_receiver = self.platformRecordIdUtil.clone(); let java_arguments = (abeClientInformation.clone(), { let java_argument = jcall!(StringMaker::new()).append({ let java_field = jcall!(CommonSeps::getInstance()).UNDERSCORE.clone(); java_field }.clone()).append(self.getName()).append(self.RECORD_ID.clone()).toString(); java_argument },); let java_result = jcall!(java_receiver).getRecordId(java_arguments.0, java_arguments.1); java_result };
    }
    

    // synchronized
    pub fn addHighScore(&mut self, mut newHighScore: Option<Rc<RefCell<HighScore>>>) {
        let mut recordStore: Option<Rc<RefCell<dyn crate::javax::microedition::rms::RecordStore::RecordStoreDyn>>> = unsafe { crate::org::allbinary::game::configuration::persistance::NullRecordStore::NullRecordStore_NULL_RECORD_STORE.clone() }.clone().map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::rms::RecordStore::RecordStoreDyn>>);
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Adding HighScore: ")).appendlong(jcall!(newHighScore).getScore()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).ADD.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            if self.isTooManyHighScores() {
                jcall!(self.logUtil).putF(String::from("Removing Lowest Score"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).ADD.clone(); java_field }.clone());
                self.removeLowestHighScore();
            }
            recordStore = { let java_value = (RecordStore::openRecordStore({ let java_arguments = (self.abeClientInformation.clone(),); self.getRecordId(java_arguments.0) }, true)).clone(); java_value };
            let mut highScoreBytes: Vec<i8> = jcall!(newHighScore).getAsBytes();
            let mut recordId: i32 = jcall!(recordStore).addRecord(highScoreBytes.clone(), 0, (highScoreBytes.len() as i32).clone());
            self.load();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).ADD.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
        { // finally
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                if recordStore.is_some() {
                    PreLogUtil::put(String::from("Closing RecordStore"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).ADD.clone(); java_field }.clone());
                    jcall!(recordStore).closeRecordStore();
                }
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).ADD.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
                }
            }
        }
    }
    

    pub fn removeLowestHighScore(&mut self) {
        let mut recordStore: Option<Rc<RefCell<dyn crate::javax::microedition::rms::RecordStore::RecordStoreDyn>>> = unsafe { crate::org::allbinary::game::configuration::persistance::NullRecordStore::NullRecordStore_NULL_RECORD_STORE.clone() }.clone().map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::rms::RecordStore::RecordStoreDyn>>);
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            recordStore = { let java_value = (RecordStore::openRecordStore({ let java_arguments = (self.abeClientInformation.clone(),); self.getRecordId(java_arguments.0) }, true)).clone(); java_value };
            let mut recordEnum: Option<Rc<RefCell<dyn RecordEnumeration>>> = jcall!(recordStore).enumerateRecords(unsafe { crate::org::allbinary::game::configuration::persistance::NullRecordFilter::NullRecordFilter_NULL_RECORD_FILTER.clone() }.clone().map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::rms::RecordFilter::RecordFilter>>), crate::org::allbinary::game::configuration::persistance::NullRecordComparator::NullRecordComparator_NULL_RECORD_COMPARATOR.with(Clone::clone).clone().map(|reference| reference as Rc<RefCell<dyn RecordComparator>>), true);
            let mut scoreComparator: Option<Rc<RefCell<ScoreComparator>>> = ((self.recordComparatorInterface).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<ScoreComparator>() }, "ClassCastException: not a ScoreComparator"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<ScoreComparator>) } }));
            let mut bestHighScore: Option<Rc<RefCell<HighScore>>> = HighScore::new(-1, String::from("none"), crate::org::allbinary::game::GameInfo::GameInfo_NONE.with(Clone::clone).clone(), jcall!(scoreComparator).getBestScore());
            let mut recordAsBytes: Vec<i8> = Vec::new();
            let mut byteArrayInputStream: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>> = None;
            let mut inputStream: Option<Rc<RefCell<DataInputStream>>> = None;
            while jcall!(recordEnum).hasNextElement() {
                let mut id: i32 = jcall!(recordEnum).nextRecordId();
                recordAsBytes = { let java_value = (jcall!(self.tsUtil).getRecord(crate::java_into_object(recordStore.clone()), id.clone())).clone(); java_value };
                if !recordAsBytes.is_empty() {
                    byteArrayInputStream = crate::java::io::InputStream::InputStream::new1(recordAsBytes.clone()).map(|reference| reference as Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>);
                    inputStream = DataInputStream::new(byteArrayInputStream.clone());
                    let mut name: String = jcall!(inputStream).readUTF();
                    let mut nextScore: i64 = jcall!(inputStream).readLong();
                    let mut nextCurrentHighScore: Option<Rc<RefCell<HighScore>>> = HighScore::new(id.clone(), name.clone(), crate::org::allbinary::game::GameInfo::GameInfo_NONE.with(Clone::clone).clone(), nextScore.clone());
                    if { let java_receiver = self.recordComparatorInterface.clone(); let java_arguments = (jcall!(nextCurrentHighScore).getAsBytes(), jcall!(bestHighScore).getAsBytes(),); let java_result = jcall!(java_receiver).compare(java_arguments.0, java_arguments.1); java_result } == crate::javax::microedition::rms::RecordComparator::RecordComparator_FOLLOWS.with(Clone::clone) {
                        bestHighScore = nextCurrentHighScore.clone();
                    }
                }
            }
            if jcall!(bestHighScore).getId() != -1 {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Removing Lowest HighScore: ")).appendlong(jcall!(bestHighScore).getScore()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).LOAD.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                jcall!(recordStore).deleteRecord(jcall!(bestHighScore).getId());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("removeLowestHighScore"), crate::java_into_object(e.clone()));
            }
        }
        { // finally
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                if recordStore.is_some() {
                    PreLogUtil::put(String::from("Closing RecordStore"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("removeLowestHighScore"));
                    jcall!(recordStore).closeRecordStore();
                }
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("removeLowestHighScore"), crate::java_into_object(e.clone()));
                }
            }
        }
    }
    

    pub fn load(&mut self) {
        let mut recordStore: Option<Rc<RefCell<dyn crate::javax::microedition::rms::RecordStore::RecordStoreDyn>>> = unsafe { crate::org::allbinary::game::configuration::persistance::NullRecordStore::NullRecordStore_NULL_RECORD_STORE.clone() }.clone().map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::rms::RecordStore::RecordStoreDyn>>);
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            recordStore = { let java_value = (RecordStore::openRecordStore({ let java_arguments = (self.abeClientInformation.clone(),); self.getRecordId(java_arguments.0) }, true)).clone(); java_value };
            self.setList(BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
            let mut recordEnum: Option<Rc<RefCell<dyn RecordEnumeration>>> = jcall!(recordStore).enumerateRecords(unsafe { crate::org::allbinary::game::configuration::persistance::NullRecordFilter::NullRecordFilter_NULL_RECORD_FILTER.clone() }.clone().map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::rms::RecordFilter::RecordFilter>>), crate::org::allbinary::game::configuration::persistance::NullRecordComparator::NullRecordComparator_NULL_RECORD_COMPARATOR.with(Clone::clone).clone().map(|reference| reference as Rc<RefCell<dyn RecordComparator>>), true);
            let mut recordAsBytes: Vec<i8> = Vec::new();
            let mut byteArrayInputStream: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>> = None;
            let mut inputStream: Option<Rc<RefCell<DataInputStream>>> = None;
            while jcall!(recordEnum).hasNextElement() {
                let mut id: i32 = jcall!(recordEnum).nextRecordId();
                recordAsBytes = { let java_value = (jcall!(self.tsUtil).getRecord(crate::java_into_object(recordStore.clone()), id.clone())).clone(); java_value };
                if !recordAsBytes.is_empty() {
                    byteArrayInputStream = crate::java::io::InputStream::InputStream::new1(recordAsBytes.clone()).map(|reference| reference as Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>);
                    inputStream = DataInputStream::new(byteArrayInputStream.clone());
                    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let mut name: String = jcall!(inputStream).readUTF();
                        let mut score: i64 = jcall!(inputStream).readLong();
                        let mut newHighScore: Option<Rc<RefCell<HighScore>>> = HighScore::new(id.clone(), name.clone(), crate::org::allbinary::game::GameInfo::GameInfo_NONE.with(Clone::clone).clone(), score.clone());
                        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getList();
                        let mut size: i32 = jcall!(list).size();
                        let mut lastIndex: i32 = size.clone();
                        for index in (0..size) {
                            let mut highScore: Option<Rc<RefCell<HighScore>>> = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<HighScore>>>>().expect("ClassCastException").clone()) };
                            if { let java_receiver = self.recordComparatorInterface.clone(); let java_arguments = (jcall!(newHighScore).getAsBytes(), jcall!(highScore).getAsBytes(),); let java_result = jcall!(java_receiver).compare(java_arguments.0, java_arguments.1); java_result } == crate::javax::microedition::rms::RecordComparator::RecordComparator_PRECEDES.with(Clone::clone) {
                                lastIndex = index.clone();
                                break;
                            }
                        }
                        jcall!(list).addAt(lastIndex.clone(), crate::java_into_object(newHighScore.clone()));
                    })) {
                        Ok(_) => {}
                        Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

                        jcall!(self.logUtil).put(String::from("EOF"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).LOAD.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
                        panic!("{:?}", e);
                        }
                    }
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put(String::from("No High Scores"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).LOAD.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
        { // finally
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                if recordStore.is_some() {
                    PreLogUtil::put(String::from("Closing RecordStore"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).LOAD.clone(); java_field }.clone());
                    jcall!(recordStore).closeRecordStore();
                }
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).LOAD.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
                }
            }
        }
    }
    

    pub fn isTooManyHighScores(&mut self) -> bool {
        if self.getList().is_some() && { let java_value = jcall!(self.getList()).size(); java_value } < self.MAXHIGHSCORES {
            return false;
        } else {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("HighScores RecordStore Max Reached: ")).appendint(self.MAXHIGHSCORES.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isTooManyHighScores"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            return true;
        }
    }
    

    // throws Exception
    // synchronized
    pub fn isBestScore(&mut self, mut newHighScore: Option<Rc<RefCell<HighScore>>>) -> bool {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if !self.isTooManyHighScores() {
                jcall!(self.logUtil).putF(String::from("Slot Available for a High Score"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isBestScore"));
                return Some(true);
            } else {
                let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getList();
                let mut size: i32 = jcall!(list).size();
                for index in (0..size) {
                    let mut highScore: Option<Rc<RefCell<HighScore>>> = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<HighScore>>>>().expect("ClassCastException").clone()) };
                    if { let java_receiver = self.recordComparatorInterface.clone(); let java_arguments = (jcall!(newHighScore).getAsBytes(), jcall!(highScore).getAsBytes(),); let java_result = jcall!(java_receiver).compare(java_arguments.0, java_arguments.1); java_result } == crate::javax::microedition::rms::RecordComparator::RecordComparator_FOLLOWS.with(Clone::clone) {
                        jcall!(self.logUtil).putF(String::from("Obtained a High Score"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isBestScore"));
                        return Some(true);
                    }
                }
            }
            jcall!(self.logUtil).putF(String::from("Not a High Score"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("isBestScore"));
            return Some(false);
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).ADD.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            panic!("{:?}", e);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(self.base.toString());
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getList();
        let mut size: i32 = jcall!(list).size();
        for index in (0..size) {
            let mut highScore: Option<Rc<RefCell<HighScore>>> = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<HighScore>>>>().expect("ClassCastException").clone()) };
            jcall!(stringBuffer).append(jcall!(highScore).getScoreString());
            jcall!(stringBuffer).append({ let java_field = jcall!(CommonSeps::getInstance()).COMMA_SEP.clone(); java_field }.clone());
        }
        return jcall!(stringBuffer).toString();
    }
}
impl std::ops::Deref for RecordStoreHighScores {
    type Target = HighScores;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RecordStoreHighScores {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::score::HighScores::HighScoresDyn for RecordStoreHighScores {
    fn as_HighScores(&self) -> &crate::org::allbinary::game::score::HighScores::HighScores { &self.base }
    fn as_HighScores_mut(&mut self) -> &mut crate::org::allbinary::game::score::HighScores::HighScores { &mut self.base }
}
impl crate::org::allbinary::game::score::HighScoresInterface::HighScoresInterface for RecordStoreHighScores {
    fn getList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { crate::org::allbinary::game::score::HighScores::HighScores::getList(crate::org::allbinary::game::score::HighScores::HighScoresDyn::as_HighScores_mut(self)) }
    fn addHighScore(&mut self, mut newHighScore: Option<Rc<RefCell<HighScore>>>) { Self::addHighScore(self, newHighScore) }
    fn getTotal(&mut self) -> i32 { crate::org::allbinary::game::score::HighScores::HighScores::getTotal(crate::org::allbinary::game::score::HighScores::HighScoresDyn::as_HighScores_mut(self)) }
    fn isBestScore(&mut self, mut newHighScore: Option<Rc<RefCell<HighScore>>>) -> bool { Self::isBestScore(self, newHighScore) }
}



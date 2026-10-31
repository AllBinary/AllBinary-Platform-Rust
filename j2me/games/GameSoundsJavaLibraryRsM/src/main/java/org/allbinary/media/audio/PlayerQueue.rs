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
// package org.allbinary.media.audio
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::Sound::Sound;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PlayerQueue {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub max: i32,
}

thread_local! { pub static PlayerQueue_TOTAL: String = String::from("Sounds In Queue: "); }

impl PlayerQueue {
    

    pub fn new_struct(mut max: i32) -> Self {
        let mut this = Self {
            commonStrings: None,
            list: None,
            max: 0,
        };
        this.commonStrings = CommonStrings::getInstance();
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.max = max.clone();
        this
    }

    pub fn new(mut max: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(max))))
    }
    

    pub fn add(&mut self, mut sound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if !jcall!(self.list).contains(crate::java_into_object(sound.clone())) {
                jcall!(self.list).add(crate::java_into_object(sound.clone()));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            PreLogUtil::put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).ADD.clone());
            }
        }
    }
    

    pub fn process(&mut self) -> bool {
        let mut sound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>> = NoSound::NoSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>);
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut played: bool = false;
            if jcall!(self.list).size() > 0 {
                sound = { let java_object = (jcall!(self.list).removeAt(0)).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>>().expect("ClassCastException").clone()) };
                if sound.is_some() {
                    jcall!(jcall!(sound).getPlayerP()).start();
                }
                while jcall!(self.list).size() > self.max {
                    jcall!(self.list).removeAt(0);
                }
                played = true;
            }
            return played.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut resource: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
            if sound.is_some() {
                resource = jcall!(sound).getResource();
            }
            PreLogUtil::put(jcall!(StringMaker::new()).append(jcall!(self.commonStrings).EXCEPTION_LABEL.clone()).append(resource.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).PROCESS.clone());
            return false;
            }
        }
    }
    

    pub fn clear(&mut self) {
        jcall!(self.list).clear();
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(StringMaker::new()).append(PlayerQueue_TOTAL.with(Clone::clone).clone()).appendint(jcall!(self.list).size()).toString();
    }
}


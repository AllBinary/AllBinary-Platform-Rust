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
use crate::android::media::MediaPlayer::MediaPlayer;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::java::lang::Thread::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MediaPlayerUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static MediaPlayerUtil_instance: Option<Rc<RefCell<MediaPlayerUtil>>> = MediaPlayerUtil::new(); }

impl MediaPlayerUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<MediaPlayerUtil>>> {
        return MediaPlayerUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn waitForMediaPlayer(&mut self, mut mediaPlayer: Option<Rc<RefCell<MediaPlayer>>>) {
        let mut MESSAGE: String = String::from("Not Stopped Waiting");
        let mut METHOD_NAME: String = String::from("wait");
        let mut index: i32 = 0;
        while jcall!(mediaPlayer).isPlaying() && index < 50 {
            jcall!(self.logUtil).putF(MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone());
            std::thread::sleep(std::time::Duration::from_millis((100) as u64));
            index += 1;
        }
    }
}


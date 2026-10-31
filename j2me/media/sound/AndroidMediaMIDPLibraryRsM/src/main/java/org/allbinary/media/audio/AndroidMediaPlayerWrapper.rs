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
// use crate::javax::microedition::media::MediaException::MediaException;
// use crate::javax::microedition::media::Player::Player;
// use crate::javax::microedition::media::PlayerListener::PlayerListener;
use crate::android::media::MediaPlayer::MediaPlayer;
use crate::org::allbinary::android::NullAndroidCanvas::NullAndroidCanvas;
use crate::org::allbinary::data::resource::ResourceUtil::ResourceUtil;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::BasicPlayer::BasicPlayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AndroidMediaPlayerWrapper {
    pub base: BasicPlayer,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub mediaPlayer: Option<Rc<RefCell<MediaPlayer>>>,
}

thread_local! { pub static AndroidMediaPlayerWrapper_NULL_ANDROID_MEDIA_PLAYER_WRAPPER: Option<Rc<RefCell<AndroidMediaPlayerWrapper>>> = AndroidMediaPlayerWrapper::createPlayerWrapper(); }

impl AndroidMediaPlayerWrapper {
    

    pub fn createPlayerWrapper() -> Option<Rc<RefCell<AndroidMediaPlayerWrapper>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return AndroidMediaPlayerWrapper::new(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone());
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
            }
        }
    }
    

    pub fn new_struct(mut resource: String) -> Self {
        let mut this = Self {
            base: BasicPlayer::new_struct(),
            logUtil: None,
            mediaPlayer: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.mediaPlayer = crate::org::allbinary::android::NullAndroidCanvas::NullAndroidCanvas_NULL_MEDIA_PLAYER.with(Clone::clone).clone();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if resource == jcall!(StringUtil::getInstance()).EMPTY_STRING {
            } else {
                let mut resourceUtil: Option<Rc<RefCell<ResourceUtil>>> = ResourceUtil::getInstance();
                let mut mediaPlayer: Option<Rc<RefCell<MediaPlayer>>> = MediaPlayer::create(jcall!(resourceUtil).getContext(), ((jcall!(resourceUtil).getResourceId(resource.clone())).unwrap() as i32));
                if mediaPlayer.is_none() {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(StringMaker::new()).append(String::from("Failed to create media player for: ")).append(resource.clone()).append(String::from(" with id: ")).append(jcall!(jcall!(resourceUtil).getResourceId(resource.clone())).toString()).toString())));
                }
                this.setMediaPlayer(mediaPlayer.clone());
                jcall!(this.mediaPlayer).setLooping(false);
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(this.logUtil).put(format!("{}{}", jcall!(this.commonStrings).EXCEPTION_LABEL, resource), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(e.clone()));
            panic!("{:?}", e);
            }
        }
        this
    }

    pub fn new(mut resource: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(resource))))
    }
    

    pub fn setLoopCount(&mut self, mut count: i32) {
        self.base.setLoopCount(count.clone());
        if !match (&self.mediaPlayer, &crate::org::allbinary::android::NullAndroidCanvas::NullAndroidCanvas_NULL_MEDIA_PLAYER.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && self.mediaPlayer.is_some() {
            if count == 0 {
                jcall!(self.mediaPlayer).setLooping(false);
            } else {
            }
        }
    }
    

    // synchronized
    pub fn addPlayerListener(&mut self, mut playerListener: Option<Rc<RefCell<PlayerListener>>>) {
        self.base.addPlayerListener(playerListener.clone());
    }
    

    pub fn removePlayerListener(&mut self, mut playerListener: Option<Rc<RefCell<PlayerListener>>>) {
        self.base.removePlayerListener(playerListener.clone());
    }
    

    pub fn getState(&mut self) -> i32 {
        if jcall!(self.mediaPlayer).isPlaying() {
            return unsafe { crate::javax::microedition::media::Player::Player_STARTED.clone() }.clone();
        } else {
            return unsafe { crate::javax::microedition::media::Player::Player_PREFETCHED.clone() }.clone();
        }
    }
    

    pub fn close(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.mediaPlayer).release();
            self.mediaPlayer = crate::org::allbinary::android::NullAndroidCanvas::NullAndroidCanvas_NULL_MEDIA_PLAYER.with(Clone::clone).clone();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CLOSE.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws MediaException
    // synchronized
    pub fn start(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(self.mediaPlayer).isPlaying() {
                jcall!(self.mediaPlayer).pause();
                jcall!(self.mediaPlayer).seekTo(0);
            }
            jcall!(self.mediaPlayer).start();
            self.base.start();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).START_METHOD_NAME.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws MediaException
    // synchronized
    pub fn stop(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.mediaPlayer).stop();
            jcall!(self.mediaPlayer).prepare();
            self.base.stop();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("stop"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn update(&mut self, mut event: String) {
        jcall!(self.logUtil).putF(format!("{}{}", String::from("LineEvent: "), event), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).UPDATE.clone());
        let mut size: i32 = jcall!(self.listenersList).size();
        for index in (0..size) {
            let mut listener: Option<Rc<RefCell<PlayerListener>>> = (jcall!(self.listenersList).as_BasicArrayList_mut().objectArray[(index) as usize]).clone() /* as PlayerListener */;
            jcall!(listener).playerUpdate(None /* Java this: a &mut self method has no Rc of itself */, event.clone(), jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());
        }
    }
    

    pub fn setVolume(&mut self, mut leftVolume: i32, mut rightVolume: i32) {
        jcall!(self.mediaPlayer).setVolume((((leftVolume) as f32)) / 100.0_f32, (((rightVolume) as f32)) / 100.0_f32);
    }
    

    pub fn getDuration(&mut self) -> i64 {
        return ((jcall!(self.mediaPlayer).getDuration()) as i64);
    }
    

    pub fn setMediaPlayer(&mut self, mut mediaPlayer: Option<Rc<RefCell<MediaPlayer>>>) {
        self.mediaPlayer = mediaPlayer.clone();
    }
    

    pub fn getMediaPlayer(&mut self) -> Option<Rc<RefCell<MediaPlayer>>> {
        return self.mediaPlayer.clone();
    }
}
impl std::ops::Deref for AndroidMediaPlayerWrapper {
    type Target = BasicPlayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AndroidMediaPlayerWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::audio::BasicPlayer::BasicPlayerDyn for AndroidMediaPlayerWrapper {
    fn as_BasicPlayer(&self) -> &crate::org::allbinary::media::audio::BasicPlayer::BasicPlayer { &self.base }
    fn as_BasicPlayer_mut(&mut self) -> &mut crate::org::allbinary::media::audio::BasicPlayer::BasicPlayer { &mut self.base }
    fn close(&mut self) { Self::close(self) }
    fn addPlayerListener(&mut self, mut playerListener: Option<Rc<RefCell<PlayerListener>>>) { Self::addPlayerListener(self, playerListener) }
    fn removePlayerListener(&mut self, mut playerListener: Option<Rc<RefCell<PlayerListener>>>) { Self::removePlayerListener(self, playerListener) }
    fn getState(&mut self) -> i32 { Self::getState(self) }
    fn getDuration(&mut self) -> i64 { Self::getDuration(self) }
    fn setLoopCount(&mut self, mut count: i32) { Self::setLoopCount(self, count) }
    fn start(&mut self) { Self::start(self) }
    fn stop(&mut self) { Self::stop(self) }
}
impl crate::org::allbinary::media::audio::Controllable2::Controllable2 for AndroidMediaPlayerWrapper {
    fn setVolume(&mut self, mut leftVolume: i32, mut rightVolume: i32) { Self::setVolume(self, leftVolume, rightVolume) }
}



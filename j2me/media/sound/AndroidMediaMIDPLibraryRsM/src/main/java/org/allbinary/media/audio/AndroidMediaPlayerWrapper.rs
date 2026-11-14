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
use crate::javax::microedition::media::MediaException::MediaException;
use crate::javax::microedition::media::Player::Player;
use crate::javax::microedition::media::PlayerListener::PlayerListener;
use crate::android::media::MediaPlayer::MediaPlayer;
use crate::org::allbinary::android::NullAndroidCanvas::NullAndroidCanvas;
use crate::org::allbinary::data::resource::ResourceUtil::ResourceUtil;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::java::lang::Exception::*;
use crate::java::lang::RuntimeException::*;
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
impl crate::javax::microedition::media::Controllable::Controllable for AndroidMediaPlayerWrapper {
    fn getControls(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::javax::microedition::media::Control::Control>>>> { crate::org::allbinary::media::audio::BasicPlayer::BasicPlayer::getControls(crate::org::allbinary::media::audio::BasicPlayer::BasicPlayerDyn::as_BasicPlayer_mut(self)) }
    fn getControl(&mut self, p1: String) -> Option<Rc<RefCell<dyn crate::javax::microedition::media::Control::Control>>> { crate::org::allbinary::media::audio::BasicPlayer::BasicPlayer::getControl(crate::org::allbinary::media::audio::BasicPlayer::BasicPlayerDyn::as_BasicPlayer_mut(self), p1) }
}
impl Player for AndroidMediaPlayerWrapper {
    fn realize(&mut self) { crate::org::allbinary::media::audio::BasicPlayer::BasicPlayer::realize(crate::org::allbinary::media::audio::BasicPlayer::BasicPlayerDyn::as_BasicPlayer_mut(self)) }
    fn prefetch(&mut self) { crate::org::allbinary::media::audio::BasicPlayer::BasicPlayer::prefetch(crate::org::allbinary::media::audio::BasicPlayer::BasicPlayerDyn::as_BasicPlayer_mut(self)) }
    fn start(&mut self) { Self::start(self) }
    fn stop(&mut self) { Self::stop(self) }
    fn deallocate(&mut self) { crate::org::allbinary::media::audio::BasicPlayer::BasicPlayer::deallocate(crate::org::allbinary::media::audio::BasicPlayer::BasicPlayerDyn::as_BasicPlayer_mut(self)) }
    fn close(&mut self) { Self::close(self) }
    fn setMediaTime(&mut self, p1: i64) -> i64 { crate::org::allbinary::media::audio::BasicPlayer::BasicPlayer::setMediaTime(crate::org::allbinary::media::audio::BasicPlayer::BasicPlayerDyn::as_BasicPlayer_mut(self), p1) }
    fn getMediaTime(&mut self) -> i64 { crate::org::allbinary::media::audio::BasicPlayer::BasicPlayer::getMediaTime(crate::org::allbinary::media::audio::BasicPlayer::BasicPlayerDyn::as_BasicPlayer_mut(self)) }
    fn getState(&mut self) -> i32 { Self::getState(self) }
    fn getDuration(&mut self) -> i64 { Self::getDuration(self) }
    fn getContentType(&mut self) -> String { crate::org::allbinary::media::audio::BasicPlayer::BasicPlayer::getContentType(crate::org::allbinary::media::audio::BasicPlayer::BasicPlayerDyn::as_BasicPlayer_mut(self)) }
    fn setLoopCount(&mut self, mut count: i32) { Self::setLoopCount(self, count) }
    fn addPlayerListener(&mut self, mut playerListener: Option<Rc<RefCell<dyn PlayerListener>>>) { Self::addPlayerListener(self, playerListener) }
    fn removePlayerListener(&mut self, mut playerListener: Option<Rc<RefCell<dyn PlayerListener>>>) { Self::removePlayerListener(self, playerListener) }
}

impl AndroidMediaPlayerWrapper {
    

    pub fn createPlayerWrapper() -> Option<Rc<RefCell<AndroidMediaPlayerWrapper>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return Some(AndroidMediaPlayerWrapper::new({ let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()));
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
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
            if resource == { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field } {
            } else {
                let mut resourceUtil: Option<Rc<RefCell<ResourceUtil>>> = ResourceUtil::getInstance();
                let mut mediaPlayer: Option<Rc<RefCell<MediaPlayer>>> = MediaPlayer::create(jcall!(resourceUtil).getContext(), ((jcall!(resourceUtil).getResourceId(resource.clone())).unwrap() as i32));
                if mediaPlayer.is_none() {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(StringMaker::new()).append(String::from("Failed to create media player for: ")).append(resource.clone()).append(String::from(" with id: ")).append((jcall!(resourceUtil).getResourceId(resource.clone())).expect("NullPointerException").to_string()).toString())));
                }
                this.setMediaPlayer(mediaPlayer.clone());
                jcall!(this.mediaPlayer).setLooping(false);
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(this.logUtil).put(format!("{}{}", { let java_field = jcall!(this.commonStrings).EXCEPTION_LABEL.clone(); java_field }, resource), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(this.commonStrings).CONSTRUCTOR.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            panic!("{:?}", e);
            }
        }
        this
    }

    pub fn new(mut resource: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(resource))))
    }
    

    pub fn setLoopCount(&mut self, mut count: i32) {
        { let java_arguments = (count.clone(),); self.base.setLoopCount(java_arguments.0) };
        if !match (&self.mediaPlayer, &crate::org::allbinary::android::NullAndroidCanvas::NullAndroidCanvas_NULL_MEDIA_PLAYER.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && self.mediaPlayer.is_some() {
            if count == 0 {
                jcall!(self.mediaPlayer).setLooping(false);
            } else {
            }
        }
    }
    

    // synchronized
    pub fn addPlayerListener(&mut self, mut playerListener: Option<Rc<RefCell<dyn PlayerListener>>>) {
        { let java_arguments = (playerListener.clone(),); self.base.addPlayerListener(java_arguments.0) };
    }
    

    pub fn removePlayerListener(&mut self, mut playerListener: Option<Rc<RefCell<dyn PlayerListener>>>) {
        { let java_arguments = (playerListener.clone(),); self.base.removePlayerListener(java_arguments.0) };
    }
    

    pub fn getState(&mut self) -> i32 {
        if jcall!(self.mediaPlayer).isPlaying() {
            return crate::javax::microedition::media::Player::Player_STARTED.with(Clone::clone).clone();
        } else {
            return crate::javax::microedition::media::Player::Player_PREFETCHED.with(Clone::clone).clone();
        }
    }
    

    pub fn close(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.mediaPlayer).release();
            self.mediaPlayer = { let java_value = (crate::org::allbinary::android::NullAndroidCanvas::NullAndroidCanvas_NULL_MEDIA_PLAYER.with(Clone::clone)).clone(); java_value }.clone();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).CLOSE.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
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
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).START_METHOD_NAME.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
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
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("stop"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn update(&mut self, mut event: String) {
        jcall!(self.logUtil).putF(format!("{}{}", String::from("LineEvent: "), event), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).UPDATE.clone(); java_field }.clone());
        let mut size: i32 = jcall!(self.listenersList).size();
        for index in (0..size) {
            let mut listener: Option<Rc<RefCell<dyn PlayerListener>>> = { let java_object = (jcall!(self.listenersList).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn PlayerListener>>>>().expect("ClassCastException").clone()) };
            jcall!(listener).playerUpdate(None /* Java this: a &mut self method has no Rc of itself */, event.clone(), crate::java_into_object({ let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }.clone()));
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
    fn addPlayerListener(&mut self, mut playerListener: Option<Rc<RefCell<dyn PlayerListener>>>) { Self::addPlayerListener(self, playerListener) }
    fn removePlayerListener(&mut self, mut playerListener: Option<Rc<RefCell<dyn PlayerListener>>>) { Self::removePlayerListener(self, playerListener) }
    fn getState(&mut self) -> i32 { Self::getState(self) }
    fn getDuration(&mut self) -> i64 { Self::getDuration(self) }
    fn setLoopCount(&mut self, mut count: i32) { Self::setLoopCount(self, count) }
    fn start(&mut self) { Self::start(self) }
    fn stop(&mut self) { Self::stop(self) }
}
impl crate::org::allbinary::media::audio::Controllable2::Controllable2 for AndroidMediaPlayerWrapper {
    fn setVolume(&mut self, mut leftVolume: i32, mut rightVolume: i32) { Self::setVolume(self, leftVolume, rightVolume) }
}



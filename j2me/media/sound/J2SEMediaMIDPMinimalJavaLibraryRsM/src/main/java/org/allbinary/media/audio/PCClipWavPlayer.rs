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
use crate::org::allbinary::thread::ARunnable::ARunnable;
// use crate::java::io::ByteArrayInputStream::ByteArrayInputStream;
use crate::java::io::InputStream::InputStream;
use crate::javax::microedition::media::Control::Control;
use crate::javax::microedition::media::MediaException::MediaException;
use crate::javax::microedition::media::PlayerListener::PlayerListener;
// use crate::javax::sound::sampled::AudioInputStream::AudioInputStream;
// use crate::javax::sound::sampled::AudioSystem::AudioSystem;
// use crate::javax::sound::sampled::Clip::Clip;
// use crate::javax::sound::sampled::FloatControl::FloatControl;
// use crate::javax::sound::sampled::LineEvent::LineEvent;
// use crate::javax::sound::sampled::LineListener::LineListener;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::thread::MusicThreadPool::MusicThreadPool;
use crate::java::lang::Exception::*;
use crate::java::lang::RuntimeException::*;
use super::BasicPlayer::BasicPlayer;
use super::NullClip::NullClip;
use super::NullControl::NullControl;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PCClipWavPlayer {
    pub base: BasicPlayer,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub audioInputStream: Option<Rc<RefCell<AudioInputStream>>>,
    pub clip: Option<Rc<RefCell<Clip>>>,
}

// impl LineListener for PCClipWavPlayer with the methods of the trait
impl crate::javax::microedition::media::Controllable::Controllable for PCClipWavPlayer {
    fn getControls(&mut self) -> Vec<Option<Rc<RefCell<dyn Control>>>> { Self::getControls(self) }
    fn getControl(&mut self, mut controlType: String) -> Option<Rc<RefCell<dyn Control>>> { Self::getControl(self, controlType) }
}
impl crate::javax::microedition::media::Player::Player for PCClipWavPlayer {
    fn realize(&mut self) { crate::org::allbinary::media::audio::BasicPlayer::BasicPlayer::realize(crate::org::allbinary::media::audio::BasicPlayer::BasicPlayerDyn::as_BasicPlayer_mut(self)) }
    fn prefetch(&mut self) { crate::org::allbinary::media::audio::BasicPlayer::BasicPlayer::prefetch(crate::org::allbinary::media::audio::BasicPlayer::BasicPlayerDyn::as_BasicPlayer_mut(self)) }
    fn start(&mut self) { Self::start(self) }
    fn stop(&mut self) { Self::stop(self) }
    fn deallocate(&mut self) { crate::org::allbinary::media::audio::BasicPlayer::BasicPlayer::deallocate(crate::org::allbinary::media::audio::BasicPlayer::BasicPlayerDyn::as_BasicPlayer_mut(self)) }
    fn close(&mut self) { Self::close(self) }
    fn setMediaTime(&mut self, p1: i64) -> i64 { crate::org::allbinary::media::audio::BasicPlayer::BasicPlayer::setMediaTime(crate::org::allbinary::media::audio::BasicPlayer::BasicPlayerDyn::as_BasicPlayer_mut(self), p1) }
    fn getMediaTime(&mut self) -> i64 { crate::org::allbinary::media::audio::BasicPlayer::BasicPlayer::getMediaTime(crate::org::allbinary::media::audio::BasicPlayer::BasicPlayerDyn::as_BasicPlayer_mut(self)) }
    fn getState(&mut self) -> i32 { crate::org::allbinary::media::audio::BasicPlayer::BasicPlayer::getState(crate::org::allbinary::media::audio::BasicPlayer::BasicPlayerDyn::as_BasicPlayer_mut(self)) }
    fn getDuration(&mut self) -> i64 { Self::getDuration(self) }
    fn getContentType(&mut self) -> String { Self::getContentType(self) }
    fn setLoopCount(&mut self, p1: i32) { crate::org::allbinary::media::audio::BasicPlayer::BasicPlayer::setLoopCount(crate::org::allbinary::media::audio::BasicPlayer::BasicPlayerDyn::as_BasicPlayer_mut(self), p1) }
    fn addPlayerListener(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::media::PlayerListener::PlayerListener>>>) { crate::org::allbinary::media::audio::BasicPlayer::BasicPlayer::addPlayerListener(crate::org::allbinary::media::audio::BasicPlayer::BasicPlayerDyn::as_BasicPlayer_mut(self), p1) }
    fn removePlayerListener(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::media::PlayerListener::PlayerListener>>>) { crate::org::allbinary::media::audio::BasicPlayer::BasicPlayer::removePlayerListener(crate::org::allbinary::media::audio::BasicPlayer::BasicPlayerDyn::as_BasicPlayer_mut(self), p1) }
}

impl PCClipWavPlayer {
    

    pub fn new_struct(mut inputStream: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>>) -> Self {
        let mut this = Self {
            base: BasicPlayer::new_struct(),
            logUtil: None,
            audioInputStream: None,
            clip: None,
        };
        this.logUtil = LogUtil::getInstance();
        let mut audioInputStream: Option<Rc<RefCell<AudioInputStream>>> = AudioInputStream::new(crate::java::io::InputStream::InputStream::new1({ let java_field = jcall!(NullUtil::getInstance()).NULL_BYTE_ARRAY.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>), NullAudioFormat_NULL_AUDIO_FORMAT.with(Clone::clone).clone(), 0);
        let mut clip: Option<Rc<RefCell<Clip>>> = NullClip::new();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            audioInputStream = { let java_value = (AudioSystem::getAudioInputStream(inputStream.clone())).clone(); java_value };
            clip = this.create(audioInputStream.clone());
            if clip.is_none() {
                jcall!(this.logUtil).put(String::from("Clip was null"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(this.commonStrings).CONSTRUCTOR.clone(); java_field }.clone(), crate::java_into_object(format!("{}: {:?}", "Exception", ())));
                panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(this.logUtil).put({ let java_field = jcall!(this.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(this.commonStrings).CONSTRUCTOR.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
        this.audioInputStream = audioInputStream.clone();
        this.clip = clip.clone();
        this
    }

    pub fn new(mut inputStream: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(inputStream))))
    }
    

    pub fn close(&mut self) {
        jcall!(MusicThreadPool::getInstance()).runTask(ARunnable::new() /* Java anonymous class body: implement it as a named type
            

            pub fn run(&mut self) {
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    self.close2();
                })) {
                    Ok(_) => {}
                    Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

                    PreLogUtil::putOE({ let java_field = jcall!(self.base.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.base.commonStrings).PROCESS.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
                    }
                }
            }
*/.map(|reference| reference as Rc<RefCell<dyn crate::Runnable>>));
    }
    

    // throws Exception
    pub fn close2(&mut self) {
        jcall!(self.clip).drain();
        jcall!(self.clip).flush();
        jcall!(self.clip).close();
    }
    

    pub fn getContentType(&mut self) -> String {
        return { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
    }
    

    // throws MediaException
    // synchronized
    pub fn start(&mut self) {
        jcall!(MusicThreadPool::getInstance()).runTask(ARunnable::new() /* Java anonymous class body: implement it as a named type
            

            pub fn run(&mut self) {
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    self.start2();
                })) {
                    Ok(_) => {}
                    Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

                    PreLogUtil::putOE({ let java_field = jcall!(self.base.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.base.commonStrings).PROCESS.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
                    }
                }
            }
*/.map(|reference| reference as Rc<RefCell<dyn crate::Runnable>>));
    }
    

    // throws Exception
    pub fn start2(&mut self) {
        jcall!(self.clip).setFramePosition(0);
        { let java_receiver = self.clip.clone(); let java_arguments = (self.getLoopCount(),); jcall!(java_receiver).r#loop(java_arguments.0); };
        jcall!(self.clip).start();
        self.base.start();
    }
    

    // throws Exception
    pub fn create(&mut self, mut audioInputStream: Option<Rc<RefCell<AudioInputStream>>>) -> Option<Rc<RefCell<Clip>>> {
        let mut clip: Option<Rc<RefCell<Clip>>> = AudioSystem::getClip();
        jcall!(clip).addLineListener(None /* Java this: a &mut self method has no Rc of itself */);
        jcall!(clip).open(audioInputStream.clone());
        return clip.clone();
    }
    

    // throws MediaException
    // synchronized
    pub fn stop(&mut self) {
        jcall!(MusicThreadPool::getInstance()).runTask(ARunnable::new() /* Java anonymous class body: implement it as a named type
            

            pub fn run(&mut self) {
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    self.stop2();
                })) {
                    Ok(_) => {}
                    Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

                    PreLogUtil::putOE({ let java_field = jcall!(self.base.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.base.commonStrings).PROCESS.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
                    }
                }
            }
*/.map(|reference| reference as Rc<RefCell<dyn crate::Runnable>>));
    }
    

    // throws Exception
    pub fn stop2(&mut self) {
        jcall!(self.clip).stop();
        self.base.stop();
    }
    

    pub fn getControl(&mut self, mut controlType: String) -> Option<Rc<RefCell<dyn Control>>> {
        return NullControl::new().map(|reference| reference as Rc<RefCell<dyn Control>>).map(|reference| reference as Rc<RefCell<dyn Control>>);
    }
    

    pub fn getControls(&mut self) -> Vec<Option<Rc<RefCell<dyn Control>>>> {
        return vec![None; (0) as usize];
    }
    

    pub fn setVolume(&mut self, mut leftVolume: i32, mut rightVolume: i32) {
        self.setVolumeF((((leftVolume) as f32)) / 100.0_f32);
    }
    

    pub fn getVolume(&mut self) -> f32 {
        let mut masterGainFloatControl: Option<Rc<RefCell<FloatControl>>> = (jcall!(self.clip).getControl({ let java_field = jcall!(unsafe { crate::javax::sound::sampled::FloatControl::FloatControl_Type.clone() }).MASTER_GAIN.clone(); java_field }.clone())).clone() /* as FloatControl */;
        return ((((10.0_f32) as f64).powf((jcall!(masterGainFloatControl).getValue() / 20.0_f32) as f64)) as f32);
    }
    

    pub fn setVolumeF(&mut self, mut volume: f32) {
        if volume < 0.0_f32 || volume > 1.0_f32 {
            panic!("{:?}", format!("{}: {:?}", "IllegalArgumentException", (format!("{}{}", String::from("Volume: "), volume))));
        }
        let mut masterGainFloatControl: Option<Rc<RefCell<FloatControl>>> = (jcall!(self.clip).getControl({ let java_field = jcall!(unsafe { crate::javax::sound::sampled::FloatControl::FloatControl_Type.clone() }).MASTER_GAIN.clone(); java_field }.clone())).clone() /* as FloatControl */;
        jcall!(masterGainFloatControl).setValue(20.0_f32 * ((((volume.clone()) as f64).log10()) as f32));
    }
    

    pub fn getDuration(&mut self) -> i64 {
        return jcall!(self.clip).getMicrosecondLength() / 1000;
    }
    

    pub fn update(&mut self, mut event: Option<Rc<RefCell<LineEvent>>>) {
        if jcall!(jcall!(event).getType()).equals({ let java_field = jcall!(unsafe { crate::javax::sound::sampled::LineEvent::LineEvent_Type.clone() }).STOP.clone(); java_field }.clone()) {
            let mut size: i32 = jcall!(self.listenersList).size();
            for index in (0..size) {
                let mut listener: Option<Rc<RefCell<dyn PlayerListener>>> = { let java_object = (jcall!(self.listenersList).get(size.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn PlayerListener>>>>().expect("ClassCastException").clone()) };
                jcall!(listener).playerUpdate(None /* Java this: a &mut self method has no Rc of itself */, crate::javax::microedition::media::PlayerListener::PlayerListener_END_OF_MEDIA.with(Clone::clone).clone(), None);
            }
        }
    }
}
impl std::ops::Deref for PCClipWavPlayer {
    type Target = BasicPlayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PCClipWavPlayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::audio::BasicPlayer::BasicPlayerDyn for PCClipWavPlayer {
    fn as_BasicPlayer(&self) -> &crate::org::allbinary::media::audio::BasicPlayer::BasicPlayer { &self.base }
    fn as_BasicPlayer_mut(&mut self) -> &mut crate::org::allbinary::media::audio::BasicPlayer::BasicPlayer { &mut self.base }
    fn getContentType(&mut self) -> String { Self::getContentType(self) }
    fn getControl(&mut self, mut controlType: String) -> Option<Rc<RefCell<dyn Control>>> { Self::getControl(self, controlType) }
    fn getControls(&mut self) -> Vec<Option<Rc<RefCell<dyn Control>>>> { Self::getControls(self) }
    fn close(&mut self) { Self::close(self) }
    fn getDuration(&mut self) -> i64 { Self::getDuration(self) }
    fn start(&mut self) { Self::start(self) }
    fn stop(&mut self) { Self::stop(self) }
}
impl crate::org::allbinary::media::audio::Controllable2::Controllable2 for PCClipWavPlayer {
    fn setVolume(&mut self, mut leftVolume: i32, mut rightVolume: i32) { Self::setVolume(self, leftVolume, rightVolume) }
}



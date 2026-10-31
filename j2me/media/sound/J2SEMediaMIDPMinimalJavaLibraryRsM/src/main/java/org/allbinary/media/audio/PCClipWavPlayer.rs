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
// use crate::java::io::InputStream::InputStream;
// use crate::javax::microedition::media::Control::Control;
// use crate::javax::microedition::media::MediaException::MediaException;
// use crate::javax::microedition::media::PlayerListener::PlayerListener;
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

impl PCClipWavPlayer {
    

    pub fn new_struct(mut inputStream: Option<Rc<RefCell<InputStream>>>) -> Self {
        let mut this = Self {
            base: BasicPlayer::new_struct(),
            logUtil: None,
            audioInputStream: None,
            clip: None,
        };
        this.logUtil = LogUtil::getInstance();
        let mut audioInputStream: Option<Rc<RefCell<AudioInputStream>>> = AudioInputStream::new(ByteArrayInputStream::new(jcall!(NullUtil::getInstance()).NULL_BYTE_ARRAY.clone()), NullAudioFormat_NULL_AUDIO_FORMAT.with(Clone::clone).clone(), 0);
        let mut clip: Option<Rc<RefCell<Clip>>> = NullClip::new();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            audioInputStream = AudioSystem::getAudioInputStream(inputStream.clone());
            clip = this.create(audioInputStream.clone());
            if clip.is_none() {
                jcall!(this.logUtil).put(String::from("Clip was null"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(format!("{}: {:?}", "Exception", ())));
                panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(this.logUtil).put(jcall!(this.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(e.clone()));
            }
        }
        this.audioInputStream = audioInputStream.clone();
        this.clip = clip.clone();
        this
    }

    pub fn new(mut inputStream: Option<Rc<RefCell<InputStream>>>) -> Option<Rc<RefCell<Self>>> {
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
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                    PreLogUtil::putOE(jcall!(self.base.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.base.commonStrings).PROCESS.clone(), e.clone());
                    }
                }
            }
*/);
    }
    

    // throws Exception
    pub fn close2(&mut self) {
        jcall!(self.clip).drain();
        jcall!(self.clip).flush();
        jcall!(self.clip).close();
    }
    

    pub fn getContentType(&mut self) -> String {
        return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
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
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                    PreLogUtil::putOE(jcall!(self.base.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.base.commonStrings).PROCESS.clone(), e.clone());
                    }
                }
            }
*/);
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
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                    PreLogUtil::putOE(jcall!(self.base.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.base.commonStrings).PROCESS.clone(), e.clone());
                    }
                }
            }
*/);
    }
    

    // throws Exception
    pub fn stop2(&mut self) {
        jcall!(self.clip).stop();
        self.base.stop();
    }
    

    pub fn getControl(&mut self, mut controlType: String) -> Option<Rc<RefCell<Control>>> {
        return NullControl::new();
    }
    

    pub fn getControls(&mut self) -> Vec<Option<Rc<RefCell<Control>>>> {
        return vec![None; (0) as usize];
    }
    

    pub fn setVolume(&mut self, mut leftVolume: i32, mut rightVolume: i32) {
        self.setVolumeF((((leftVolume) as f32)) / 100.0_f32);
    }
    

    pub fn getVolume(&mut self) -> f32 {
        let mut masterGainFloatControl: Option<Rc<RefCell<FloatControl>>> = (jcall!(self.clip).getControl(jcall!(unsafe { crate::javax::sound::sampled::FloatControl::FloatControl_Type.clone() }).MASTER_GAIN.clone())).clone() /* as FloatControl */;
        return ((((10.0_f32) as f64).powf((jcall!(masterGainFloatControl).getValue() / 20.0_f32) as f64)) as f32);
    }
    

    pub fn setVolumeF(&mut self, mut volume: f32) {
        if volume < 0.0_f32 || volume > 1.0_f32 {
            panic!("{:?}", format!("{}: {:?}", "IllegalArgumentException", (format!("{}{}", String::from("Volume: "), volume))));
        }
        let mut masterGainFloatControl: Option<Rc<RefCell<FloatControl>>> = (jcall!(self.clip).getControl(jcall!(unsafe { crate::javax::sound::sampled::FloatControl::FloatControl_Type.clone() }).MASTER_GAIN.clone())).clone() /* as FloatControl */;
        jcall!(masterGainFloatControl).setValue(20.0_f32 * ((((volume.clone()) as f64).log10()) as f32));
    }
    

    pub fn getDuration(&mut self) -> i64 {
        return jcall!(self.clip).getMicrosecondLength() / 1000;
    }
    

    pub fn update(&mut self, mut event: Option<Rc<RefCell<LineEvent>>>) {
        if jcall!(jcall!(event).getType()).equals(jcall!(unsafe { crate::javax::sound::sampled::LineEvent::LineEvent_Type.clone() }).STOP.clone()) {
            let mut size: i32 = jcall!(self.listenersList).size();
            for index in (0..size) {
                let mut listener: Option<Rc<RefCell<PlayerListener>>> = (jcall!(self.listenersList).get(size.clone())).clone() /* as PlayerListener */;
                jcall!(listener).playerUpdate(None /* Java this: a &mut self method has no Rc of itself */, unsafe { crate::javax::microedition::media::PlayerListener::PlayerListener_END_OF_MEDIA.clone() }.clone(), None);
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
    fn getControl(&mut self, mut controlType: String) -> Option<Rc<RefCell<Control>>> { Self::getControl(self, controlType) }
    fn getControls(&mut self) -> Vec<Option<Rc<RefCell<Control>>>> { Self::getControls(self) }
    fn close(&mut self) { Self::close(self) }
    fn getDuration(&mut self) -> i64 { Self::getDuration(self) }
    fn start(&mut self) { Self::start(self) }
    fn stop(&mut self) { Self::stop(self) }
}
impl crate::org::allbinary::media::audio::Controllable2::Controllable2 for PCClipWavPlayer {
    fn setVolume(&mut self, mut leftVolume: i32, mut rightVolume: i32) { Self::setVolume(self, leftVolume, rightVolume) }
}



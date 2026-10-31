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
// use crate::java::io::BufferedInputStream::BufferedInputStream;
// use crate::java::io::ByteArrayInputStream::ByteArrayInputStream;
// use crate::java::io::IOException::IOException;
// use crate::java::io::InputStream::InputStream;
// use crate::javax::microedition::media::Manager::Manager;
// use crate::javax::microedition::media::MediaException::MediaException;
// use crate::javax::microedition::media::Player::Player;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::audio::AudioContentTypeDataFactory::AudioContentTypeDataFactory;
use crate::org::allbinary::data::resource::ResourceUtil::ResourceUtil;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::GameFeatureFactory::GameFeatureFactory;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvasFactory::ProgressCanvasFactory;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::Sounds::Sounds;
use super::SoundsFactoryInterface::SoundsFactoryInterface;
use super::PCClipWavPlayer::PCClipWavPlayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//J2SEMediaMIDPMinimalJavaLibrary
pub struct AllBinaryMediaManager {
}

thread_local! { pub static AllBinaryMediaManager_THIS: String = String::from("AllBinaryMediaManagerPC"); }
thread_local! { pub static AllBinaryMediaManager_CREATE_PLAYER: String = String::from("createPlayer"); }

impl AllBinaryMediaManager {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isMuted() -> bool {
        return false;
    }
    

    pub fn setMuted(mut aMuted: bool) {
    }
    

    pub fn update() -> bool {
        return true;
    }
    

    // throws Exception
    pub fn init(mut soundsFactoryInterface: Option<Rc<RefCell<dyn SoundsFactoryInterface>>>) {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        let mut commonString: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        jcall!(logUtil).putF(jcall!(commonString).START.clone(), crate::java_into_object(AllBinaryMediaManager_THIS.with(Clone::clone).clone()), jcall!(commonString).INIT.clone());
        jcall!(ProgressCanvasFactory::getInstance()).addNormalPortion(50, String::from("Media Manager"));
        jcall!(Sounds::new(soundsFactoryInterface.clone())).init();
    }
    

    // throws Exception
    pub fn shutdown(mut soundsFactoryInterface: Option<Rc<RefCell<dyn SoundsFactoryInterface>>>) {
        jcall!(Sounds::new(soundsFactoryInterface.clone())).stopAll();
        jcall!(Sounds::new(soundsFactoryInterface.clone())).closeAll();
        jcall!(TsUtil::getInstance()).gc();
    }
    

    // throws Exception
    pub fn createPlayer(mut resource: String) -> Option<Rc<RefCell<Player>>> {
        if resource.starts_with(&unsafe { crate::javax::microedition::media::Manager::Manager_TONE_DEVICE_LOCATOR.clone() }.clone()) {
            return AllBinaryMediaManager::createPlayerFromInputStream(ByteArrayInputStream::new(jcall!(NullUtil::getInstance()).NULL_BYTE_ARRAY.clone()), jcall!(jcall!(AudioContentTypeDataFactory::getInstance()).MIME_AUDIO_TONE).getName());
        } else if jcall!(Features::getInstance()).isFeature(jcall!(GameFeatureFactory::getInstance()).SOUND.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut inputStream: Option<Rc<RefCell<InputStream>>> = jcall!(ResourceUtil::getInstance()).getResourceAsStream(resource.clone());
                let mut bufferedInputStream: Option<Rc<RefCell<BufferedInputStream>>> = BufferedInputStream::new(inputStream.clone());
                return PCClipWavPlayer::new(bufferedInputStream.clone());
            })) {
                Ok(java_return) => return java_return,
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
                let mut commonString: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(logUtil).put(jcall!(commonString).EXCEPTION.clone(), crate::java_into_object(AllBinaryMediaManager_THIS.with(Clone::clone).clone()), AllBinaryMediaManager_CREATE_PLAYER.with(Clone::clone).clone(), crate::java_into_object(e.clone()));
                return NoPlayer_NO_PLAYER.with(Clone::clone).clone();
                }
            }
        } else {
            return NoPlayer_NO_PLAYER.with(Clone::clone).clone();
        }
    }
    

    // throws IOException, MediaException
    pub fn createPlayerFromInputStream(mut stream: Option<Rc<RefCell<InputStream>>>, mut r#type: String) -> Option<Rc<RefCell<Player>>> {
        panic!("{:?}", format!("{}: {:?}", "MediaException", (String::from("No Impl"))));
    }
    

    // throws MediaException
    // synchronized
    pub fn playTone(mut frequency: i32, mut time: i32, mut volume: i32) {
        panic!("{:?}", format!("{}: {:?}", "MediaException", (String::from("No Impl"))));
    }
    

    pub fn isInitialized() -> bool {
        return true;
    }
}


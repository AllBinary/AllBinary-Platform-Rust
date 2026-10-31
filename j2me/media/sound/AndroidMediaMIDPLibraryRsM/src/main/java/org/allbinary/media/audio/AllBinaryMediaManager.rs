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
// use crate::java::io::IOException::IOException;
// use crate::java::io::InputStream::InputStream;
// use crate::javax::microedition::media::MediaException::MediaException;
// use crate::javax::microedition::media::Player::Player;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::GameFeatureFactory::GameFeatureFactory;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvasFactory::ProgressCanvasFactory;
use crate::org::allbinary::logic::ABSystemWrapper::ABSystemWrapper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::Sounds::Sounds;
use super::SoundsFactoryInterface::SoundsFactoryInterface;
use super::Sound::Sound;
use super::AndroidMediaPlayerWrapper::AndroidMediaPlayerWrapper;
use super::PlayerComposite::PlayerComposite;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//AndroidMediaMIDPLibrary
pub struct AllBinaryMediaManager {
}

thread_local! { pub static AllBinaryMediaManager_THIS: String = String::from("AllBinaryMediaManagerAndroid"); }
pub static mut AllBinaryMediaManager_mostUsedTotal: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| 0);

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
        AllBinaryMediaManager::shutdown(soundsFactoryInterface.clone());
        jcall!(ProgressCanvasFactory::getInstance()).addNormalPortion(50, String::from("Media Manager"));
        jcall!(TsUtil::getInstance()).gc();
        jcall!(Sounds::new(soundsFactoryInterface.clone())).init();
        jcall!(logUtil).putF(jcall!(commonString).END.clone(), crate::java_into_object(AllBinaryMediaManager_THIS.with(Clone::clone).clone()), jcall!(commonString).INIT.clone());
    }
    

    // throws Exception
    pub fn shutdown(mut soundsFactoryInterface: Option<Rc<RefCell<dyn SoundsFactoryInterface>>>) {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        let mut commonString: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        jcall!(logUtil).putF(jcall!(commonString).START.clone(), crate::java_into_object(AllBinaryMediaManager_THIS.with(Clone::clone).clone()), String::from("shutdown"));
        if jcall!(soundsFactoryInterface).isInitialized() {
            jcall!(Sounds::new(soundsFactoryInterface.clone())).stopAll();
            let mut soundInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>> = jcall!(soundsFactoryInterface).getSoundInterfaceArray();
            let mut player: Option<Rc<RefCell<Player>>> = None;
            let mut player2: Option<Rc<RefCell<Player>>> = None;
            let mut androidMediaPlayerWrapper: Option<Rc<RefCell<AndroidMediaPlayerWrapper>>> = None;
            for i in (0..(soundInterfaceArray.len() as i32)) {
                if soundInterfaceArray[(i) as usize].is_some() {
                    player = jcall!(soundInterfaceArray[(i) as usize]).getPlayerP();
                    if player.is_some() {
                        player2 = jcall!(((player).clone() /* as PlayerComposite */)).getPlayerP();
                        if (player2).is_some() /* instanceof AndroidMediaPlayerWrapper */ {
                            androidMediaPlayerWrapper = (player2).clone() /* as AndroidMediaPlayerWrapper */;
                            jcall!(MediaPlayerUtil::MediaPlayerUtil::getInstance()).waitForMediaPlayer(jcall!(androidMediaPlayerWrapper).getMediaPlayer());
                        } else {
                            panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Unknown Property Player: "), std::any::type_name_of_val(&player).to_string()))));
                        }
                    }
                }
            }
            jcall!(Sounds::new(soundsFactoryInterface.clone())).closeAll();
            jcall!(TsUtil::getInstance()).gc();
            jcall!(soundsFactoryInterface).setInitialized(false);
            unsafe { *AllBinaryMediaManager_mostUsedTotal = 0 };
        }
        jcall!(logUtil).putF(jcall!(commonString).START.clone(), crate::java_into_object(AllBinaryMediaManager_THIS.with(Clone::clone).clone()), String::from("shutdown"));
    }
    

    // throws Exception
    pub fn createPlayer(mut resource: String) -> Option<Rc<RefCell<Player>>> {
        unsafe { *AllBinaryMediaManager_mostUsedTotal += 1 };
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        if jcall!(Features::getInstance()).isFeature(jcall!(GameFeatureFactory::getInstance()).SOUND.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                return AndroidMediaPlayerWrapper::new(resource.clone());
            })) {
                Ok(java_return) => return java_return,
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                jcall!(logUtil).put(format!("{}{}", format!("{}{}", String::from("Could not create AndroidMediaPlayerWrapper using NoPlayer at "), jcall!(CommonLabels::getInstance()).TOTAL_LABEL), unsafe { AllBinaryMediaManager_mostUsedTotal.clone() }), crate::java_into_object(AllBinaryMediaManager_THIS.with(Clone::clone).clone()), String::from("createPlayer"), crate::java_into_object(e.clone()));
                return NoPlayer_NO_PLAYER.with(Clone::clone).clone();
                }
            }
        } else {
            return NoPlayer_NO_PLAYER.with(Clone::clone).clone();
        }
    }
    

    // throws IOException, MediaException
    pub fn createPlayerFromInputStream(mut stream: Option<Rc<RefCell<InputStream>>>, mut r#type: String) -> Option<Rc<RefCell<Player>>> {
        panic!("{:?}", format!("{}: {:?}", "MediaException", (String::from("No Input Stream Player"))));
    }
    

    // throws MediaException
    // synchronized
    pub fn playTone(mut frequency: i32, mut time: i32, mut volume: i32) {
        panic!("{:?}", format!("{}: {:?}", "MediaException", (String::from("No Tone Player"))));
    }
    

    pub fn isInitialized() -> bool {
        return true;
    }
}


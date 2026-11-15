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
use crate::java::io::IOException::IOException;
use crate::java::io::InputStream::InputStream;
use crate::javax::microedition::media::MediaException::MediaException;
use crate::javax::microedition::media::Player::Player;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::GameFeatureFactory::GameFeatureFactory;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvasFactory::ProgressCanvasFactory;
use crate::org::allbinary::logic::ABSystemWrapper::ABSystemWrapper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;
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

pub static AllBinaryMediaManager_THIS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("AllBinaryMediaManagerAndroid"));
pub static mut AllBinaryMediaManager_mostUsedTotal: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| 0);

impl AllBinaryMediaManager {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

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
        jcall!(logUtil).putF({ let java_field = jcall!(commonString).START.clone(); java_field }.clone(), crate::java_into_object(AllBinaryMediaManager_THIS.with(Clone::clone).clone()), { let java_field = jcall!(commonString).INIT.clone(); java_field }.clone());
        AllBinaryMediaManager::shutdown(soundsFactoryInterface.clone());
        jcall!(ProgressCanvasFactory::getInstance()).addNormalPortion(50, String::from("Media Manager"));
        jcall!(TsUtil::getInstance()).gc();
        jcall!(Sounds::new(soundsFactoryInterface.clone())).init();
        jcall!(logUtil).putF({ let java_field = jcall!(commonString).END.clone(); java_field }.clone(), crate::java_into_object(AllBinaryMediaManager_THIS.with(Clone::clone).clone()), { let java_field = jcall!(commonString).INIT.clone(); java_field }.clone());
    }
    

    // throws Exception
    pub fn shutdown(mut soundsFactoryInterface: Option<Rc<RefCell<dyn SoundsFactoryInterface>>>) {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        let mut commonString: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        jcall!(logUtil).putF({ let java_field = jcall!(commonString).START.clone(); java_field }.clone(), crate::java_into_object(AllBinaryMediaManager_THIS.with(Clone::clone).clone()), String::from("shutdown"));
        if jcall!(soundsFactoryInterface).isInitialized() {
            jcall!(Sounds::new(soundsFactoryInterface.clone())).stopAll();
            let mut soundInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>> = jcall!(soundsFactoryInterface).getSoundInterfaceArray();
            let mut player: Option<Rc<RefCell<dyn Player>>> = None;
            let mut player2: Option<Rc<RefCell<dyn Player>>> = None;
            let mut androidMediaPlayerWrapper: Option<Rc<RefCell<AndroidMediaPlayerWrapper>>> = None;
            for i in (0..(soundInterfaceArray.len() as i32)) {
                if soundInterfaceArray[(i) as usize].is_some() {
                    player = { let java_value = (jcall!(soundInterfaceArray[(i) as usize]).as_Sound_mut().getPlayerP()).clone(); java_value };
                    if player.is_some() {
                        player2 = { let java_value = (jcall!(((player).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<PlayerComposite>() }, "ClassCastException: not a PlayerComposite"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<PlayerComposite>) } }))).getPlayerP()).clone(); java_value };
                        if (player2).is_some() /* instanceof AndroidMediaPlayerWrapper */ {
                            androidMediaPlayerWrapper = (player2).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<AndroidMediaPlayerWrapper>() }, "ClassCastException: not a AndroidMediaPlayerWrapper"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<AndroidMediaPlayerWrapper>) } });
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
        jcall!(logUtil).putF({ let java_field = jcall!(commonString).START.clone(); java_field }.clone(), crate::java_into_object(AllBinaryMediaManager_THIS.with(Clone::clone).clone()), String::from("shutdown"));
    }
    

    // throws Exception
    pub fn createPlayer(mut resource: String) -> Option<Rc<RefCell<dyn Player>>> {
        unsafe { *AllBinaryMediaManager_mostUsedTotal += 1 };
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        if jcall!(Features::getInstance()).isFeature({ let java_field = jcall!(GameFeatureFactory::getInstance()).SOUND.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                return Some(AndroidMediaPlayerWrapper::new(resource.clone()).map(|reference| reference as Rc<RefCell<dyn Player>>).map(|reference| reference as Rc<RefCell<dyn Player>>));
            })) {
                Ok(java_return) => java_return,
                Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

                jcall!(logUtil).put(format!("{}{}", format!("{}{}", String::from("Could not create AndroidMediaPlayerWrapper using NoPlayer at "), { let java_field = jcall!(CommonLabels::getInstance()).TOTAL_LABEL.clone(); java_field }), unsafe { AllBinaryMediaManager_mostUsedTotal.clone() }), crate::java_into_object(AllBinaryMediaManager_THIS.with(Clone::clone).clone()), String::from("createPlayer"), crate::java_into_object(e.clone()));
                return NoPlayer_NO_PLAYER.with(Clone::clone).clone().map(|reference| reference as Rc<RefCell<dyn Player>>);
                None
                }
            };
        if let Some(java_return) = java_try_result { return java_return; }
        } else {
            return NoPlayer_NO_PLAYER.with(Clone::clone).clone().map(|reference| reference as Rc<RefCell<dyn Player>>);
        }
    }
    

    // throws IOException, MediaException
    pub fn createPlayerFromInputStream(mut stream: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>>, mut r#type: String) -> Option<Rc<RefCell<dyn Player>>> {
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


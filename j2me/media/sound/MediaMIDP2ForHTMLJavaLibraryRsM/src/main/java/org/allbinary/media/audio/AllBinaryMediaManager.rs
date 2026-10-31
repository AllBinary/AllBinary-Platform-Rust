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
// use crate::javax::microedition::media::Manager::Manager;
// use crate::javax::microedition::media::MediaException::MediaException;
// use crate::javax::microedition::media::Player::Player;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::GameFeatureFactory::GameFeatureFactory;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvasFactory::ProgressCanvasFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::Sounds::Sounds;
use super::SoundsFactoryInterface::SoundsFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//MediaMIDP2ForHTMLJavaLibrary
pub struct AllBinaryMediaManager {
}

thread_local! { pub static AllBinaryMediaManager_THIS: String = String::from("AllBinaryMediaManagerHTML5"); }

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
        if jcall!(Features::getInstance()).isFeature(jcall!(GameFeatureFactory::getInstance()).SOUND.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            if { let left = &(resource); let right = &(unsafe { crate::javax::microedition::media::Manager::Manager_TONE_DEVICE_LOCATOR.clone() }); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } == 0 {
                return Manager::createPlayer(resource.clone());
            } else {
                return Manager::createPlayer(resource[(0) as usize..((resource.chars().count() as i32) - 4) as usize].to_string());
            }
        } else {
            return NoPlayer_NO_PLAYER.with(Clone::clone).clone();
        }
    }
    

    // throws MediaException
    // synchronized
    pub fn playTone(mut frequency: i32, mut time: i32, mut volume: i32) {
        if jcall!(Features::getInstance()).isFeature(jcall!(GameFeatureFactory::getInstance()).SOUND.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            Manager::playTone(frequency.clone(), time.clone(), volume.clone());
        }
    }
}


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
use crate::javax::microedition::media::Player::Player;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvasFactory::ProgressCanvasFactory;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::java::lang::Exception::*;
use super::SoundStrings::SoundStrings;
use super::SoundsFactoryInterface::SoundsFactoryInterface;
use super::Sound::Sound;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Sounds {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub soundStrings: Option<Rc<RefCell<SoundStrings>>>,
    pub soundsFactoryInterface: Option<Rc<RefCell<dyn SoundsFactoryInterface>>>,
}


impl Sounds {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut soundsFactoryInterface: Option<Rc<RefCell<dyn SoundsFactoryInterface>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            soundStrings: None,
            soundsFactoryInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.soundStrings = SoundStrings::getInstance();
        this.soundsFactoryInterface = soundsFactoryInterface.clone();
        this
    }

    pub fn new(mut soundsFactoryInterface: Option<Rc<RefCell<dyn SoundsFactoryInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(soundsFactoryInterface))))
    }
    

    // throws Exception
    pub fn init(&mut self) {
        jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone());
        let mut commonLabels: Option<Rc<RefCell<CommonLabels>>> = CommonLabels::getInstance();
        let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
        jcall!(self.soundsFactoryInterface).init();
        let mut soundInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>> = jcall!(self.soundsFactoryInterface).getSoundInterfaceArray();
        let mut smallIntegerSingletonFactory: Option<Rc<RefCell<SmallIntegerSingletonFactory>>> = SmallIntegerSingletonFactory::getInstance();
        let mut indexInteger: Option<i32> = None;
        let mut indexString: String = String::new();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        for i in (0..(soundInterfaceArray.len() as i32)) {
            let mut soundInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>> = soundInterfaceArray[(i) as usize].clone();
            if soundInterface.is_some() {
                indexInteger = { let java_value = (jcall!(smallIntegerSingletonFactory).getAt(i.clone())).clone(); java_value };
                indexString = { let java_value = ((indexInteger).expect("NullPointerException").to_string()).clone(); java_value };
                jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                jcall!(stringBuffer).append({ let java_field = jcall!(commonLabels).INDEX_LABEL.clone(); java_field }.clone());
                jcall!(stringBuffer).append(indexString.clone());
                jcall!(stringBuffer).append({ let java_field = jcall!(self.soundStrings).SOUND.clone(); java_field }.clone());
                jcall!(stringBuffer).append(std::any::type_name_of_val(&soundInterface).to_string());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                jcall!(soundInterface).as_Sound_mut().init();
                jcall!(progressCanvas).addNormalPortion(100, jcall!(StringMaker::new()).append({ let java_field = jcall!(self.soundStrings).INIT_SOUND.clone(); java_field }.clone()).append(indexString.clone()).toString());
            }
        }
        jcall!(self.soundsFactoryInterface).setInitialized(true);
        jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).END.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone());
    }
    

    // throws Exception
    pub fn stopAll(&mut self) {
        jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.soundStrings).STOP_ALL.clone(); java_field }.clone());
        let mut soundInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>> = jcall!(self.soundsFactoryInterface).getSoundInterfaceArray();
        let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
        let mut player: Option<Rc<RefCell<dyn Player>>> = None;
        for index in (0..(soundInterfaceArray.len() as i32)) {
            if soundInterfaceArray[(index) as usize].is_some() {
                player = { let java_value = (jcall!(soundInterfaceArray[(index) as usize]).as_Sound_mut().getPlayerP()).clone(); java_value };
                if player.is_some() {
                    jcall!(player).stop();
                    jcall!(progressCanvas).addPortion(100, { let java_field = jcall!(self.soundStrings).STOPPING_SOUND.clone(); java_field }.clone(), index.clone());
                }
            }
        }
    }
    

    // throws Exception
    pub fn closeAll(&mut self) {
        jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.soundStrings).CLOSE_ALL.clone(); java_field }.clone());
        let mut soundInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>> = jcall!(self.soundsFactoryInterface).getSoundInterfaceArray();
        let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
        let mut player: Option<Rc<RefCell<dyn Player>>> = None;
        for index in (0..(soundInterfaceArray.len() as i32)) {
            if soundInterfaceArray[(index) as usize].is_some() {
                player = { let java_value = (jcall!(soundInterfaceArray[(index) as usize]).as_Sound_mut().getPlayerP()).clone(); java_value };
                if player.is_some() {
                    jcall!(player).close();
                    jcall!(progressCanvas).addPortion(100, { let java_field = jcall!(self.soundStrings).CLOSING_SOUND.clone(); java_field }.clone(), index.clone());
                }
            }
        }
    }
}


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
// package org.allbinary.audio
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::AudioContentTypeData::AudioContentTypeData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AudioContentTypeDataFactory {
    pub MIME_AUDIO_TONE: Option<Rc<RefCell<AudioContentTypeData>>>,
    pub MIME_AUDIO_AMR: Option<Rc<RefCell<AudioContentTypeData>>>,
    pub MIME_AUDIO_AMR_WB: Option<Rc<RefCell<AudioContentTypeData>>>,
    pub MIME_AUDIO_WAV: Option<Rc<RefCell<AudioContentTypeData>>>,
    pub MIME_AUDIO_MIDI: Option<Rc<RefCell<AudioContentTypeData>>>,
    pub MIME_AUDIO_SP_MIDI: Option<Rc<RefCell<AudioContentTypeData>>>,
}

thread_local! { pub static AudioContentTypeDataFactory_instance: Option<Rc<RefCell<AudioContentTypeDataFactory>>> = AudioContentTypeDataFactory::new(); }

impl AudioContentTypeDataFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<AudioContentTypeDataFactory>>> {
        return AudioContentTypeDataFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            MIME_AUDIO_TONE: None,
            MIME_AUDIO_AMR: None,
            MIME_AUDIO_AMR_WB: None,
            MIME_AUDIO_WAV: None,
            MIME_AUDIO_MIDI: None,
            MIME_AUDIO_SP_MIDI: None,
        };
        this.MIME_AUDIO_TONE = AudioContentTypeData::new(String::from("audio/x-tone-seq"));
        this.MIME_AUDIO_AMR = AudioContentTypeData::new(String::from("audio/amr"));
        this.MIME_AUDIO_AMR_WB = AudioContentTypeData::new(String::from("audio/amr-wb"));
        this.MIME_AUDIO_WAV = AudioContentTypeData::new(String::from("audio/x-wav"));
        this.MIME_AUDIO_MIDI = AudioContentTypeData::new(String::from("audio/midi"));
        this.MIME_AUDIO_SP_MIDI = AudioContentTypeData::new(String::from("audio/sp-midi"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}


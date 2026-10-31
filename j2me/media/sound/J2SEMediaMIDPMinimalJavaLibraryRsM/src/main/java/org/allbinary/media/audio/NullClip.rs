// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// use crate::javax::sound::sampled::AudioFormat::AudioFormat;
// use crate::javax::sound::sampled::AudioInputStream::AudioInputStream;
// use crate::javax::sound::sampled::BooleanControl::BooleanControl;
// use crate::javax::sound::sampled::Clip::Clip;
// use crate::javax::sound::sampled::Control::Control;
// use crate::javax::sound::sampled::Control::Type::Type;
// use crate::javax::sound::sampled::Line::Info::Info;
// use crate::javax::sound::sampled::LineListener::LineListener;
// use crate::javax::sound::sampled::LineUnavailableException::LineUnavailableException;
use super::NullJ2SEControl::NullJ2SEControl;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullClip {
}

thread_local! { pub static NullClip_NULL_CONTROL: Option<Rc<RefCell<NullJ2SEControl>>> = NullJ2SEControl::new(jcall!(unsafe { crate::javax::sound::sampled::BooleanControl::BooleanControl_Type.clone() }).MUTE.clone()); }
// impl Clip for NullClip with the methods of the trait

impl NullClip {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws LineUnavailableException
    pub fn open4(&mut self, mut format: Option<Rc<RefCell<AudioFormat>>>, mut data: Vec<i8>, mut offset: i32, mut bufferSize: i32) {
    }
    

    // throws LineUnavailableException, IOException
    // Java overloaded method: Rust needs a different name
    pub fn open1(&mut self, mut stream: Option<Rc<RefCell<AudioInputStream>>>) {
    }
    

    pub fn getFrameLength(&mut self) -> i32 {
        return 0;
    }
    

    pub fn getMicrosecondLength(&mut self) -> i64 {
        return 0_i64;
    }
    

    pub fn setFramePosition(&mut self, mut frames: i32) {
    }
    

    pub fn setMicrosecondPosition(&mut self, mut microseconds: i64) {
    }
    

    pub fn setLoopPoints(&mut self, mut start: i32, mut end: i32) {
    }
    

    pub fn r#loop(&mut self, mut count: i32) {
    }
    

    pub fn drain(&mut self) {
    }
    

    pub fn flush(&mut self) {
    }
    

    pub fn start(&mut self) {
    }
    

    pub fn stop(&mut self) {
    }
    

    pub fn isRunning(&mut self) -> bool {
        return false;
    }
    

    pub fn isActive(&mut self) -> bool {
        return false;
    }
    

    pub fn getFormat(&mut self) -> Option<Rc<RefCell<AudioFormat>>> {
        return NullAudioFormat_NULL_AUDIO_FORMAT.with(Clone::clone).clone();
    }
    

    pub fn getBufferSize(&mut self) -> i32 {
        return 0;
    }
    

    pub fn available(&mut self) -> i32 {
        return 0;
    }
    

    pub fn getFramePosition(&mut self) -> i32 {
        return 0;
    }
    

    pub fn getLongFramePosition(&mut self) -> i64 {
        return 0_i64;
    }
    

    pub fn getMicrosecondPosition(&mut self) -> i64 {
        return 0_i64;
    }
    

    pub fn getLevel(&mut self) -> f32 {
        return 0.0_f32;
    }
    

    pub fn getLineInfo(&mut self) -> Option<Rc<RefCell<Info>>> {
        return Info::new(std::any::type_name_of_val(&self), NullAudioFormat_NULL_AUDIO_FORMAT.with(Clone::clone).clone());
    }
    

    // throws LineUnavailableException
    // Java overloaded method: Rust needs a different name
    pub fn open(&mut self) {
    }
    

    pub fn close(&mut self) {
    }
    

    pub fn isOpen(&mut self) -> bool {
        return false;
    }
    

    pub fn getControls(&mut self) -> Vec<Option<Rc<RefCell<Control>>>> {
        return vec![None; (0) as usize];
    }
    

    pub fn isControlSupported(&mut self, mut control: Option<Rc<RefCell<Type>>>) -> bool {
        return false;
    }
    

    pub fn getControl(&mut self, mut control: Option<Rc<RefCell<Type>>>) -> Option<Rc<RefCell<Control>>> {
        return NullClip_NULL_CONTROL.with(Clone::clone).clone();
    }
    

    pub fn addLineListener(&mut self, mut listener: Option<Rc<RefCell<LineListener>>>) {
    }
    

    pub fn removeLineListener(&mut self, mut listener: Option<Rc<RefCell<LineListener>>>) {
    }
}


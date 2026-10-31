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
// package org.allbinary.graphics.j2me.workarea
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::w3c::dom::Document::Document;
use super::KeyListener::KeyListener;
use super::event::event;
use super::awt::awt;
use super::java::java;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait WorkAreaJPanelInterface: KeyListener {
    

    fn getName(&mut self) -> String;
    

    fn deselect(&mut self);
    

    fn select(&mut self);
    

    fn changeZoom(&mut self, factor: i32);
    

    fn explodeAll(&mut self);
    

    // throws Exception
    fn autoRotate(&mut self, increments: i32, totalAngle: i32);
    

    // throws Exception
    fn autoExplode(&mut self, numberOfFrames: i32, explodeType: i32);
    

    // throws Exception
    fn autoMirror(&mut self);
    

    fn play(&mut self);
    

    fn stop(&mut self);
    

    fn isPlaying(&mut self) -> bool;
    

    // throws Exception
    fn toDocument(&mut self) -> Option<Rc<RefCell<Document>>>;
}



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
// package org.allbinary.input.automation.log
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::File::File;
// use crate::java::io::FileOutputStream::FileOutputStream;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::input::automation::ImageOutputData::ImageOutputData;
use crate::org::allbinary::logic::java::number::LongUtil::LongUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FrameLog {
    pub frame: Option<i64>,
    pub infoStringBuffer: Option<Rc<RefCell<StringMaker>>>,
    pub actionsStringBuffer: Option<Rc<RefCell<StringMaker>>>,
}

pub static mut FrameLog_frameBytes: std::cell::LazyCell<Vec<i8>> = std::cell::LazyCell::new(|| jcall!(String::from("Frame: ").clone()).getBytes());
pub static mut FrameLog_info: std::cell::LazyCell<Vec<i8>> = std::cell::LazyCell::new(|| jcall!(String::from("Info: \n").clone()).getBytes());
pub static mut FrameLog_actions: std::cell::LazyCell<Vec<i8>> = std::cell::LazyCell::new(|| jcall!(String::from("Actions: \n").clone()).getBytes());

impl FrameLog {
    

    pub fn new_struct(mut frame: Option<i64>) -> Self {
        let mut this = Self {
            frame: None,
            infoStringBuffer: None,
            actionsStringBuffer: None,
        };
        this.infoStringBuffer = StringMaker::new();
        this.actionsStringBuffer = StringMaker::new();
        this.frame = frame.clone();
        this
    }

    pub fn new(mut frame: Option<i64>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(frame))))
    }
    

    pub fn addInfo(&mut self, mut infoString: String) {
        jcall!(self.infoStringBuffer).append(infoString.clone());
    }
    

    pub fn addActions(&mut self, mut vectorOfStrings: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut size: i32 = jcall!(vectorOfStrings).size();
        for index in (0..size) {
            { let java_arguments = ((jcall!(vectorOfStrings).get(index.clone())).clone() /* as String */,); self.addAction(java_arguments.0) };
        }
    }
    

    pub fn addAction(&mut self, mut action: String) {
        jcall!(self.actionsStringBuffer).append(action.clone());
        { let java_receiver = self.actionsStringBuffer.clone(); let java_arguments = (jcall!(CommonSeps::getInstance()).NEW_LINE.clone(),); jcall!(java_receiver).append(java_arguments.0); };
    }
    

    // throws Exception
    pub fn write(&mut self) {
        let mut filePathStringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(filePathStringBuffer).append(crate::org::allbinary::input::automation::ImageOutputData::ImageOutputData_SAVE_PATH.with(Clone::clone).clone());
        jcall!(filePathStringBuffer).append(LongUtil::fillIn((self.frame).expect("NullPointerException").to_string()));
        jcall!(filePathStringBuffer).append(String::from(".txt"));
        let mut filePath: String = jcall!(filePathStringBuffer).toString();
        let mut fileOutputStream: Option<Rc<RefCell<FileOutputStream>>> = FileOutputStream::new(File::new(filePath.clone()));
        jcall!(fileOutputStream).write(unsafe { FrameLog_frameBytes.clone() }.clone());
        jcall!(fileOutputStream).write(jcall!(self.getFrame()).toString().bytes().map(|value| value as i8).collect::<Vec<i8>>());
        jcall!(fileOutputStream).write('\n');
        jcall!(fileOutputStream).write(unsafe { FrameLog_info.clone() }.clone());
        jcall!(fileOutputStream).write(jcall!(self.infoStringBuffer).toString().bytes().map(|value| value as i8).collect::<Vec<i8>>());
        jcall!(fileOutputStream).write('\n');
        jcall!(fileOutputStream).write(unsafe { FrameLog_actions.clone() }.clone());
        jcall!(fileOutputStream).write(jcall!(self.actionsStringBuffer).toString().bytes().map(|value| value as i8).collect::<Vec<i8>>());
        jcall!(fileOutputStream).close();
    }
    

    pub fn getFrame(&mut self) -> Option<i64> {
        return self.frame.clone();
    }
}


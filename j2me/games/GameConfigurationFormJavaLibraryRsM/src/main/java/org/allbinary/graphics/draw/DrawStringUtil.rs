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
// package org.allbinary.graphics.draw
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::AndroidUtil::AndroidUtil;
use crate::org::allbinary::J2MEUtil::J2MEUtil;
use crate::org::allbinary::graphics::Anchor::Anchor;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureUtil::OpenGLFeatureUtil;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DrawStringUtil {
    pub EMPTY_STRING: String,
    pub anchor: i32,
}

thread_local! { pub static DrawStringUtil_instance: Option<Rc<RefCell<DrawStringUtil>>> = DrawStringUtil::new(); }

impl DrawStringUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            EMPTY_STRING: String::new(),
            anchor: 0,
        };
        this.EMPTY_STRING = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<DrawStringUtil>>> {
        return DrawStringUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn drawCenterString(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut string: String, mut offset: i32, mut length: i32, mut x: i32, mut y: i32) {
        let mut width: i32 = (jcall!(jcall!(graphics).getFont()).substringWidth(string.clone(), offset.clone(), length.clone()) >> 1);
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(graphics).drawSubstring(string.clone(), offset.clone(), length.clone(), x - width, y.clone(), self.anchor.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            PreLogUtil::put(jcall!(StringMaker::new()).append(String::from("Exception: stringLength: ")).appendint((string.chars().count() as i32)).append(String::from(" offset: ")).appendint(offset.clone()).append(String::from(" currentLength: ")).appendint(length.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("drawCenterString"));
            }
        }
    }
    

    pub fn drawCenterStrings(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut stringArray: Vec<String>, mut maxWidth: i32, mut charHeight: i32, mut x: i32, mut y: i32) {
        let mut extraLines: i32 = 0;
        for index in (0..(stringArray.len() as i32)) {
            let mut string: String = stringArray[(index) as usize].clone();
            if string != self.EMPTY_STRING {
                let mut width: i32 = jcall!(jcall!(graphics).getFont()).stringWidth(string.clone());
                let mut minTotalLines: i32 = 1;
                if width > maxWidth {
                    minTotalLines = (width / maxWidth) + 1;
                }
                let mut linePortion: i32 = (string.chars().count() as i32) / minTotalLines;
                let mut offset: i32 = 0;
                let mut currentLength: i32 = linePortion.clone();
                let mut size: i32 = (string.chars().count() as i32);
                while offset < size {
                    if offset + currentLength > size {
                        currentLength = size - offset;
                    }
                    if offset + currentLength != size {
                        while currentLength > 0 && stringArray[(index) as usize].chars().nth((offset + currentLength) as usize).unwrap() != ' ' {
                            currentLength -= 1;
                        }
                        if currentLength <= 4 {
                            currentLength = linePortion.clone();
                        }
                    }
                    { let java_arguments = (graphics.clone(), string.clone(), offset.clone(), currentLength.clone(), x.clone(), y + ((index + { let value = extraLines; extraLines += 1; value }) * charHeight),); self.drawCenterString(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5) };
                    offset = offset + currentLength;
                    currentLength = linePortion.clone();
                }
                extraLines -= 1;
            }
        }
    }
}


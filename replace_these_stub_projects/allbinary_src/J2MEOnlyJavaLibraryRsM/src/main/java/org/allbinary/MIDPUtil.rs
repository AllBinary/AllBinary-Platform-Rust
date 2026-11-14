//
// AllBinary Open License Version 1
// Copyright (c) 2026 AllBinary
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
// package org.allbinary
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::io::IOException::IOException;
use crate::java::io::InputStream::InputStream;
use crate::javax::microedition::lcdui::Form::Form;
use crate::javax::microedition::lcdui::Image::Image;
use crate::javax::microedition::lcdui::Item::Item;
use crate::javax::microedition::media::Manager::Manager;
use crate::javax::microedition::media::MediaException::MediaException;
use crate::javax::microedition::media::Player::Player;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//like microemu-playn, so the languages without overloading can be generated. J2ME uses the MIDP overloads.
pub struct MIDPUtil {
}

thread_local! { pub static MIDPUtil_instance: Option<Rc<RefCell<MIDPUtil>>> = MIDPUtil::new(); }

impl MIDPUtil {

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
    

    pub fn getInstance() -> Option<Rc<RefCell<MIDPUtil>>> {
        return MIDPUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn createImageWH(&mut self, mut width: i32, mut height: i32) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        return Image::createImageWH(width.clone(), height.clone());
    }
    

    pub fn createImageFromByteArray(&mut self, mut imageData: Vec<i8>, mut imageOffset: i32, mut imageLength: i32) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        return Image::createImageFromByteArray(imageData.clone(), imageOffset.clone(), imageLength.clone());
    }
    

    // throws IOException
    pub fn createImageFromInputStream(&mut self, mut inputStream: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>>) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        return Image::createImageFromInputStream(inputStream.clone());
    }
    

    // throws IOException, MediaException
    pub fn createPlayerFromInputStream(&mut self, mut inputStream: Option<Rc<RefCell<dyn crate::java::io::InputStream::InputStreamDyn>>>, mut r#type: String) -> Option<Rc<RefCell<dyn Player>>> {
        return Manager::createPlayerFromInputStream(inputStream.clone(), r#type.clone());
    }
    

    pub fn appendItem(&mut self, mut form: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Form::FormDyn>>>, mut item: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Item::ItemDyn>>>) -> i32 {
        return jcall!(form).as_Form_mut().appendItem(item.clone());
    }
}


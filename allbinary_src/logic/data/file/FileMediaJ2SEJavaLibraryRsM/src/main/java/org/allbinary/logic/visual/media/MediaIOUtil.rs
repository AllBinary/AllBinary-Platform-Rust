// Generated Code Do Not Modify
// package org.allbinary.logic.visual.media
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::imageio::ImageIO::ImageIO;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MediaIOUtil {
}

thread_local! { pub static MediaIOUtil_instance: Option<Rc<RefCell<MediaIOUtil>>> = MediaIOUtil::new(); }

impl MediaIOUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<MediaIOUtil>>> {
        return MediaIOUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn getReaderFormatNames(&mut self) -> Vec<String> {
        return ImageIO::getReaderFormatNames();
    }
    

    pub fn getWriterFormatNames(&mut self) -> Vec<String> {
        return ImageIO::getWriterFormatNames();
    }
}


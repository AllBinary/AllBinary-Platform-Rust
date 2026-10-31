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
// package org.allbinary.logic.visual.media
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::MediaData::MediaData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MediaDataFactory {
    pub AVI: Option<Rc<RefCell<MediaData>>>,
    pub BMP: Option<Rc<RefCell<MediaData>>>,
    pub CEL: Option<Rc<RefCell<MediaData>>>,
    pub FITS: Option<Rc<RefCell<MediaData>>>,
    pub FLI: Option<Rc<RefCell<MediaData>>>,
    pub GBR: Option<Rc<RefCell<MediaData>>>,
    pub GIF: Option<Rc<RefCell<MediaData>>>,
    pub GIH: Option<Rc<RefCell<MediaData>>>,
    pub JPG: Option<Rc<RefCell<MediaData>>>,
    pub PIX: Option<Rc<RefCell<MediaData>>>,
    pub PNG: Option<Rc<RefCell<MediaData>>>,
    pub PNM: Option<Rc<RefCell<MediaData>>>,
    pub PSD: Option<Rc<RefCell<MediaData>>>,
    pub PSP: Option<Rc<RefCell<MediaData>>>,
    pub PS: Option<Rc<RefCell<MediaData>>>,
    pub SGI: Option<Rc<RefCell<MediaData>>>,
    pub SWF: Option<Rc<RefCell<MediaData>>>,
    pub TGA: Option<Rc<RefCell<MediaData>>>,
    pub TIFF: Option<Rc<RefCell<MediaData>>>,
    pub WMF: Option<Rc<RefCell<MediaData>>>,
}

thread_local! { pub static MediaDataFactory_instance: Option<Rc<RefCell<MediaDataFactory>>> = MediaDataFactory::new(); }

impl MediaDataFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            AVI: None,
            BMP: None,
            CEL: None,
            FITS: None,
            FLI: None,
            GBR: None,
            GIF: None,
            GIH: None,
            JPG: None,
            PIX: None,
            PNG: None,
            PNM: None,
            PSD: None,
            PSP: None,
            PS: None,
            SGI: None,
            SWF: None,
            TGA: None,
            TIFF: None,
            WMF: None,
        };
        this.AVI = MediaData::new(String::from("avi"));
        this.BMP = MediaData::new(String::from("bmp"));
        this.CEL = MediaData::new(String::from("cel"));
        this.FITS = MediaData::new(String::from("fits"));
        this.FLI = MediaData::new(String::from("fli"));
        this.GBR = MediaData::new(String::from("gbr"));
        this.GIF = MediaData::new(String::from("gif"));
        this.GIH = MediaData::new(String::from("gih"));
        this.JPG = MediaData::new(String::from("jpg"));
        this.PIX = MediaData::new(String::from("pix"));
        this.PNG = MediaData::new(String::from("png"));
        this.PNM = MediaData::new(String::from("pnm"));
        this.PSD = MediaData::new(String::from("psd"));
        this.PSP = MediaData::new(String::from("psp"));
        this.PS = MediaData::new(String::from("ps"));
        this.SGI = MediaData::new(String::from("SGI"));
        this.SWF = MediaData::new(String::from("swf"));
        this.TGA = MediaData::new(String::from("tga"));
        this.TIFF = MediaData::new(String::from("tiff"));
        this.WMF = MediaData::new(String::from("wmf"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<MediaDataFactory>>> {
        return MediaDataFactory_instance.with(Clone::clone).clone();
    }
}


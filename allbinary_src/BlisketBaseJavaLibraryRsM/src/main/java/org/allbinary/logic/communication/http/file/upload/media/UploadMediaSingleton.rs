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
// package org.allbinary.logic.communication.http.file.upload.media
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Set::Set;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::visual::media::MediaData::MediaData;
use crate::org::allbinary::logic::visual::media::MediaIOUtil::MediaIOUtil;
use crate::org::allbinary::logic::visual::media::MediaTypeData::MediaTypeData;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UploadMediaSingleton {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub readerFileTypesHashMap: Option<Rc<RefCell<HashMap>>>,
    pub writerFileTypesHashMap: Option<Rc<RefCell<HashMap>>>,
}

pub static mut UploadMediaSingleton_uploadMedia: std::cell::LazyCell<Option<Rc<RefCell<UploadMediaSingleton>>>> = std::cell::LazyCell::new(|| None);

impl UploadMediaSingleton {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            readerFileTypesHashMap: None,
            writerFileTypesHashMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.readerFileTypesHashMap = jcall!(StdUtil::getInstance()).createHashMap();
        this.writerFileTypesHashMap = jcall!(StdUtil::getInstance()).createHashMap();
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = MediaData::toHashMap();
        let mut set: Option<Rc<RefCell<dyn Set>>> = jcall!(hashMap).keySet();
        let mut mediaDataNameArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(set).toArray();
        let mut size: i32 = (mediaDataNameArray.len() as i32).clone();
        for index in (0..size) {
            let mut mediaDataName: String = (mediaDataNameArray[(index) as usize]).clone() /* as String */;
            let mut mediaData: Option<Rc<RefCell<MediaData>>> = (jcall!(hashMap).get(mediaDataName.clone())).clone() /* as MediaData */;
            { let java_receiver = this.readerFileTypesHashMap.clone(); let java_arguments = (jcall!(mediaData).getName(), jcall!(mediaData).getType(),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1); };
        }
        let mut mediaIOUtil: Option<Rc<RefCell<MediaIOUtil>>> = MediaIOUtil::getInstance();
        let mut RESIZABLE_MEDIA: String = jcall!(MediaTypeData::getInstance()).RESIZABLE_MEDIA.clone();
        let mut readerFileTypes: Vec<String> = jcall!(mediaIOUtil).getReaderFormatNames();
        for index in (0..(readerFileTypes.len() as i32)) {
            jcall!(this.readerFileTypesHashMap).put(readerFileTypes[(index) as usize].clone(), RESIZABLE_MEDIA.clone());
        }
        let mut writerFileTypes: Vec<String> = jcall!(mediaIOUtil).getWriterFormatNames();
        for index in (0..(writerFileTypes.len() as i32)) {
            jcall!(this.writerFileTypesHashMap).put(writerFileTypes[(index) as usize].clone(), RESIZABLE_MEDIA.clone());
        }
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).HTTPREQUEST.clone()) {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Supported Media Readers: "));
            jcall!(stringBuffer).append(jcall!(this.readerFileTypesHashMap).toString());
            jcall!(stringBuffer).append(String::from(" Writers: "));
            jcall!(stringBuffer).append(jcall!(this.writerFileTypesHashMap).toString());
            { let java_receiver = this.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // synchronized
    pub fn getInstance() -> Option<Rc<RefCell<UploadMediaSingleton>>> {
        if unsafe { UploadMediaSingleton_uploadMedia.clone() }.is_none() {
            unsafe { *UploadMediaSingleton_uploadMedia = UploadMediaSingleton::new() };
        }
        return unsafe { UploadMediaSingleton_uploadMedia.clone() }.clone();
    }
    

    pub fn isReaderSupported(&mut self, mut mediaFileType: String) -> bool {
        return jcall!(self.readerFileTypesHashMap).containsKey(mediaFileType.clone());
    }
    

    pub fn isWriterSupported(&mut self, mut mediaFileType: String) -> bool {
        return jcall!(self.readerFileTypesHashMap).containsKey(mediaFileType.clone());
    }
    

    pub fn isSupported(&mut self, mut mediaFileType: String) -> bool {
        return self.isReaderSupported(mediaFileType.clone()) && self.isWriterSupported(mediaFileType.clone());
    }
    

    pub fn isMedia(&mut self, mut mediaFileType: String, mut aMediaType: String) -> bool {
        return self.isReaderMedia(mediaFileType.clone(), aMediaType.clone()) && self.isWriterMedia(mediaFileType.clone(), aMediaType.clone());
    }
    

    pub fn isReaderMedia(&mut self, mut mediaFileType: String, mut aMediaType: String) -> bool {
        return self.isMedia3(mediaFileType.clone(), aMediaType.clone(), self.readerFileTypesHashMap.clone());
    }
    

    pub fn isWriterMedia(&mut self, mut mediaFileType: String, mut aMediaType: String) -> bool {
        return self.isMedia3(mediaFileType.clone(), aMediaType.clone(), self.writerFileTypesHashMap.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn isMedia3(&mut self, mut mediaFileType: String, mut aMediaType: String, mut hashMap: Option<Rc<RefCell<HashMap>>>) -> bool {
        if jcall!(hashMap).containsKey(mediaFileType.clone()) {
            let mut mediaType: String = (jcall!(hashMap).get(mediaFileType.clone())).clone() /* as String */;
            if (mediaType.cmp(&aMediaType.clone()) as i32) == 0 {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
}

